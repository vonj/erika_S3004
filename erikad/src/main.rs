// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use erika_3004::TypewriterInterface;

use std::io;

const SERIAL_DEVICE: &str = "/dev/ttyUSB0";

fn main() -> io::Result<()> {
    let mut interface = TypewriterInterface::new(SERIAL_DEVICE)?;

    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut buffer)?;
        interface.write_unicode(&buffer)?;
        if let Some(character) = interface.read_character() {
            use erika_3004::InputEvent::*;
            match character {
                ControlCode(code) => println!("{:?}", code),
                Character(character) => print!("{}", character),
            }
        }
    }
}
