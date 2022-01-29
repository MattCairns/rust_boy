use crate::memorymap::{MemSectors, MemoryMap};

#[derive(Default)]
struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub struct Cpu<'m> {
    registers: Registers,
    sp: u16,
    pc: u16,
    mem: &'m MemoryMap,
}

impl<'m> Cpu<'m> {
    pub fn load(mut mem: &'m MemoryMap) -> Self {
        println!(
            "First instruction: {}",
            mem.read_byte(0x8100).expect("Could not read byte")
        );
        Self {
            registers: Registers::default(),
            sp: 0x0000,
            pc: 0x0100,
            mem,
        }
    }

    pub fn step(&mut self) {
        let opcode = self.mem.read_byte(self.pc).unwrap();
        match opcode {
            0x00 => self.nop(),
            0xC3 => self.jp_nn(),
            _ => println!("Opcode not implmented : 0x{:X}", opcode),
        }
    }

    fn nop(&mut self) {
        self.pc += 1
    }

    fn jp_nn(&mut self) {
        let lo = self.mem.read_byte(self.pc + 0x01).unwrap();
        let hi = self.mem.read_byte(self.pc + 0x02).unwrap();
        let jp_loc = ((hi as u16) << 8) | lo as u16;
        self.pc = jp_loc;
    }
    /*
    fn xor_AA(&mut self) {

    } */
}
