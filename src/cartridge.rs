use std::fs::File;
use std::io::prelude::*;

pub fn load_cartridge(path: &str) -> Result<Vec<u8>, &'static str> {
    let mut f = File::open(path).expect("Cartridge could not be opened");
    let mut cartridge = Vec::new();

    f.read_to_end(&mut cartridge)
        .expect("Could not read cartridge");

    Ok(cartridge)
}
