// SPDX-FileCopyrightText: 2022 Jonah Brüchert <jbb@kaidan.im>
//
// SPDX-License-Identifier: EUPL-1.2

use std::io;
use std::io::{Read, Write};

use serial::prelude::*;

/// Boud rates supported by the typewriter
#[repr(u8)]
pub enum BoudRate {
    Rate1200 = 10,
    Rate2400 = 8,
    Rate4800 = 4,
    Rate9600 = 2,
    Rate19200 = 1,
}

/// Possible steps to move the paper by
#[repr(u8)]
pub enum PaperStep {
    Step1,
    Step2,
}

/// Possible control codes to send
#[repr(u8)]
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
    KeyboardOff,
    KeyboardOn,
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
    KeyStrength,
    TabStep = 0xA3,
    MovePaper,
    RotateWheel,
    MoveTape,
    DoublePrint = 0xA9,
    Bell,
    KeyboardInput,
    KeyboardInput2,
    DeleteRelocate,
    DeleteLastChar,
    Relocate,
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
        self.write(&[code as u8])?;
        Ok(())
    }

    /// Read a character from a serial device. The character is decoded along the way.
    pub fn read_character(&mut self) -> Option<char> {
        let mut buf = Vec::<u8>::with_capacity(3); // 3 is the maximum number of bytes used for a multi-byte character
        if let Ok(size) = self.read(&mut buf) {
            if size > 0 {
                if let Ok(text) = gdrascii_codec::decode_char(&buf[0..size]) {
                    return Some(text);
                }
            }
        }

        None
    }

    /// Sound the bell
    pub fn bell(&mut self) -> io::Result<()> {
        self.send_control(ControlCode::Bell)?;
        Ok(())
    }

    pub fn set_tab_size(&mut self, strength: u8) -> io::Result<()> {
        self.send_control(ControlCode::TabStep)?;
        self.file.write(&[strength])?;
        Ok(())
    }

    pub fn move_paper(&mut self, step: PaperStep) -> io::Result<()> {
        self.send_control(ControlCode::MovePaper)?;
        self.file.write(&[step as u8])?;
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
