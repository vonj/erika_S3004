// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use std::io;
use std::io::{Read, Write};

use std::time::Duration;

use std::fmt::{Display, Formatter};

use serial::prelude::*;

use num_enum::TryFromPrimitive;

use gdrascii_codec::EncodingError;

#[derive(Debug)]
pub enum ErikaError {
    IO(io::Error),
    Serial(serial::Error),
    UnknownCode(u8),
    InvalidBellDuration,
}

impl Display for ErikaError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        use ErikaError::*;

        match self {
            IO(e) => e.fmt(fmt),
            Serial(e) => e.fmt(fmt),
            UnknownCode(code) => write!(fmt, "Data received should either be in the codec range or a control code. This may indicate a character missing in the codec implementation. Code was {}", code),
            InvalidBellDuration => write!(fmt, "Bell duration can not be encoded in a u8")
        }
    }
}

impl From<io::Error> for ErikaError {
    fn from(e: io::Error) -> ErikaError {
        ErikaError::IO(e)
    }
}

impl From<serial::Error> for ErikaError {
    fn from(e: serial::Error) -> ErikaError {
        ErikaError::Serial(e)
    }
}

pub type Result<T> = std::result::Result<T, ErikaError>;

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
    pub fn new(device: &str) -> Result<TypewriterInterface> {
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
        self.file.write(&gdrascii_codec::encode(text))
    }

    /// Send a control code
    fn send_control(&mut self, code: ControlCode) -> io::Result<()> {
        self.file.write_all(&[code as u8])?;
        Ok(())
    }

    /// Read a character from a serial device. The character is decoded along the way.
    pub fn read_character(&mut self) -> Result<Option<InputEvent>> {
        let mut buf = [0; 3]; // 3 is the maximum number of bytes used for a multi-byte character
        if let Ok(size) = self.file.read(&mut buf) {
            if size > 0 {
                return match gdrascii_codec::decode_char(&buf[0..size]) {
                    Ok(text) => Ok(Some(InputEvent::Character(text))),
                    Err(EncodingError::InvalidInput) => {
                        let byte = buf[0].try_into();
                        match byte {
                            Ok(control_code) => Ok(Some(InputEvent::ControlCode(control_code))),
                            Err(_) => Err(ErikaError::UnknownCode(buf[0])),
                        }
                    }
                    _ => Ok(None),
                };
            }
        }

        Ok(None)
    }

    /// Sound the bell
    pub fn bell(&mut self, duration: Duration) -> Result<()> {
        let time_code = (duration.as_millis() / 20).try_into(); // One step on the typewriter is 20ms
        match time_code {
            Ok(steps) => {
                self.send_control(ControlCode::Bell)?;
                self.file.write_all(&[steps])?;
                Ok(())
            }
            Err(_) => Err(ErikaError::InvalidBellDuration),
        }
    }

    pub fn set_tab_size(&mut self, strength: u8) -> Result<()> {
        self.send_control(ControlCode::TabStep)?;
        self.file.write_all(&[strength])?;
        Ok(())
    }

    /// Move the paper. The step size is 1/240.
    /// The steps 3, 4, 5, 6 are invalid, and may not be used.
    pub fn move_paper(&mut self, step: u8) -> Result<()> {
        assert!(!(2..=6).contains(&step));

        self.send_control(ControlCode::MovePaper)?;
        self.file.write_all(&[step as u8])?;
        Ok(())
    }

    /// Split keyboard and printing. Key presses only be sent to the computer, not printed.
    pub fn enable_remote_mode(&mut self) -> Result<()> {
        self.send_control(ControlCode::KeyboardOff)?;
        Ok(())
    }

    /// Connect keyboard and printing. Key presses will directly result in printing.
    pub fn disable_remote_mode(&mut self) -> Result<()> {
        self.send_control(ControlCode::KeyboardOn)?;
        Ok(())
    }
}
