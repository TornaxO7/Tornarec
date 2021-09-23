use std::fs::File;
use std::io::Read;

use nds::parser::NDSParser;

use core::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomReader {
    rom_content: Vec<u8>,
    parser: NDSParser,
}

impl RomReader {
    pub fn new<R: AsRef<str>>(path: R) -> Self {
        let path = path.as_ref();
        let mut rom_content: Vec<u8> = Vec::new();

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("Couldn't open the file: {}", path);
                println!("{}", err);
                panic!("Stopping, due to error above.");
            }
        };

        if let Err(err) = file.read_to_end(&mut rom_content) {
            println!("Couldn't read the content of the file: {}", path);
            println!("{}", err);
            panic!("Stopping, due to error above.");
        };

        let parser = match NDSParser::try_from(&rom_content) {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("Couldn't parse the given file: {}", path);
                println!("{}", err);
                panic!("Aborting...");
            }
        };

        Self {
            rom_content,
            parser,
        }
    }
}
