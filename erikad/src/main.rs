// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use erika_3004::TypewriterInterface;

use std::io;

const SERIAL_DEVICE: &str = "/dev/ttyUSB0";

fn main() -> io::Result<()> {
    let mut interface = TypewriterInterface::new(SERIAL_DEVICE)?;
    loop {
        let mut buf = Vec::<u8>::with_capacity(3); // 3 is the maximum number of bytes used for a multi-byte character
        if let Ok(size) = interface.read(&mut buf) {
            if size > 0 {
                if let Some(text) = gdrascii_codec::decode_char(&buf[0..size]) {
                    println!("{}", text);
                }
            }
        }
    }
}
