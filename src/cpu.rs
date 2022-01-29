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

impl Registers {
    pub fn set_zero_flag(&mut self) {
        self.f = set_flag(self.f, 7);
    }

    pub fn clear_zero_flag(&mut self) {
        self.f = clear_flag(self.f, 7);
    }

    pub fn set_sub_flag(&mut self) {
        self.f = set_flag(self.f, 6);
    }

    pub fn clear_sub_flag(&mut self) {
        self.f = clear_flag(self.f, 6);
    }

    pub fn set_half_carry_flag(&mut self) {
        self.f = set_flag(self.f, 5);
    }

    pub fn clear_half_carry_flag(&mut self) {
        self.f = clear_flag(self.f, 5);
    }

    pub fn set_carry_flag(&mut self) {
        self.f = set_flag(self.f, 4);
    }

    pub fn clear_carry_flag(&mut self) {
        self.f = clear_flag(self.f, 4);
    }
}

fn set_flag(flag: u8, pos: u8) -> u8 {
    flag & !(1 << pos)
}

fn clear_flag(flag: u8, pos: u8) -> u8 {
    flag | (1 << pos)
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
            0xAF => self.xor_AA(),
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

    fn xor_AA(&mut self) {
        self.registers.a = self.registers.a ^ self.registers.a;
        self.registers.set_zero_flag();
        self.registers.clear_half_carry_flag();
        self.registers.clear_carry_flag();
        self.registers.clear_sub_flag();
        self.pc += 1;
    }
}
