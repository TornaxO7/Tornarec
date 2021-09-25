use crate::cpus::endianess::Endianess;
use crate::ram::Ram;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingState {
    Arm,
    Thumb,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Arm7TDMI {

}

impl Arm7TDMI {
    pub fn fetch(&mut self, ram: &Ram) {

    }

    pub fn decode(&self) {
    }

    pub fn execute(&self) {
    }

    pub fn get_endianess(&self) -> Endianess {
        Endianess::Big
    }
}
