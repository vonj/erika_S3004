// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use erika_3004::TypewriterInterface;

use std::io;

const SERIAL_DEVICE: &str = "/dev/ttyUSB0";

fn main() -> io::Result<()> {
    let mut interface = TypewriterInterface::new(SERIAL_DEVICE)?;
    loop {
        if let Some(character) = interface.read_character() {
            print!("{}", character);
        }
    }
}
