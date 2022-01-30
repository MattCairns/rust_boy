use rust_boy::cartridge::Cartridge;
use rust_boy::cpu::Cpu;
use rust_boy::header::Header;
use rust_boy::memorymap::MemoryMap;

fn main() {
    let rom_path = "roms/tetris.gb";

    let cartridge = Cartridge::load(rom_path);
    let header = Header::new(&cartridge.data);
    if !header.is_compatible() {
        println!(
            "{:?} is not supported by rust_boy yet.",
            header.cartridge_type
        );
        std::process::exit(0);
    }

    let mut memmap = MemoryMap::default();
    memmap.load_cartridge(&cartridge);

    let mut cpu = Cpu::load(&mut memmap);

    header.print_logo();
    println!("ROM Size: {}kb", cartridge.data.len() / 1024);
    println!("{}", header);

    loop {
        cpu.step();
    }

    // println!("{:?}", memmap.memory)

    // let tile_test = Tile::new(header.logo[0..16].to_vec());
    // let tile_map_1 = cartridge.data[0x8000..0x87FF + 1].to_vec();
    // let mut mem_region_1 = 0x64010;
    // let tile_sz = 16;
    // let tile_id_offset = 128;

    /* (0..384).for_each(|id| {
           // let tile_addr = mem_region_1 + (id * tile_sz);
           // println!("{:X?}", tile_addr);
           // println!("{:X?}", tile_addr + 15);
           println!("{:X?}", mem_region_1);
           println!(
               "{}",
               Tile::new(cartridge.data[mem_region_1..mem_region_1 + 16].to_vec())
           );

           mem_region_1 += 16;
       });
       println!(
           "{}",
           Tile::new(cartridge.data[0x640A0..0x640A0 + 16].to_vec())
       );
    */
    /* (0..40).for_each(|sprite| {
        // The OAM (Object Attribute Map) is empty until runtime
        let y_pos = cartridge.data[0xFE00 + (sprite * 4) + 0] as usize;
        let x_pos = cartridge.data[0xFE00 + (sprite * 4) + 1] as usize;
        let tile_index = cartridge.data[0xFE00 + (sprite * 4) + 2] as usize;
        let attrs = cartridge.data[0xFE00 + (sprite * 4) + 3] as usize;
        println!("y_pos: {}", y_pos);
        println!("x_pos: {}", x_pos);
        println!("tile_index: {:?}", tile_index);
        println!("attrs: {}", attrs);

        let tile_addr = mem_region_1 + (tile_index * tile_sz);
        println!("{:X?}", tile_addr);
        println!("{:X?}", tile_addr + 15);
        println!(
            "{}",
            Tile::new(cartridge.data[tile_addr..tile_addr + 16].to_vec())
        );
    }); */
}
