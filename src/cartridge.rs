use std::fs::File;
use std::io::prelude::*;

/* const CARTRIDGE_SIZE: usize = 0x200000;
const HEADER_SIZE: usize = 0x4F;
const HEADER_START: usize = 0x0100;
const HEADER_END: usize = 0x014F; */
pub struct Cartridge {
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn load(path: &str) -> Self {
        let data = read_file_as_bytes(path).unwrap();
        /* let header: [u8; HEADER_SIZE] = data[HEADER_START..HEADER_END]
        .try_into()
        .expect("Slice is not corrent length"); */
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
