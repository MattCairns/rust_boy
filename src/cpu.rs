use crate::memorymap::MemoryMap;
use crate::registers::{dec, AnyReg, AnyRegN, DecRegisters, JumpCond, Registers};

pub struct Cpu<'m> {
    reg: Registers,
    sp: u16,
    pc: u16,
    mem: &'m mut MemoryMap,
}

impl<'m> Cpu<'m> {
    pub fn load(mem: &'m mut MemoryMap) -> Self {
        Self {
            reg: Registers::dmg0(),
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
            0x3D => self.dec_reg(DecRegisters::A),
            0x05 => self.dec_reg(DecRegisters::B),
            0x0D => self.dec_reg(DecRegisters::C),
            0x15 => self.dec_reg(DecRegisters::D),
            0x1D => self.dec_reg(DecRegisters::E),
            0x25 => self.dec_reg(DecRegisters::H),
            0x2D => self.dec_reg(DecRegisters::L),
            0x20 => self.jp_cond(JumpCond::NZ),
            0x28 => self.jp_cond(JumpCond::Z),
            0x30 => self.jp_cond(JumpCond::NC),
            0x38 => self.jp_cond(JumpCond::C),
            0x7F => self.ld_r_r(AnyReg::A, AnyReg::A),
            0x78 => self.ld_r_r(AnyReg::A, AnyReg::B),
            0x79 => self.ld_r_r(AnyReg::A, AnyReg::C),
            0x7A => self.ld_r_r(AnyReg::A, AnyReg::D),
            0x7B => self.ld_r_r(AnyReg::A, AnyReg::E),
            0x7C => self.ld_r_r(AnyReg::A, AnyReg::H),
            0x7D => self.ld_r_r(AnyReg::A, AnyReg::L),
            0x7E => self.ld_r_r(AnyReg::A, AnyReg::HL),
            0x40 => self.ld_r_r(AnyReg::B, AnyReg::B),
            0x41 => self.ld_r_r(AnyReg::B, AnyReg::C),
            0x42 => self.ld_r_r(AnyReg::B, AnyReg::D),
            0x43 => self.ld_r_r(AnyReg::B, AnyReg::E),
            0x44 => self.ld_r_r(AnyReg::B, AnyReg::H),
            0x45 => self.ld_r_r(AnyReg::B, AnyReg::L),
            0x46 => self.ld_r_r(AnyReg::B, AnyReg::HL),
            0x48 => self.ld_r_r(AnyReg::C, AnyReg::B),
            0x49 => self.ld_r_r(AnyReg::C, AnyReg::C),
            0x4A => self.ld_r_r(AnyReg::C, AnyReg::D),
            0x4B => self.ld_r_r(AnyReg::C, AnyReg::E),
            0x4C => self.ld_r_r(AnyReg::C, AnyReg::H),
            0x4D => self.ld_r_r(AnyReg::C, AnyReg::L),
            0x4E => self.ld_r_r(AnyReg::C, AnyReg::HL),
            0x50 => self.ld_r_r(AnyReg::D, AnyReg::B),
            0x51 => self.ld_r_r(AnyReg::D, AnyReg::C),
            0x52 => self.ld_r_r(AnyReg::D, AnyReg::D),
            0x53 => self.ld_r_r(AnyReg::D, AnyReg::E),
            0x54 => self.ld_r_r(AnyReg::D, AnyReg::H),
            0x55 => self.ld_r_r(AnyReg::D, AnyReg::L),
            0x56 => self.ld_r_r(AnyReg::D, AnyReg::HL),
            0x58 => self.ld_r_r(AnyReg::E, AnyReg::B),
            0x59 => self.ld_r_r(AnyReg::E, AnyReg::C),
            0x5A => self.ld_r_r(AnyReg::E, AnyReg::D),
            0x5B => self.ld_r_r(AnyReg::E, AnyReg::E),
            0x5C => self.ld_r_r(AnyReg::E, AnyReg::H),
            0x5D => self.ld_r_r(AnyReg::E, AnyReg::L),
            0x5E => self.ld_r_r(AnyReg::E, AnyReg::HL),
            0x60 => self.ld_r_r(AnyReg::H, AnyReg::B),
            0x61 => self.ld_r_r(AnyReg::H, AnyReg::C),
            0x62 => self.ld_r_r(AnyReg::H, AnyReg::D),
            0x63 => self.ld_r_r(AnyReg::H, AnyReg::E),
            0x64 => self.ld_r_r(AnyReg::H, AnyReg::H),
            0x65 => self.ld_r_r(AnyReg::H, AnyReg::L),
            0x66 => self.ld_r_r(AnyReg::H, AnyReg::HL),
            0x68 => self.ld_r_r(AnyReg::L, AnyReg::B),
            0x69 => self.ld_r_r(AnyReg::L, AnyReg::C),
            0x6A => self.ld_r_r(AnyReg::L, AnyReg::D),
            0x6B => self.ld_r_r(AnyReg::L, AnyReg::E),
            0x6C => self.ld_r_r(AnyReg::L, AnyReg::H),
            0x6D => self.ld_r_r(AnyReg::L, AnyReg::L),
            0x6E => self.ld_r_r(AnyReg::L, AnyReg::HL),
            0x70 => self.ld_r_r(AnyReg::HL, AnyReg::B),
            0x71 => self.ld_r_r(AnyReg::HL, AnyReg::C),
            0x72 => self.ld_r_r(AnyReg::HL, AnyReg::D),
            0x73 => self.ld_r_r(AnyReg::HL, AnyReg::E),
            0x74 => self.ld_r_r(AnyReg::HL, AnyReg::H),
            0x75 => self.ld_r_r(AnyReg::HL, AnyReg::L),
            0x36 => self.ld_r_r(AnyReg::HL, AnyReg::HL),
            0x06 => self.ld_b_n(),
            0x0E => self.ld_c_n(),
            0x21 => self.ld_hl_nn(),
            0x32 => self.ld_mem_hl_a(),
            0xC3 => self.jp_nn(),
            0xAF => self.xor_aa(),
            0xDF => self.rst_18(),
            0xFF => self.rst_38(),
            0x1F => self.rr_n(AnyReg::A),
            // 0x18 => self.rr_n(AnyReg::B),
            // 0x19 => self.rr_n(AnyReg::C),
            // 0x1A => self.rr_n(AnyReg::D),
            // 0x1B => self.rr_n(AnyReg::E),
            // 0x1C => self.rr_n(AnyReg::H),
            // 0x1D => self.rr_n(AnyReg::L),
            // 0x1E => self.rr_n(AnyReg::HL),
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

    fn adc(&mut self, reg: AnyRegN) -> u8 {
        let cycles = 4;
        self.pc += 1;

        match reg {
            AnyRegN::A => {
                self.reg.a += self.reg.a;
            }
            AnyRegN::B => todo!(),
            AnyRegN::C => todo!(),
            AnyRegN::D => todo!(),
            AnyRegN::E => todo!(),
            AnyRegN::H => todo!(),
            AnyRegN::L => todo!(),
            AnyRegN::HL => todo!(),
            AnyRegN::N => todo!(),
        }

        cycles
    }

    fn dec_reg(&mut self, reg: DecRegisters) -> u8 {
        self.pc += 1;
        let half_carry: bool = match reg {
            DecRegisters::A => {
                let dec = dec(self.reg.a, 0x01);
                self.reg.a = dec.0;
                dec.1
            }
            DecRegisters::B => {
                let dec = dec(self.reg.b, 0x01);
                self.reg.b = dec.0;
                dec.1
            }
            DecRegisters::C => {
                let dec = dec(self.reg.c, 0x01);
                self.reg.c = dec.0;
                dec.1
            }
            DecRegisters::D => {
                let dec = dec(self.reg.d, 0x01);
                self.reg.d = dec.0;
                dec.1
            }
            DecRegisters::E => {
                let dec = dec(self.reg.e, 0x01);
                self.reg.e = dec.0;
                dec.1
            }
            DecRegisters::H => {
                let dec = dec(self.reg.h, 0x01);
                self.reg.h = dec.0;
                dec.1
            }
            DecRegisters::L => {
                let dec = dec(self.reg.l, 0x01);
                self.reg.l = dec.0;
                dec.1
            }
            DecRegisters::HL => todo!(),
        };

        if half_carry {
            self.reg.set_half_carry_flag();
        }

        self.reg.set_sub_flag();

        match reg {
            DecRegisters::HL => 12,
            _ => 4,
        }
    }

    fn jp_cond(&mut self, cond: JumpCond) -> u8 {
        if cond.check(self.reg.f) {
            self.pc += 1;
            self.pc = self.pc + self.mem.read_byte(self.pc).unwrap() as u16;
        }

        8
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

    fn xor_aa(&mut self) -> u8 {
        self.reg.a = self.reg.a ^ self.reg.a;
        self.reg.unset_all_flags();
        self.reg.set_zero_flag();
        self.pc += 1;
        4
    }

    fn ld_r_r(&mut self, r1: AnyReg, r2: AnyReg) -> u8 {
        println!("LD {:?} {:?}", r1, r2);
        let mut cycles = 4;
        let ret = match r1 {
            AnyReg::A => match r2 {
                AnyReg::A => self.reg.a = self.reg.a,
                AnyReg::B => self.reg.a = self.reg.b,
                AnyReg::C => self.reg.a = self.reg.c,
                AnyReg::D => self.reg.a = self.reg.d,
                AnyReg::E => self.reg.a = self.reg.e,
                AnyReg::H => self.reg.a = self.reg.h,
                AnyReg::L => self.reg.a = self.reg.l,
                AnyReg::HL => {
                    cycles = cycles + 4;
                    self.reg.a = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            AnyReg::B => match r2 {
                AnyReg::A => self.reg.b = self.reg.a,
                AnyReg::B => self.reg.b = self.reg.b,
                AnyReg::C => self.reg.b = self.reg.c,
                AnyReg::D => self.reg.b = self.reg.d,
                AnyReg::E => self.reg.b = self.reg.e,
                AnyReg::H => self.reg.b = self.reg.h,
                AnyReg::L => self.reg.b = self.reg.l,
                AnyReg::HL => {
                    cycles = cycles + 4;
                    self.reg.b = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            AnyReg::C => match r2 {
                AnyReg::A => self.reg.c = self.reg.a,
                AnyReg::B => self.reg.c = self.reg.b,
                AnyReg::C => self.reg.c = self.reg.c,
                AnyReg::D => self.reg.c = self.reg.d,
                AnyReg::E => self.reg.c = self.reg.e,
                AnyReg::H => self.reg.c = self.reg.h,
                AnyReg::L => self.reg.c = self.reg.l,
                AnyReg::HL => {
                    cycles = cycles + 4;
                    self.reg.c = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            AnyReg::D => match r2 {
                AnyReg::A => self.reg.d = self.reg.a,
                AnyReg::B => self.reg.d = self.reg.b,
                AnyReg::C => self.reg.d = self.reg.c,
                AnyReg::D => self.reg.d = self.reg.d,
                AnyReg::E => self.reg.d = self.reg.e,
                AnyReg::H => self.reg.d = self.reg.h,
                AnyReg::L => self.reg.d = self.reg.l,
                AnyReg::HL => {
                    cycles = cycles + 4;
                    self.reg.d = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            AnyReg::E => match r2 {
                AnyReg::A => self.reg.e = self.reg.a,
                AnyReg::B => self.reg.e = self.reg.b,
                AnyReg::C => self.reg.e = self.reg.c,
                AnyReg::D => self.reg.e = self.reg.d,
                AnyReg::E => self.reg.e = self.reg.e,
                AnyReg::H => self.reg.e = self.reg.h,
                AnyReg::L => self.reg.e = self.reg.l,
                AnyReg::HL => {
                    cycles = cycles + 4;
                    self.reg.e = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            AnyReg::H => match r2 {
                AnyReg::A => self.reg.h = self.reg.a,
                AnyReg::B => self.reg.h = self.reg.b,
                AnyReg::C => self.reg.h = self.reg.c,
                AnyReg::D => self.reg.h = self.reg.d,
                AnyReg::E => self.reg.h = self.reg.e,
                AnyReg::H => self.reg.h = self.reg.h,
                AnyReg::L => self.reg.h = self.reg.l,
                AnyReg::HL => {
                    cycles = cycles + 4;
                    self.reg.h = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            AnyReg::L => match r2 {
                AnyReg::A => self.reg.l = self.reg.a,
                AnyReg::B => self.reg.l = self.reg.b,
                AnyReg::C => self.reg.l = self.reg.c,
                AnyReg::D => self.reg.l = self.reg.d,
                AnyReg::E => self.reg.l = self.reg.e,
                AnyReg::H => self.reg.l = self.reg.h,
                AnyReg::L => self.reg.l = self.reg.l,
                AnyReg::HL => {
                    cycles = cycles + 4;
                    self.reg.l = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            AnyReg::HL => {
                cycles = cycles + 4;
                match r2 {
                    AnyReg::A => self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap(),
                    AnyReg::B => self.mem.write_byte(self.reg.get_hl(), self.reg.b).unwrap(),
                    AnyReg::C => self.mem.write_byte(self.reg.get_hl(), self.reg.c).unwrap(),
                    AnyReg::D => self.mem.write_byte(self.reg.get_hl(), self.reg.d).unwrap(),
                    AnyReg::E => self.mem.write_byte(self.reg.get_hl(), self.reg.e).unwrap(),
                    AnyReg::H => self.mem.write_byte(self.reg.get_hl(), self.reg.h).unwrap(),
                    AnyReg::L => self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap(),
                    AnyReg::HL => {
                        self.pc += 1;
                        self.mem
                            .write_byte(self.reg.get_hl(), self.mem.read_byte(self.pc).unwrap())
                            .unwrap();
                        0x00
                    }
                };
            }
        };
        self.pc += 1;
        cycles
    }

    fn ld_mem_hl_a(&mut self) -> u8 {
        self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap();
        self.pc += 1;

        8
    }

    fn ld_hl_nn(&mut self) -> u8 {
        self.pc += 1;
        let n0 = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;
        let n1 = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;

        self.reg.h = n0;
        self.reg.l = n1;

        12
    }

    fn ld_c_n(&mut self) -> u8 {
        self.pc += 1;
        let n = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;

        self.reg.c = n;

        8
    }

    fn ld_b_n(&mut self) -> u8 {
        self.pc += 1;
        let n = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;

        self.reg.b = n;

        8
    }
    fn rst_18(&mut self) -> u8 {
        self.push();
        self.pc = 0x0018;
        32
    }

    fn rst_38(&mut self) -> u8 {
        self.push();
        self.pc = 0x0038;
        32
    }

    fn rr_n(&mut self, reg: AnyReg) -> u8 {
        let mut cycles = 8;
        let c;

        match reg {
            AnyReg::A => {
                c = self.reg.a & 0x01;
                self.reg.a = self.reg.a.rotate_right(1);
                if self.reg.is_carry() {
                    self.reg.a = self.reg.a | 0x80;
                } else {
                    self.reg.a = self.reg.a & 0x7F;
                }
            }
            AnyReg::B => {
                c = self.reg.a & 0x01;
                self.reg.a = self.reg.a.rotate_right(1);
                if self.reg.is_carry() {
                    self.reg.a = self.reg.a | 0x80;
                } else {
                    self.reg.a = self.reg.a & 0x7F;
                }
            }
            AnyReg::C => {
                c = self.reg.a & 0x01;
                self.reg.a = self.reg.a.rotate_right(1);
                if self.reg.is_carry() {
                    self.reg.a = self.reg.a | 0x80;
                } else {
                    self.reg.a = self.reg.a & 0x7F;
                }
            }
            AnyReg::D => {
                c = self.reg.a & 0x01;
                self.reg.a = self.reg.a.rotate_right(1);
                if self.reg.is_carry() {
                    self.reg.a = self.reg.a | 0x80;
                } else {
                    self.reg.a = self.reg.a & 0x7F;
                }
            }
            AnyReg::E => {
                c = self.reg.a & 0x01;
                self.reg.a = self.reg.a.rotate_right(1);
                if self.reg.is_carry() {
                    self.reg.a = self.reg.a | 0x80;
                } else {
                    self.reg.a = self.reg.a & 0x7F;
                }
            }
            AnyReg::H => {
                c = self.reg.a & 0x01;
                self.reg.a = self.reg.a.rotate_right(1);
                if self.reg.is_carry() {
                    self.reg.a = self.reg.a | 0x80;
                } else {
                    self.reg.a = self.reg.a & 0x7F;
                }
            }
            AnyReg::L => {
                c = self.reg.a & 0x01;
                self.reg.a = self.reg.a.rotate_right(1);
                if self.reg.is_carry() {
                    self.reg.a = self.reg.a | 0x80;
                } else {
                    self.reg.a = self.reg.a & 0x7F;
                }
            }
            AnyReg::HL => {
                cycles = cycles + 8;
                let mut val = self.mem.read_byte(self.reg.get_hl()).unwrap();
                c = val & 0x01;
                val = val.rotate_right(1);
                if self.reg.is_carry() {
                    val |= 0x80;
                } else {
                    val &= 0x7F;
                }
                self.mem.write_byte(self.reg.get_hl(), val).unwrap();
            }
        };

        if c == 0x01 {
            self.reg.set_carry_flag();
        } else {
            self.reg.unset_carry_flag();
        }

        self.pc += 1;

        cycles // HL = 16
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
