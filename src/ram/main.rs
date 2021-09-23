use crate::ram::address::Address;
use crate::ram::data::Data;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum RamError {
    #[error("Address: `{0:X}` can't be accessed, because")]
    RamOverflow(Address),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Default)]
pub struct Ram {
    ram: Vec<u8>,
    max_size: Option<usize>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            ram: Vec::new(),
            max_size: None,
        }
    }

    pub fn load_data(&mut self, data: Data, starting_address: Address) {
        let last_address = starting_address.get_ref() + data.len();
        if self.ram.len() < last_address {
            self.ram.resize(last_address, 0);
        }

        self.ram[starting_address.get()..].copy_from_slice(&data);
    }

    pub fn set_max_size(&mut self, new_max_size: usize) {
        self.max_size = Some(new_max_size);
    }
}
