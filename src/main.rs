extern crate ndarray;
use ndarray::Array3;

use rust_boy::cartridge::Cartridge;
use rust_boy::cartridge::CartridgeHeader;
use rust_boy::tile::Tile;
// const CARTRIDGE_SIZE: usize = 0x200000;

/* fn update(cartridge: Vec<u8>) {
    const MAX_CYCLES: u32 = 69905;
    let mut cycles = 0;
} */

fn main() {
    const SCREEN_WIDTH: usize = 160;
    const SCREEN_HEIGHT: usize = 144;
    const RGB_SZ: usize = 3;
    const RAM_SZ: usize = 0x10000;
    let mut screen = Array3::<u8>::zeros((SCREEN_WIDTH, SCREEN_HEIGHT, RGB_SZ));
    let mut ram: Vec<u8> = Vec::with_capacity(RAM_SZ);

    let rom_path = "roms/test.gb";

    let cartridge = Cartridge::load(rom_path); //.expect("Could not read ROM");
    let header = CartridgeHeader::new(&cartridge.data);

    let tile_test = Tile::new(header.logo[0..16].to_vec());

    println!("ROM Size: {}kb", cartridge.data.len() / 1024);
    println!("{}", header);
    println!("{}", tile_test);
    header.print_logo();
}
