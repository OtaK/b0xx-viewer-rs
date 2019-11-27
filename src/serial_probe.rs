use crate::b0xx_state::*;
use crate::error::ViewerError;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WhitelistFile {
    arduino: Vec<UsbStringDef>
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

const ARDUINO_WHITELIST_BYTES: &[u8] = include_bytes!("../assets/arduino_whitelist.toml");
lazy_static! {
    static ref ARDUINO_WHITELIST: Vec<UsbDefinition> = {
        let res: WhitelistFile = toml::from_slice(ARDUINO_WHITELIST_BYTES).unwrap();
        use std::convert::TryFrom as _;
        res.arduino.into_iter().map(|s_def| UsbDefinition::try_from(s_def).unwrap()).collect()
    };
}

#[cfg_attr(feature = "fake_serial", allow(dead_code))]
#[derive(Debug)]
pub enum B0xxMessage {
    State(B0xxState),
    Error(ViewerError),
    Reconnect,
    Quit,
}

#[inline]
pub fn reconnect(custom_tty: &Option<String>) -> crossbeam_channel::Receiver<B0xxMessage> {
    loop {
        if let Ok(new_rx) = start_serial_probe(custom_tty) {
            return new_rx;
        }
    }
}

#[cfg(not(feature = "fake_serial"))]
pub fn start_serial_probe(custom_tty: &Option<String>) -> Result<crossbeam_channel::Receiver<B0xxMessage>, ViewerError> {
    use std::io::Read;

    let b0xx_port = serialport::available_ports()?
        .into_iter()
        .find(move |port| {
            if let Some(custom_tty) = custom_tty {
                if port.port_name == *custom_tty {
                    return true;
                }
            } else if let serialport::SerialPortType::UsbPort(portinfo) = &port.port_type {
                if std::env::var("RELAX_ARDUINO_DETECT").is_ok() {
                    if ARDUINO_WHITELIST.iter().find(|def| def.vid == portinfo.vid && def.pid == portinfo.pid).is_some() {
                        return true;
                    }
                } else if portinfo.vid == 9025 && portinfo.pid == 32822 {
                    return true;
                }

                if let Some(product) = &portinfo.product {
                    if product == "Arduino_Leonardo" {
                        return true;
                    }
                }
            }

            false
        })
        .ok_or_else(|| ViewerError::B0xxNotFound)?;

    info!("Found B0XX on port {}", b0xx_port.port_name);

    let port_settings = serialport::SerialPortSettings {
        baud_rate: 115_200,
        data_bits: serialport::DataBits::Eight,
        flow_control: serialport::FlowControl::Hardware,
        parity: serialport::Parity::None,
        stop_bits: serialport::StopBits::One,
        timeout: std::time::Duration::from_millis(500),
    };

    let (tx, rx) = crossbeam_channel::bounded(1);
    std::thread::Builder::new()
        .name("b0xx_viewer_serial".into())
        .spawn(move || {
            let mut buf = [0u8; 25];
            let mut state = [B0xxReport::default(); 20];

            let mut port =
                match serialport::open_with_settings(&b0xx_port.port_name, &port_settings) {
                    Ok(port) => port,
                    Err(e) => return tx.send(B0xxMessage::Error(e.into())),
                };

            // Exhaust the initial buffer till we find the end of a report and consume it.
            // This is caused by a UB in Windows' COM port handling causing partial reports
            // sometimes
            debug!("Buffer exhaustion started");
            let mut exhaust_buffer = [0u8; 1];
            loop {
                if let Err(e) = port.read_exact(&mut exhaust_buffer).map_err(ViewerError::from) {
                    error!("{:?}", e);
                    return tx.send(B0xxMessage::Quit);
                }

                if exhaust_buffer[0] == B0xxReport::End as u8 {
                    debug!("Buffer exhausted successfully, continuing...");
                    break;
                }
            }

            if let Err(e) = port.clear(serialport::ClearBuffer::All) {
                return tx.send(B0xxMessage::Error(e.into()));
            }

            if let Err(e) = port.write_request_to_send(true) {
                return tx.send(B0xxMessage::Error(e.into()));
            }

            loop {
                if let Err(e) = port.read_exact(&mut buf).map_err(Into::into) {
                    match &e {
                        ViewerError::IoError(io_error) => match io_error.kind() {
                            std::io::ErrorKind::TimedOut | std::io::ErrorKind::BrokenPipe => {
                                return tx.send(B0xxMessage::Reconnect);
                            }
                            _ => {
                                error!("{:?}", ViewerError::from(e));
                                return tx.send(B0xxMessage::Quit);
                            }
                        },
                        _ => {
                            error!("{:?}", ViewerError::from(e));
                            return tx.send(B0xxMessage::Quit);
                        }
                    }
                }

                for (i, a) in buf.iter_mut().take(20).enumerate() {
                    state[i] = (*a).into();
                    *a = 0;
                }

                if let Err(_) = tx.send(B0xxMessage::State(state.into())) {
                    info!("Reconnection detected, exiting runloop");
                    return Ok(());
                }
            }
        })?;

    Ok(rx)
}

#[cfg(feature = "fake_serial")]
pub fn start_serial_probe(_: &Option<String>) -> Result<crossbeam_channel::Receiver<B0xxMessage>, ViewerError> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    if std::env::var("RELAX_ARDUINO_DETECT").is_ok() {
        info!("{:#?}", *ARDUINO_WHITELIST)
    }
    std::thread::spawn(move || loop {
        let _ = tx.send(B0xxMessage::State(B0xxState::random()));
    });

    Ok(rx)
}
