// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use erika_3004::TypewriterInterface;

use std::fs;
use std::io;

use clap::{App, AppSettings, Arg};

const SERIAL_DEVICE: &str = "/dev/ttyUSB0";

fn main() -> io::Result<()> {
    let matches = App::new("erika-cli")
        .arg(
            Arg::new("device")
                .short('d')
                .long("device")
                .help("Serial device to use, usually /dev/ttyUSB0")
                .default_value(SERIAL_DEVICE),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("print")
                .about("Print a text file")
                .arg(Arg::new("NAME").required(true)),
        )
        .subcommand(App::new("connect").about(
            "Connect to the typewriter, send text from stdin and print received text to stdout",
        ))
        .subcommand(App::new("bell").about("Sound the bell"))
        .get_matches();

    match matches.subcommand() {
        Some(subcommand) => {
            let device = matches
                .value_of("device")
                .expect("device should have a default value");

            let mut interface = TypewriterInterface::new(device)?;

            match subcommand {
                ("print", print_args) => {
                    let path = print_args.value_of("NAME").expect("NAME is required");
                    interface.write_unicode(&fs::read_to_string(path)?)?;
                }
                ("connect", _) => {
                    let mut buffer = String::new();
                    let stdin = io::stdin();
                    loop {
                        stdin.read_line(&mut buffer)?;
                        interface.write_unicode(&buffer)?;
                        if let Some(character) = interface.read_character() {
                            use erika_3004::InputEvent::*;
                            match character {
                                ControlCode(code) => print!(" {:?} ", code),
                                Character(character) => print!("{}", character),
                            }
                        }
                    }
                }
                ("bell", _) => {
                    interface.bell()?;
                }
                _ => {}
            }
        }
        None => {}
    }

    Ok(())
}
