// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use erika_3004::TypewriterInterface;

use std::fs;
use std::io;
use std::io::Read;

use std::time::Duration;

use clap::{App, AppSettings, Arg};

const SERIAL_DEVICE: &str = "/dev/ttyUSB0";

#[cfg(target_os = "linux")]
mod keyboard;

fn main() -> erika_3004::Result<()> {
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

                let mut stdin = io::stdin();

                loop {
                    let mut buffer = Vec::<u8>::new();
                    buffer.resize(20, 0);

                    let size = stdin.read(&mut buffer)?;
                    if size <= 0 {
                        break;
                    }

                    let decoded = String::from_utf8(buffer[0..size].to_vec()).unwrap();

                    interface.write_unicode(&decoded)?;
                }

                interface.disable_remote_mode()?;
            }
            #[cfg(target_os = "linux")]
            ("keyboard", _) => {
                interface.enable_remote_mode()?;
                keyboard::watch_keyboard_input(&mut interface)?;
            }
            ("bell", _) => {
                interface.bell(Duration::from_secs(1))?;
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
