use std::fs::File;
use std::io::prelude::*;

pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn load(path: &str) -> Self {
        let data = read_file_as_bytes(path).unwrap();
        Self { data }
    }
}

pub fn read_file_as_bytes(path: &str) -> Result<Vec<u8>, &'static str> {
    let mut f = File::open(path).expect("Cartridge could not be opened");
    let mut cartridge = Vec::new();

    f.read_to_end(&mut cartridge)
        .expect("Could not read cartridge");

    Ok(cartridge)
}
