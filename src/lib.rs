pub mod ram;
pub mod rom_reader;
pub mod cpus;

use crate::ram::Ram;
use crate::rom_reader::RomReader;
use crate::cpus::Arm7TDMI;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NintendoDS {
    arm7tdmi: Arm7TDMI,
    ram: Ram,
}

impl NintendoDS {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) {
        loop {
            self.arm7tdmi.step(&mut self.ram);
        }
    }

    pub fn load_file_to_ram<S: AsRef<str>>(&mut self, path: S) {
        let rom_reader = RomReader::new(path);
        rom_reader.load_arm7_tdmi(&mut self.ram);
    }
}
