use crate::b0xx_state::*;
use crate::error::{ViewerResult, ViewerError};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WhitelistFile {
    def: Vec<UsbStringDef>,
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

const ARDUINO_WHITELIST_BYTES: &str = include_str!("../assets/arduino_whitelist.toml");
const B0XX_WHITELIST_BYTES: &str = include_str!("../assets/b0xx_whitelist.toml");
lazy_static! {
    static ref ARDUINO_WHITELIST: Vec<UsbDefinition> = {
        let res: WhitelistFile = toml::from_str(ARDUINO_WHITELIST_BYTES).unwrap();
        use std::convert::TryFrom as _;
        res.def
            .into_iter()
            .map(|s_def| UsbDefinition::try_from(s_def).unwrap())
            .collect()
    };

    static ref B0XX_WHITELIST: Vec<UsbDefinition> = {
        let res: WhitelistFile = toml::from_str(B0XX_WHITELIST_BYTES).unwrap();
        use std::convert::TryFrom as _;
        res.def
            .into_iter()
            .map(|s_def| UsbDefinition::try_from(s_def).unwrap())
            .collect()
    };
}

#[cfg_attr(feature = "fake_inputs", allow(dead_code))]
#[derive(Debug)]
pub enum B0xxMessage {
    State(B0xxState),
    Error(ViewerError),
    Reconnect,
    Quit,
}

#[inline]
pub fn reconnect(custom_tty: &Option<String>) -> crossbeam_channel::Receiver<B0xxMessage> {
    use backoff::backoff::Backoff as _;
    let mut backoff = backoff::ExponentialBackoff::default();
    loop {
        if let Ok(new_rx) = start_serial_probe(custom_tty) {
            return new_rx;
        }

        if let Some(backoff_duration) = backoff.next_backoff() {
            std::thread::sleep(backoff_duration);
        }
    }
}

#[cfg(not(feature = "fake_inputs"))]
pub fn start_serial_probe(
    custom_tty: &Option<String>,
) -> ViewerResult<crossbeam_channel::Receiver<B0xxMessage>> {

    let b0xx_port = serialport::available_ports()?
        .into_iter()
        .find(move |port| {
            if let Some(custom_tty) = custom_tty {
                if port.port_name == *custom_tty {
                    return true;
                }
            } else if let serialport::SerialPortType::UsbPort(portinfo) = &port.port_type {
                if std::env::var("RELAX_ARDUINO_DETECT").is_ok() {
                    if ARDUINO_WHITELIST
                        .iter()
                        .any(|def| def.vid == portinfo.vid && def.pid == portinfo.pid)
                    {
                        return true;
                    }
                } else if B0XX_WHITELIST.iter().any(|def| def.vid == portinfo.vid && def.pid == portinfo.pid) {
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

    log::info!("Found B0XX on port {}", b0xx_port.port_name);

    let (tx, rx) = crossbeam_channel::bounded(1);

    std::thread::Builder::new()
        .name("b0xx_viewer_serial".into())
        .spawn(move || {
            let mut buf = Vec::with_capacity(25);
            let mut state = [B0xxReport::default(); 20];

            let port_builder = serialport::new(&b0xx_port.port_name, 115_200)
                .data_bits(serialport::DataBits::Eight)
                .flow_control(serialport::FlowControl::Hardware)
                .parity(serialport::Parity::None)
                .stop_bits(serialport::StopBits::One)
                .timeout(std::time::Duration::from_millis(500));

            let mut port =
                match port_builder.open() {
                    Ok(port) => port,
                    Err(e) => return tx.send(B0xxMessage::Error(e.into())),
                };


            exhaust_buffer(&mut port, &tx);

            let mut port = std::io::BufReader::with_capacity(25, port);

            use std::io::BufRead as _;
            loop {
                if let Err(e) = port.get_mut().write_request_to_send(true) {
                    return tx.send(B0xxMessage::Error(e.into()));
                }

                let bytes_read: usize = match port.read_until(B0xxReport::End as u8, &mut buf).map_err(Into::into) {
                    Ok(bytes) => bytes,
                    Err(e) => match &e {
                        ViewerError::IoError(io_error) => match io_error.kind() {
                            std::io::ErrorKind::TimedOut | std::io::ErrorKind::BrokenPipe => {
                                return tx.send(B0xxMessage::Reconnect);
                            }
                            _ => {
                                log::error!("{e:?}");
                                return tx.send(B0xxMessage::Quit);
                            }
                        },
                        _ => {
                            log::error!("{e:?}");
                            return tx.send(B0xxMessage::Quit);
                        }
                    }
                };

                if let Err(e) = port.get_mut().write_request_to_send(false) {
                    return tx.send(B0xxMessage::Error(e.into()));
                }

                log::trace!("Bytes read: {bytes_read}");

                port.consume(bytes_read);
                if bytes_read == 25 {
                    let end_index = buf.iter().position(|item| *item == B0xxReport::End as u8).unwrap() - 4;
                    let start_index = end_index - 20;
                    log::trace!("Selected range: {start_index}..{end_index}");

                    for i in start_index..end_index {
                        state[i] = buf[i].into();
                    }
                } else {
                    exhaust_buffer(port.get_mut(), &tx);
                }

                buf.clear();

                if tx.send(B0xxMessage::State(state.into())).is_err() {
                    log::info!("Reconnection detected, exiting runloop");
                    return Ok(());
                }
            }
        })?;

    Ok(rx)
}

#[cfg(feature = "fake_inputs")]
pub fn start_serial_probe(
    _: &Option<String>,
) -> ViewerResult<crossbeam_channel::Receiver<B0xxMessage>> {
    let (tx, rx) = crossbeam_channel::bounded(1);
    if std::env::var("RELAX_ARDUINO_DETECT").is_ok() {
        log::info!("{:#?}", *ARDUINO_WHITELIST)
    }
    use rand::SeedableRng as _;
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let sleep_dur = std::time::Duration::from_micros(8700);
    std::thread::spawn(move || loop {
        let _ = tx.send(B0xxMessage::State(B0xxState::random(&mut rng)));
        #[cfg(not(feature = "benchmark"))]
        std::thread::sleep(sleep_dur);
    });

    Ok(rx)
}

#[allow(dead_code)]
#[inline(always)]
fn exhaust_buffer(port: &mut Box<dyn serialport::SerialPort>, tx: &crossbeam_channel::Sender<B0xxMessage>) {
    // Exhaust the initial buffer till we find the end of a report and consume it.
    // This is caused by a UB in Windows' COM port handling causing partial reports
    // sometimes
    log::trace!("Buffer exhaustion started");
    let mut exhaust_buffer = [0u8; 1];
    use std::io::Read as _;
    loop {
        if let Err(e) = port
            .read_exact(&mut exhaust_buffer)
            .map_err(ViewerError::from)
        {
            log::error!("{:?}", e);
            let _ = tx.send(B0xxMessage::Quit);
            break;
        }

        if exhaust_buffer[0] == B0xxReport::End as u8 {
            log::trace!("Buffer exhausted successfully, continuing...");
            break;
        }
    }

    if let Err(e) = port.clear(serialport::ClearBuffer::All) {
        let _ = tx.send(B0xxMessage::Error(e.into()));
    }
}
