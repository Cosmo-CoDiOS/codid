//! Module for interfacing with `CoDi` over serial.
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

use serialport::open_with_settings;
use serialport::prelude::*;
use serialport::Error;
use std::time::Duration;

/// Returns the CoDi header magic number.
/// We had to put this in a function because `hex::decode(T)` can't be called from a const context.
#[allow(dead_code)]
fn get_codi_msg_header() -> Vec<u8> {
    hex::decode("58 21 58 21")
        .unwrap()
}

fn process_serial() {
    let mut serial = open_port("/dev/ttyS1")
        .unwrap();

    loop {
        let header = get_codi_msg_header();
        let mut buf: [u8; 300] = [0; 300]; // should this be 299?
        serial.read_exact(&mut buf).expect("Failed to read bytes from CoDi.");

        if buf.len() >= 4 {
            println!("Found the header!");
            let mut msg_size: [u8; 4] = [0; 4];
            serial.read_exact(&mut msg_size).expect("Failed to read message size header from CoDi.");
        }
    }
}

/// This function opens a serial connection to `CoDi`.
#[allow(dead_code)]
pub(crate) fn open_port(port: &str) -> Result<Box<dyn SerialPort>, Error> {
    let s = SerialPortSettings {
        baud_rate: 115_200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1),
    };

    open_with_settings(port, &s)
}
