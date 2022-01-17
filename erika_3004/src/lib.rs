use std::fs::File;
use std::io::Result as IOResult;
use std::io::{Read, Write};
use std::path::Path;

#[repr(u8)]
pub enum BoudRate {
    Rate1200 = 10,
    Rate2400 = 8,
    Rate4800 = 4,
    Rate9600 = 2,
    Rate19200 = 1,
}

#[repr(u8)]
pub enum PaperStep {
    Step1,
    Step2,
}

#[repr(u8)]
pub enum ControlCode {
    Space = 0x71,
    Backspace,
    HalfstepRight,
    HalfstepLeft,
    HalfstepDown,
    HalfstepUp,
    Newline,
    CarriageReturn,
    HorizontabTab,
    TabSet,
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

pub struct TypewriterInterface {
    file: File,
}

impl TypewriterInterface {
    pub fn new(device: &Path) -> IOResult<TypewriterInterface> {
        Ok(TypewriterInterface {
            file: File::open(device)?,
        })
    }

    fn write(&mut self, code: ControlCode) -> IOResult<()> {
        self.file.write(&[code as u8, 'H' as u8])?;
        Ok(())
    }

    pub fn read(&mut self) -> IOResult<Vec<u8>> {
        let mut buf = Vec::<u8>::new();
        self.file.read(&mut buf)?;
        Ok(buf)
    }

    pub fn send_space(&mut self) -> IOResult<()> {
        self.write(ControlCode::Space)?;
        Ok(())
    }

    pub fn send_backspace(&mut self) -> IOResult<()> {
        self.write(ControlCode::Backspace)?;
        Ok(())
    }

    pub fn send_bell(&mut self) -> IOResult<()> {
        self.write(ControlCode::Bell)?;
        Ok(())
    }

    pub fn send_set_tab_size(&mut self, strength: u8) -> IOResult<()> {
        self.write(ControlCode::TabStep)?;
        self.file.write(&[strength])?;
        Ok(())
    }

    pub fn send_newline(&mut self) -> IOResult<()> {
        self.write(ControlCode::Newline)?;
        Ok(())
    }

    pub fn send_move_paper(&mut self, step: PaperStep) -> IOResult<()> {
        self.write(ControlCode::MovePaper)?;
        self.file.write(&[step as u8])?;
        Ok(())
    }
}
