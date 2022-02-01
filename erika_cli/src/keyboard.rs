// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use uinput::event;
use uinput::event::keyboard::{Key::*, Misc};

use nix::errno::Errno;

use erika_3004::{ControlCode, InputEvent, TypewriterInterface};

pub struct ErikaKeyboard {
    device: uinput::Device,
}

fn needs_shift_pressed(input: InputEvent) -> bool {
    matches!(input, InputEvent::Character(character) if
        character.is_uppercase()
            || matches!(character, '!' | '"' | '§' | '%' | '&' | '/' | '(' | ')' | '=' | '?' | '`' | '_' | ';' | ':' | '\'' | '*'))
}

impl ErikaKeyboard {
    pub fn new() -> uinput::Result<ErikaKeyboard> {
        Ok(ErikaKeyboard {
            device: uinput::default()?
                .name("Erika 3004")?
                .event(uinput::event::Keyboard::All)?
                .create()?,
        })
    }

    pub fn simulate_keypress(&mut self, input: InputEvent) {
        // Special case this key, it's in the Misc enum
        if let InputEvent::Character('|') = input {
            self.device
                .press(&RightAlt)
                .expect("Failed to press right alt key");
            self.device
                .press(&Misc::ND102)
                .expect("Failed to press 102ND key");
            self.device
                .release(&Misc::ND102)
                .expect("Failed to release 102ND key");
            self.device
                .release(&RightAlt)
                .expect("Failed to release right alt key");
            self.device
                .synchronize()
                .expect("Failed to simulate keypress");

            return;
        }

        let keyboard_event: event::keyboard::Key = match input {
            InputEvent::ControlCode(code) => match code {
                ControlCode::DeleteLastChar => BackSpace,
                ControlCode::HalfstepUp => Up,
                ControlCode::HalfstepDown => Down,
                ControlCode::HalfstepLeft => Left,
                ControlCode::MarginAllDel => Home,
                ControlCode::Relocate => End,
                ControlCode::MarginSet => Esc,
                ControlCode::Row1 => return,
                ControlCode::GetPaper => return,
                ControlCode::Chars10PerInch => return,
                _ => {
                    eprintln!("Unimplemented control code: {:?}", code);
                    return;
                }
            },
            InputEvent::Character(character) => match character {
                '1' | '!' => _1,
                '2' | '"' => _2,
                '3' | '§' => _3,
                '4' | '$' => _4,
                '5' | '%' => _5,
                '6' | '&' => _6,
                '7' | '/' => _7,
                '8' | '(' => _8,
                '9' | ')' => _9,
                '0' | '=' => _0,
                'ß' | '?' => Minus,
                'Ü' | 'ü' => LeftBrace,
                'Ä' | 'ä' => Apostrophe,
                'Ö' | 'ö' => SemiColon,
                '+' | '*' => RightBrace,
                '\t' => Tab,
                'Q' | 'q' => Q,
                'W' | 'w' => W,
                'E' | 'e' => E,
                'R' | 'r' => R,
                'T' | 't' => T,
                'Y' | 'y' => Z,
                'U' | 'u' => U,
                'I' | 'i' => I,
                'O' | 'o' => O,
                'P' | 'p' => P,
                '\n' => Enter,
                'A' | 'a' => A,
                'S' | 's' => S,
                'D' | 'd' => D,
                'F' | 'f' => F,
                'G' | 'g' => G,
                'H' | 'h' => H,
                'J' | 'j' => J,
                'K' | 'k' => K,
                'L' | 'l' => L,
                ';' => Comma,
                '\'' | '#' => BackSlash,
                'Z' | 'z' => Y,
                'X' | 'x' => X,
                'C' | 'c' => C,
                'V' | 'v' => V,
                'B' | 'b' => B,
                'N' | 'n' => N,
                'M' | 'm' => M,
                ',' => Comma,
                '.' | ':' => Dot,
                '-' | '_' => Slash,
                ' ' => Space,
                '´' => Equal,
                '`' => Equal,
                '\x08' => Right,
                _ => {
                    println!("keyboard: Unimplemented character code {:?}", character);
                    return;
                }
            },
        };

        if needs_shift_pressed(input) {
            self.device
                .press(&LeftShift)
                .expect("Failed to press shift key");
        }

        self.device.click(&keyboard_event).unwrap();

        if needs_shift_pressed(input) {
            self.device
                .release(&LeftShift)
                .expect("Failed to release shift key");
        }

        self.device
            .synchronize()
            .expect("Failed to simulate keypress");
    }
}

pub fn watch_keyboard_input(interface: &mut TypewriterInterface) -> erika_3004::Result<()> {
    match ErikaKeyboard::new() {
        Ok(mut virtual_keyboard) => loop {
            if let Some(character) = interface.read_character()? {
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

    Ok(())
}
