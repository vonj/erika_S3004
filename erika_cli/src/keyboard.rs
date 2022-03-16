// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use uinput::event;
use uinput::event::keyboard::{Key::*, Misc};

use erika_3004::{ControlCode, InputEvent, TypewriterInterface};

pub struct ErikaKeyboard {
    device: uinput::Device,
}

fn needs_shift_pressed(input: InputEvent) -> bool {
    matches!(input, InputEvent::Character(character) if
        character.is_uppercase()
            || matches!(character, '!' | '"' | '§' | '$' | '%' | '&' | '/' | '(' | ')' | '=' | '?' | '`' | '_' | ';' | ':' | '\'' | '*'))
}

fn needs_ctrl_pressed(input: InputEvent) -> bool {
    matches!(input, InputEvent::Character(character) if
        character < '\x20')
}

impl ErikaKeyboard {
    pub fn new() -> uinput::Result<ErikaKeyboard> {
        Ok(ErikaKeyboard {
            device: uinput::default()?
                .name("Erika 300x")?
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
                ControlCode::HalfstepRight => Right,
                ControlCode::MarginAllDel => Home,
                ControlCode::Relocate => End,
                ControlCode::MarginSet => Esc,
                ControlCode::Row1 => return,
                ControlCode::GetPaper => return,
                ControlCode::Chars10PerInch => return,
                ControlCode::Enter => Enter,
		        ControlCode::Backstep => Insert,
		        ControlCode::Tab => Tab,
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
                'Q' | 'q' | '\x11' => Q,
                'W' | 'w' | '\x17' => W,
                'E' | 'e' | '\x05' => E,
                'R' | 'r' | '\x12' => R,
                'T' | 't' | '\x14' => T,
                'Y' | 'y' | '\x1A' => Z,
                'U' | 'u' | '\x15' => U,
                'I' | 'i' | '\x09' => I,
                'O' | 'o' | '\x0F' => O,
                'P' | 'p' | '\x10' => P,
                'A' | 'a' | '\x01' => A,
                'S' | 's' | '\x13' => S,
                'D' | 'd' | '\x04' => D,
                'F' | 'f' | '\x06' => F,
                'G' | 'g' | '\x07' => G,
                'H' | 'h' | '\x08' => H,
                'J' | 'j' | '\x0A' => J,
                'K' | 'k' | '\x0B' => K,
                'L' | 'l' | '\x0C' => L,
                ';' => Comma,
                '\'' | '#' => BackSlash,
                'Z' | 'z' | '\x19' => Y,
                'X' | 'x' | '\x18' => X,
                'C' | 'c' | '\x03' => C,
                'V' | 'v' | '\x16' => V,
                'B' | 'b' | '\x02' => B,
                'N' | 'n' | '\x0E' => N,
                'M' | 'm' | '\x0D' => M,
                ',' => Comma,
                '.' | ':' => Dot,
                '-' | '_' => Slash,
                ' ' => Space,
                '´' => Equal,
                '`' => Equal,
                // '\t' => Tab,
                // '\n' => Enter,
                // '\x08' => Right,
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

        if needs_ctrl_pressed(input) {
            self.device
                .press(&LeftControl)
                .expect("Failed to press ctrl key");
        }	

	self.device.click(&keyboard_event).unwrap();

        if needs_ctrl_pressed(input) {
            self.device
                .release(&LeftControl)
                .expect("Failed to release ctrl key");
        }

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
        Err(uinput::Error::Nix(nix::Error::EACCES)) => {
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
