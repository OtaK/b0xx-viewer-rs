pub mod b0xx_state;
mod error;

use self::b0xx_state::*;

pub use self::error::*;
use std::io::Read;

fn main() -> Result<(), ViewerError> {
    let b0xx_port = serialport::available_ports()?
        .into_iter()
        .find(|port| {
            if let serialport::SerialPortType::UsbPort(portinfo) = &port.port_type {
                if let Some(product) = &portinfo.product {
                    return product == "Arduino_Leonardo";
                }
            }

            false
        })
        .ok_or_else(|| ViewerError::B0xxNotFound)?;

    let port_settings = serialport::SerialPortSettings {
        baud_rate: 115200,
        timeout: std::time::Duration::from_secs(1),
        ..Default::default()
    };

    let mut buf = Vec::with_capacity(24);
    let port = serialport::open_with_settings(&b0xx_port.port_name, &port_settings)?;
    port.bytes()
        .try_for_each(|b: Result<u8, std::io::Error>| -> Result<(), ViewerError> {
            let report: B0xxReport = b?.into();
            match report {
                B0xxReport::End => {
                    for (i, value) in buf.iter().enumerate() {
                        println!("Buf pos {}: {:?}", i, value);
                    }
                    let state: B0xxState = buf.as_slice().into();

                    println!("{:#?}", state);
                    buf.clear();
                }
                _ => {
                    if buf.len() < buf.capacity() {
                        buf.push(report);
                    }
                }
            }

            Ok(())
        })?;

    Ok(())
}
