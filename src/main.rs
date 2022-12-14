use std::env::{self, args};
use rust_boy::cartridge::Cartridge;
use rust_boy::cpu::Cpu;
use rust_boy::interrupts::InterruptHandler;
use rust_boy::memorymap::MemoryMap;

fn main() {
    // LOAD CARTRIDGE
    let args: Vec<String> = env::args().collect();
    let rom_path = &args[1];
    println!("{}", rom_path);
    let cartridge = Cartridge::load(rom_path);
    let memmap = MemoryMap::default();
    memmap.load_cartridge(&cartridge);
    let mut cpu = Cpu::load(&memmap);
    // let mut interrupt = InterruptHandler::new(&memmap);
    //
    let mut cycles : u64 = 0;

    loop {
        let cpud = cpu.get_cpu_data_debug();

        println!(
            "A:{:0>2X} F:{:0>2X} B:{:0>2X} C:{:0>2X} D:{:0>2X} E:{:0>2X} H:{:0>2X} L:{:0>2X} SP:{:0>4X} PC:{:0>4X} PCMEM:{:0>2X},{:0>2X},{:0>2X},{:0>2X}",
            cpud.a,
            cpud.f,
            cpud.b,
            cpud.c,
            cpud.d,
            cpud.e,
            cpud.h,
            cpud.l,
            cpud.sp,
            cpud.pc,
            cpud.mem0,
            cpud.mem1,
            cpud.mem2,
            cpud.mem3,
        );

        cycles += cpu.step() as u64;
        // interrupt.update_ie();
        // interrupt.update_if();
    }
}
