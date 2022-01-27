// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use std::io;
use std::io::{Read, Write};

use std::time::Duration;

use serial::prelude::*;

use num_enum::TryFromPrimitive;

use gdrascii_codec::EncodingError;

/// Boud rates supported by the typewriter
#[repr(u8)]
pub enum BoudRate {
    Rate1200 = 10,
    Rate2400 = 8,
    Rate4800 = 4,
    Rate9600 = 2,
    Rate19200 = 1,
}

/// Possible control codes to send
#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Clone, Copy)]
pub enum ControlCode {
    // numbers before here are covered by the text codec
    HalfstepRight = 0x73,
    HalfstepLeft = 0x74,
    HalfstepDown = 0x75,
    HalfstepUp = 0x76,
    TabSet = 0x7a,
    TabDel,
    TabAllDel,
    TabStandard,
    MarginSet,
    MarginDel,
    MarginAllDel,
    MarginUnset,
    RowSizeDown,
    RowSizeUp,
    GetPaper,
    Row1,
    Row1Point5,
    Row2,
    Chars10PerInch,
    Chars12PerInch,
    Chars15PerInch,
    DeleteOff,
    DeleteOn,
    BackwardsOn,
    BackwardsOff,
    RightMarginOn,
    MarginSetOffInofficial,
    KeyboardOff = 0x91,
    KeyboardOn = 0x92,
    Reset,
    PrinterReady,
    SecondCharsetOff,
    SecondCharsetOn,
    AutorepeatOn = 0x9B,
    AutorepeatOff,
    AutorepeatOffAndPilgrimNormal = 0x9D,
    Pilgrim = 0x9E,
    LineDown,
    AutorepeatAllOn,
    BoudRate,
    KeyStrength = 0xA3,
    TabStep = 0xA5,
    MovePaper = 0xA6,
    RotateWheel,
    MoveTape,
    DoublePrint = 0xA9,
    Bell = 0xAA,
    KeyboardInput = 0xAB,
    KeyboardInput2,
    DeleteRelocate,
    DeleteLastChar,
    Relocate,
}

/// Classification of an input event
#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    ControlCode(ControlCode),
    Character(char),
}

/// Interface for receiving and sending text to the typewriter
pub struct TypewriterInterface {
    file: serial::SystemPort,
}

impl TypewriterInterface {
    /// Open a new serial connection to a device
    pub fn new(device: &str) -> io::Result<TypewriterInterface> {
        let mut port = serial::open(device)?;
        port.reconfigure(&|settings| {
            settings.set_baud_rate(serial::Baud1200)?;
            settings.set_char_size(serial::Bits8);
            settings.set_flow_control(serial::FlowControl::FlowHardware);
            Ok(())
        })?;

        Ok(TypewriterInterface { file: port })
    }

    /// Send a unicode encoded rust string to the typewriter. The data will be encoded with the proprietary codec before sending.
    /// Returns the number of bytes written
    pub fn write_unicode(&mut self, text: &str) -> io::Result<usize> {
        self.write(&gdrascii_codec::encode(text))
    }

    /// Send a control code
    fn send_control(&mut self, code: ControlCode) -> io::Result<()> {
        self.write_all(&[code as u8])?;
        Ok(())
    }

    fn send_enter(&mut self) -> io::Result<()> {
        self.write_unicode("\n")?;
        Ok(())
    }

    /// Read a character from a serial device. The character is decoded along the way.
    pub fn read_character(&mut self) -> Option<InputEvent> {
        let mut buf = [0; 3]; // 3 is the maximum number of bytes used for a multi-byte character
        if let Ok(size) = self.read(&mut buf) {
            if size > 0 {
                return match gdrascii_codec::decode_char(&buf[0..size]) {
                    Ok(text) => Some(InputEvent::Character(text)),
                    Err(EncodingError::InvalidInput) => {
                        let byte = buf[0].try_into();
                        match byte {
                            Ok(control_code) => Some(InputEvent::ControlCode(control_code)),
                            Err(_) => {
                                eprintln!("Data received should either be in the codec range or a control code. This may indicate a character missing in the codec implementation. Code was {}", buf[0]);
                                None
                            }
                        }
                    }
                    _ => None,
                };
            }
        }

        None
    }

    /// Sound the bell
    pub fn bell(&mut self, duration: Duration) -> io::Result<()> {
        let time_code: Result<u8, _> = (duration.as_millis() / 20).try_into(); // One step on the typewriter is 20ms
        match time_code {
            Ok(steps) => {
                self.send_control(ControlCode::Bell)?;
                self.write_all(&[steps])?;
                Ok(())
            }
            Err(e) => {
                // TODO add our own error type and handle this better
                eprintln!("{}", e);
                Ok(())
            }
        }
    }

    pub fn set_tab_size(&mut self, strength: u8) -> io::Result<()> {
        self.send_control(ControlCode::TabStep)?;
        self.file.write_all(&[strength])?;
        self.send_enter()
    }

    /// Move the paper. The step size is 1/240.
    /// The steps 3, 4, 5, 6 are invalid, and may not be used.
    pub fn move_paper(&mut self, step: u8) -> io::Result<()> {
        assert!(!(2..=6).contains(&step));

        self.send_control(ControlCode::MovePaper)?;
        self.file.write_all(&[step as u8])?;
        self.send_enter()
    }

    /// Split keyboard and printing. Key presses only be sent to the computer, not printed.
    pub fn enable_remote_mode(&mut self) -> io::Result<()> {
        self.send_control(ControlCode::KeyboardOff)?;
        Ok(())
    }

    /// Connect keyboard and printing. Key presses will directly result in printing.
    pub fn disable_remote_mode(&mut self) -> io::Result<()> {
        self.send_control(ControlCode::KeyboardOn)?;
        Ok(())
    }
}

impl Read for TypewriterInterface {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

impl Write for TypewriterInterface {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.file.write(data)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
