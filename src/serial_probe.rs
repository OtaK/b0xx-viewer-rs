use crate::b0xx_state::*;
use crate::error::ViewerError;

#[cfg(not(feature = "fake_serial"))]
pub fn start_serial_probe(
) -> Result<crossbeam_channel::Receiver<Result<B0xxState, ViewerError>>, ViewerError> {
    use std::io::Read;

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
        baud_rate: 3_686_400,
        data_bits: serialport::DataBits::Eight,
        flow_control: serialport::FlowControl::None,
        parity: serialport::Parity::None,
        stop_bits: serialport::StopBits::One,
        timeout: std::time::Duration::from_millis(500),
    };

    let (tx, rx) = crossbeam_channel::unbounded();
    std::thread::spawn(move || {
        let mut buf = Vec::with_capacity(18);

        let port = match serialport::open_with_settings(&b0xx_port.port_name, &port_settings) {
            Ok(port) => port,
            Err(e) => return tx.send(Err(e.into())),
        };

        let loop_tx = tx.clone();

        if let Err(e) = port.bytes().try_for_each(
            move |b: Result<u8, std::io::Error>| -> Result<(), ViewerError> {
                let report: B0xxReport = b?.into();
                match report {
                    B0xxReport::End => {
                        let state: B0xxState = buf.as_slice().into();
                        let _ = loop_tx.send(Ok(state));
                        buf.clear();
                    }
                    _ => {
                        if buf.len() < buf.capacity() {
                            buf.push(report);
                        }
                    }
                }

                Ok(())
            },
        ) {
            return tx.send(Err(e.into()));
        }

        Ok(())
    });

    Ok(rx)
}

#[cfg(feature = "fake_serial")]
pub fn start_serial_probe(
) -> Result<crossbeam_channel::Receiver<Result<B0xxState, ViewerError>>, ViewerError> {
    let (tx, rx) = crossbeam_channel::unbounded();
    let wait = std::time::Duration::from_micros(16667);
    std::thread::spawn(move || loop {
        let _ = tx.send(Ok(B0xxState::random()));
        std::thread::sleep(wait);
    });

    Ok(rx)
}
