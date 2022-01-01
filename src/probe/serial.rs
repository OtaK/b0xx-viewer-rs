use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use super::{ControllerMessage, ControllerProbe};
use crate::controllers::{b0xx::*, ControllerType};
use crate::error::ViewerError;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serialport::SerialPortInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WhitelistFile {
    arduino: Vec<UsbStringDef>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct UsbStringDef {
    pub vid: String,
    pub pid: String,
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct UsbDefinition {
    pub vid: u16,
    pub pid: u16,
}

impl std::convert::TryFrom<UsbStringDef> for UsbDefinition {
    type Error = std::num::ParseIntError;

    fn try_from(def: UsbStringDef) -> Result<Self, Self::Error> {
        Ok(Self {
            pid: u16::from_str_radix(def.pid.trim_start_matches("0x"), 16)?,
            vid: u16::from_str_radix(def.vid.trim_start_matches("0x"), 16)?,
        })
    }
}

const ARDUINO_WHITELIST_BYTES: &[u8] = include_bytes!("../../assets/arduino_whitelist.toml");
lazy_static! {
    static ref ARDUINO_WHITELIST: Vec<UsbDefinition> = {
        let res: WhitelistFile = toml::from_slice(ARDUINO_WHITELIST_BYTES).unwrap();
        use std::convert::TryFrom as _;
        res.arduino
            .into_iter()
            .map(|s_def| UsbDefinition::try_from(s_def).unwrap())
            .collect()
    };
}
#[derive(Debug)]
pub struct SerialControllerProbe {
    serial_port_info: SerialPortInfo,
    controller_type: ControllerType,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    is_connected: Arc<AtomicBool>,
}

impl ControllerProbe for SerialControllerProbe {
    fn new(config: &crate::config::ViewerOptions) -> crate::ViewerResult<Self>
    where
        Self: Sized,
    {
        let (serial_port_info, controller_type) = serialport::available_ports()?
            .into_iter()
            .find_map(move |port| {
                if let Some(custom_tty) = config.custom_tty.as_ref() {
                    if port.port_name == *custom_tty {
                        return Some((port, ControllerType::DIYB0XX));
                    }
                } else if let serialport::SerialPortType::UsbPort(portinfo) = &port.port_type {
                    if std::env::var("RELAX_ARDUINO_DETECT").is_ok() {
                        if ARDUINO_WHITELIST
                            .iter()
                            .any(|def| def.vid == portinfo.vid && def.pid == portinfo.pid)
                        {
                            return Some((port, ControllerType::DIYB0XX));
                        }
                    } else if portinfo.vid == 9025 && portinfo.pid == 32822 {
                        return Some((port, ControllerType::B0XX));
                    }

                    if let Some(product) = &portinfo.product {
                        if product == "Arduino_Leonardo" {
                            return Some((port, ControllerType::B0XX));
                        } else if product == "Frame1" {
                            return Some((port, ControllerType::Frame1));
                        }
                    }
                }

                None
            })
            .ok_or(ViewerError::ControllerNotFound)?;

        Ok(Self {
            serial_port_info,
            controller_type,
            thread_handle: None,
            is_connected: Arc::new(true.into()),
        })
    }

    fn controller_type(&self) -> crate::controllers::ControllerType {
        self.controller_type
    }

    fn is_connected(&self) -> bool {
        self.thread_handle.is_some() && self.is_connected.load(Ordering::SeqCst)
    }

    fn connect(&mut self) -> crate::ViewerResult<crossbeam_channel::Receiver<ControllerMessage>> {
        let controller_port = self.serial_port_info.clone();

        let (tx, rx) = crossbeam_channel::bounded(1);
        let thread_hwnd = std::thread::spawn(move || {
            let mut buf = Vec::with_capacity(25);
            let mut state = [B0xxReport::default(); 20];

            let port_builder = serialport::new(&controller_port.port_name, 115_200)
                .data_bits(serialport::DataBits::Eight)
                .flow_control(serialport::FlowControl::Hardware)
                .parity(serialport::Parity::None)
                .stop_bits(serialport::StopBits::One)
                .timeout(std::time::Duration::from_millis(500));

            let mut port = match port_builder.open() {
                Ok(port) => port,
                Err(e) => {
                    let _ = tx.send(ControllerMessage::Error(e.into()));
                    return;
                }
            };

            exhaust_buffer(&mut port, &tx);

            let mut port = std::io::BufReader::with_capacity(25, port);

            use std::io::BufRead as _;
            loop {
                if let Err(e) = port.get_mut().write_request_to_send(true) {
                    let _ = tx.send(ControllerMessage::Error(e.into()));
                    return;
                }

                let bytes_read: usize = match port
                    .read_until(B0xxReport::End as u8, &mut buf)
                    .map_err(Into::into)
                {
                    Ok(bytes) => bytes,
                    Err(e) => match &e {
                        ViewerError::IoError(io_error) => match io_error.kind() {
                            std::io::ErrorKind::TimedOut | std::io::ErrorKind::BrokenPipe => {
                                let _ = tx.send(ControllerMessage::Reconnect);
                                return;
                            }
                            _ => {
                                error!("{:?}", e);
                                let _ = tx.send(ControllerMessage::Quit);
                                return;
                            }
                        },
                        _ => {
                            error!("{:?}", e);
                            let _ = tx.send(ControllerMessage::Quit);
                            return;
                        }
                    },
                };

                if let Err(e) = port.get_mut().write_request_to_send(false) {
                    let _ = tx.send(ControllerMessage::Error(e.into()));
                    return;
                }

                trace!("Bytes read: {}", bytes_read);

                port.consume(bytes_read);
                if bytes_read == 25 {
                    let end_index = buf
                        .iter()
                        .position(|item| *item == B0xxReport::End as u8)
                        .unwrap()
                        - 4;
                    let start_index = end_index - 20;
                    trace!("Selected range: {}..{}", start_index, end_index);

                    for i in start_index..end_index {
                        state[i] = buf[i].into();
                    }
                } else {
                    exhaust_buffer(port.get_mut(), &tx);
                }

                buf.clear();

                if tx.send(ControllerMessage::State(state.into())).is_err() {
                    info!("Reconnection detected, exiting runloop");
                    return;
                }
            }
        });
        self.thread_handle = Some(thread_hwnd);
        Ok(rx)
    }

    fn disconnect(&mut self) {
        if !self.is_connected() {
            return;
        }

        if let Some(thread_hwnd) = self.thread_handle.take() {
            let _ = thread_hwnd.join();
        }
    }
}

#[inline(always)]
fn exhaust_buffer(
    port: &mut Box<dyn serialport::SerialPort>,
    tx: &crossbeam_channel::Sender<ControllerMessage>,
) {
    // Exhaust the initial buffer till we find the end of a report and consume it.
    // This is caused by a UB in Windows' COM port handling causing partial reports
    // sometimes
    trace!("Buffer exhaustion started");
    let mut exhaust_buffer = [0u8; 1];
    use std::io::Read as _;
    loop {
        if let Err(e) = port
            .read_exact(&mut exhaust_buffer)
            .map_err(ViewerError::from)
        {
            error!("{:?}", e);
            let _ = tx.send(ControllerMessage::Quit);
            break;
        }

        if exhaust_buffer[0] == B0xxReport::End as u8 {
            trace!("Buffer exhausted successfully, continuing...");
            break;
        }
    }

    if let Err(e) = port.clear(serialport::ClearBuffer::All) {
        let _ = tx.send(ControllerMessage::Error(e.into()));
    }
}
