pub mod ram;
pub mod rom_reader;

use crate::ram::main::Ram;
use crate::rom_reader::RomReader;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NintendoDS {
    arm7tdmi: u32,
    ram: Ram,
}

impl NintendoDS {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_file<S: AsRef<str>>(&mut self, path: S) {
        let rom_reader = RomReader::new(path);
    }
}
