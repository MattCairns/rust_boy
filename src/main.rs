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

    let rom_path = "roms/pokemon.gb";

    let cartridge = Cartridge::load(rom_path); //.expect("Could not read ROM");
    let header = CartridgeHeader::new(&cartridge.data);

    let tile_test = Tile::new(header.logo[0..16].to_vec());
    // let tile_map_1 = cartridge.data[0x8000..0x87FF + 1].to_vec();
    let mem_region_1 = 0x8000;
    let mem_region_2 = 0x8800;
    let tile_sz = 16;
    let tile_id_offset = 128;

    (0..255).for_each(|id| {
        let tile_addr = mem_region_1 + (id * tile_sz);
        println!("{:X?}", tile_addr);
        println!(
            "{}",
            Tile::new(cartridge.data[tile_addr..tile_addr + 16].to_vec())
        );
    });
    // (0..255).for_each(|id| {
    //
    //p
    //Sprite attrs at 0xFE00-0xFE9F
    (0..40).for_each(|sprite| {
        let loc = 0xFE00 + (sprite * 4) + 2;
        let tile_loc: usize = cartridge.data[loc] as usize;
        let tile_addr = mem_region_2 + (tile_loc * tile_sz);
        println!("{:X?}", tile_addr);
        println!("{:X?}", tile_addr + 15);
        println!(
            "{}",
            Tile::new(cartridge.data[tile_addr..tile_addr + 16].to_vec())
        );
    });
    // });
    /* for i in (0..0x8FFF - 0x8000).step_by(16) {
        println!("{}", Tile::new(tile_map_2[i..i + 16].to_vec()));
    } */
    let test: Vec<u8> = vec![
        0xFF, 0x00, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97, 0x7E,
        0xFF,
    ];
    println!("{}", Tile::new(test));

    println!("ROM Size: {}kb", cartridge.data.len() / 1024);
    println!("{}", header);
    println!("{}", tile_test);
    header.print_logo();
}
