//Spencer Denton
//Created: 5/2/2021
//This listens and sends to Tiva TM4C123GH6PM LaunchPad's UART0 using windows COM5
use serial::{prelude::*, windows::COMPort, PortSettings};
use std::{
    ffi::OsStr,
    io::{Read, Write},
};

const STX: u8 = 0x02;
const ETX: u8 = 0x03;

pub fn uart0_listen(port: &mut COMPort) -> anyhow::Result<String> {
    let mut buf = String::new();
    let mut iter = port.bytes();
    let mut stx_flag: bool = false;

    loop {
        let next_byte = iter.next();

        match next_byte {
            Some(byte) => {
                match byte {
                    Ok(character) => {
                        if character == STX {
                            stx_flag = true;
                            continue;
                        } else if character == ETX {
                            if stx_flag == false {
                                buf = String::new();
                                continue;
                            } else {
                                return Ok(buf);
                            }
                        } else if stx_flag == true {
                            buf.push(character as char);
                            continue;
                        }
                    }
                    Err(_) => {
                        continue;
                    }
                };
            }
            None => break,
        };
    }

    Err(anyhow::Error::msg("Couldn't Read"))
}

pub fn uart0_write_one_message(port: &mut COMPort, message: &[u8]) -> anyhow::Result<usize> {
    let tx_len = port.write(message)?;
    assert_eq!(tx_len, message.len());
    port.flush()?;
    Ok(tx_len)
}

pub fn uart0_init_port() -> anyhow::Result<COMPort> {
    let mut port = serial::open(OsStr::new("COM5"))?;
    const SETTINGS: PortSettings = PortSettings {
        baud_rate: serial::BaudRate::Baud110,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone,
    };

    port.configure(&SETTINGS)?;

    Ok(port)
}
