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
    pub fn dmg0() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
        }
    }
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
    mem: &'m mut MemoryMap,
}

impl<'m> Cpu<'m> {
    pub fn load(mut mem: &'m mut MemoryMap) -> Self {
        println!(
            "First instruction: {}",
            mem.read_byte(0x8100).expect("Could not read byte")
        );
        Self {
            registers: Registers::dmg0(),
            sp: 0xFFFF,
            pc: 0x0100,
            mem,
        }
    }

    pub fn step(&mut self) -> u8 {
        let opcode = self.mem.read_byte(self.pc).unwrap();
        println!("0x{:X?}", opcode);
        match opcode {
            0x00 => self.nop(),
            0xC3 => self.jp_nn(),
            0xAF => self.xor_AA(),
            0x21 => self.ld_HL_nn(),
            0xDF => self.rst_18(),
            0x0E => self.ld_C_n(),
            0x06 => self.ld_B_n(),
            // 0xFF => self.rst_38(),
            _ => {
                println!("Opcode not implmented : 0x{:X}", opcode);
                std::process::abort()
            }
        }
    }

    fn nop(&mut self) -> u8 {
        self.pc += 1;
        4
    }

    fn jp_nn(&mut self) -> u8 {
        self.pc += 1;
        let lo = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;
        let hi = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;
        let jp_loc = ((hi as u16) << 8) | lo as u16;
        println!("jp {:X?}", jp_loc);
        self.pc = jp_loc;
        16
    }

    fn xor_AA(&mut self) -> u8 {
        self.registers.a = self.registers.a ^ self.registers.a;
        self.registers.set_zero_flag();
        self.registers.clear_half_carry_flag();
        self.registers.clear_carry_flag();
        self.registers.clear_sub_flag();
        self.pc += 1;
        4
    }

    fn ld_HL_nn(&mut self) -> u8 {
        //TODO Add helper functions to set HL and other combo registers
        self.pc += 1;
        let n0 = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;
        let n1 = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;

        self.registers.h = n0;
        self.registers.l = n1;

        12
    }

    fn ld_C_n(&mut self) -> u8 {
        self.pc += 1;
        let n = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;

        self.registers.c = n;

        8
    }

    fn ld_B_n(&mut self) -> u8 {
        self.pc += 1;
        let n = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;

        self.registers.b = n;

        8
    }
    fn rst_18(&mut self) -> u8 {
        self.push();
        self.pc = 0x0000 + 0x18;
        32
    }

    fn rst_38(&mut self) -> u8 {
        self.push();
        self.pc = 0x0000 + 0x38;
        32
    }

    fn push(&mut self) {
        let bytes = self.pc.to_be_bytes();
        self.sp -= 1;
        self.mem.write_byte(self.sp, bytes[0]).unwrap();
        self.sp -= 1;
        self.mem.write_byte(self.sp, bytes[1]).unwrap();
    }

    fn pop(&mut self) -> u16 {
        let lo = self.mem.read_byte(self.sp).unwrap();
        self.sp += 1;
        let hi = self.mem.read_byte(self.sp).unwrap();
        self.sp += 1;
        ((hi as u16) << 8) | lo as u16
    }
}
