// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use erika_3004::TypewriterInterface;

use std::fs;
use std::io;

use clap::{App, AppSettings, Arg};

use nix::errno::Errno;

const SERIAL_DEVICE: &str = "/dev/ttyUSB0";

mod keyboard;

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
            App::new("print-file")
                .about("Print a text file")
                .arg(Arg::new("NAME").required(true)),
        )
        .subcommand(App::new("keyboard").about(
            "Connect the typewriter as a keyboard. You need to run erika-cli enable-keyboard afterwards to make the machine print again.",
        ))
        .subcommand(App::new("print").about("Print text from stdin"))
        .subcommand(App::new("bell").about("Sound the bell"))
        .subcommand(App::new("enable-keyboard").about("Re-enable direct printing of key presses"))
        .subcommand(
            App::new("move-paper")
                .arg(
                    Arg::new("STEP")
                        .help("Value needs to be 1 or 2")
                        .default_value("2")
                        .required(false),
                )
                .about("Move the paper"),
        )
        .get_matches();

    if let Some(subcommand) = matches.subcommand() {
        let device = matches
            .value_of("device")
            .expect("device should have a default value");

        let mut interface = TypewriterInterface::new(device)?;

        match subcommand {
            ("print-file", print_args) => {
                let path = print_args.value_of("NAME").expect("NAME is required");
                interface.enable_remote_mode()?;
                interface.write_unicode(&fs::read_to_string(path)?)?;
                interface.disable_remote_mode()?;
            }
            ("print", _) => {
                println!("Info: Text typed here will be printed.");
                println!("Info: Exit by pressing Ctrl + D");
                interface.enable_remote_mode()?;

                let mut buffer = String::new();
                let stdin = io::stdin();

                loop {
                    let size = stdin.read_line(&mut buffer)?;
                    if size <= 1 {
                        break;
                    }

                    interface.write_unicode(&buffer)?;
                }

                interface.disable_remote_mode()?;
            }
            ("keyboard", _) => {
                interface.enable_remote_mode()?;

                match keyboard::ErikaKeyboard::new() {
                    Ok(mut virtual_keyboard) => loop {
                        if let Some(character) = interface.read_character() {
                            virtual_keyboard.simulate_keypress(character);
                        }
                    },
                    Err(uinput::Error::Nix(nix::Error::Sys(Errno::EACCES))) => {
                        eprintln!(
                            r#"Error: Not enough permissions to simulate keyboard input.
Either run erika-cli keyboard as root, or add

    KERNEL=="uinput", OWNER="<YOUR USERNAME HERE>"

to /etc/udev/rules.d/erika.rules."#
                        );
                    }
                    Err(e) => panic!("Unexpected error occurred: {:?}", e),
                }
            }
            ("bell", _) => {
                interface.bell()?;
            }
            ("move-paper", move_args) => {
                let arg = move_args
                    .value_of("STEP")
                    .expect("Default value exists")
                    .parse::<u8>();
                match arg {
                    Ok(steps) => {
                        if !(2..=6).contains(&steps) {
                            interface.move_paper(steps)?;
                        } else {
                            eprintln!(
                                "Invalid number of paper steps passed. 3, 4, 5, 6 may not be used."
                            );
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "The argument passed as paper steps was not a parsable number: {}",
                            e
                        );
                    }
                }
            }
            ("enable-keyboard", _) => {
                interface.disable_remote_mode()?;
            }
            _ => {}
        }
    }

    Ok(())
}
