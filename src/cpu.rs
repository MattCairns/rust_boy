use crate::memorymap::MemoryMap;
use crate::registers::*;

pub struct CpuData {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub pc: u16,
    pub sp: u16,
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

pub struct Cpu<'m> {
    reg: Registers,
    sp: u16,
    pc: u16,
    mem: &'m mut MemoryMap,
}

impl<'m> Cpu<'m> {
    pub fn load(mem: &'m mut MemoryMap) -> Self {
        // Load the state of the gameboy after
        // loading the boot rom.
        mem.write_byte(0xFF05, 0x00).unwrap();
        mem.write_byte(0xFF06, 0x00).unwrap();
        mem.write_byte(0xFF07, 0x00).unwrap();
        mem.write_byte(0xFF10, 0x80).unwrap();
        mem.write_byte(0xFF11, 0xBF).unwrap();
        mem.write_byte(0xFF12, 0xF3).unwrap();
        mem.write_byte(0xFF14, 0xBF).unwrap();
        mem.write_byte(0xFF16, 0x3F).unwrap();
        mem.write_byte(0xFF17, 0x00).unwrap();
        mem.write_byte(0xFF19, 0xBF).unwrap();
        mem.write_byte(0xFF1A, 0x7F).unwrap();
        mem.write_byte(0xFF1B, 0xFF).unwrap();
        mem.write_byte(0xFF1C, 0x9F).unwrap();
        mem.write_byte(0xFF1E, 0xBF).unwrap();
        mem.write_byte(0xFF20, 0xFF).unwrap();
        mem.write_byte(0xFF21, 0x00).unwrap();
        mem.write_byte(0xFF22, 0x00).unwrap();
        mem.write_byte(0xFF23, 0xBF).unwrap();
        mem.write_byte(0xFF24, 0x77).unwrap();
        mem.write_byte(0xFF25, 0xF3).unwrap();
        mem.write_byte(0xFF26, 0xF1).unwrap();
        mem.write_byte(0xFF40, 0x91).unwrap();
        mem.write_byte(0xFF42, 0x00).unwrap();
        mem.write_byte(0xFF43, 0x00).unwrap();
        mem.write_byte(0xFF45, 0x00).unwrap();
        mem.write_byte(0xFF47, 0xFC).unwrap();
        mem.write_byte(0xFF48, 0xFF).unwrap();
        mem.write_byte(0xFF49, 0xFF).unwrap();
        mem.write_byte(0xFF4A, 0x00).unwrap();
        mem.write_byte(0xFF4B, 0x00).unwrap();
        mem.write_byte(0xFFFF, 0x00).unwrap();
        Self {
            reg: Registers::dmg0(),
            sp: 0xFFFE,
            pc: 0x0100,
            mem,
        }
    }

    pub fn get_cpu_data(&self) -> CpuData {
        CpuData {
            af: self.reg.get_af(),
            bc: self.reg.get_bc(),
            de: self.reg.get_de(),
            hl: self.reg.get_hl(),
            pc: self.pc,
            sp: self.sp,
            z: self.reg.is_z(),
            n: self.reg.is_n(),
            h: self.reg.is_h(),
            c: self.reg.is_carry(),
        }
    }

    pub fn step(&mut self) -> u8 {
        /* let start = 0x8000;
        for i in (0..384) {
            let start = start + i * 16;
            self.mem.print_tile(start);
        } */
        let opcode = self.mem.read_byte(self.pc).unwrap();
        println!("[{:#06X?}] {:#04X?}", self.pc, opcode);
        match opcode {
            0x00 => self.nop(),                //tested
            0x0F => self.rrca(),               //tested
            0xC9 => self.ret(),                //tested
            0xC0 => self.ret_cc(FlagCond::NZ), //tested
            0xC8 => self.ret_cc(FlagCond::Z),  //tested
            0xD0 => self.ret_cc(FlagCond::NC), //tested
            0xD8 => self.ret_cc(FlagCond::C),  //tested
            0xCD => self.call(),
            0xC4 => self.call_cc(FlagCond::NZ),
            0xCC => self.call_cc(FlagCond::Z),
            0xD4 => self.call_cc(FlagCond::NC),
            0xDC => self.call_cc(FlagCond::C),
            0xE5 => self.push_hl(),
            0x3D => self.dec_reg(StdReg::A), //tested
            0x05 => self.dec_reg(StdReg::B), //tested
            0x0D => self.dec_reg(StdReg::C), //tested
            0x15 => self.dec_reg(StdReg::D), //tested
            0x1D => self.dec_reg(StdReg::E), //tested
            0x25 => self.dec_reg(StdReg::H), //tested
            0x2D => self.dec_reg(StdReg::L), //tested
            0x18 => self.jr(),
            0x20 => self.jr_cond(FlagCond::NZ),
            0x28 => self.jr_cond(FlagCond::Z),
            0x30 => self.jr_cond(FlagCond::NC),
            0x38 => self.jr_cond(FlagCond::C),
            0x01 => self.ld_n_nn(LoadRegnnn::BC),
            0x11 => self.ld_n_nn(LoadRegnnn::DE),
            0x21 => self.ld_n_nn(LoadRegnnn::HL),
            0x31 => self.ld_n_nn(LoadRegnnn::SP),
            0x7F => self.ld_a_n(LoadReg::A),     //tested
            0x78 => self.ld_a_n(LoadReg::B),     //tested
            0x79 => self.ld_a_n(LoadReg::C),     //tested
            0x7A => self.ld_a_n(LoadReg::D),     //tested
            0x7B => self.ld_a_n(LoadReg::E),     //tested
            0x7C => self.ld_a_n(LoadReg::H),     //tested
            0x7D => self.ld_a_n(LoadReg::L),     //tested
            0x0A => self.ld_a_n(LoadReg::MemBC), //tested
            0x1A => self.ld_a_n(LoadReg::MemDE), //tested
            0x7E => self.ld_a_n(LoadReg::MemHL), //tested
            0xFA => self.ld_a_n(LoadReg::MemNN), //tested
            0x3E => self.ld_a_n(LoadReg::N),     //tested
            0x47 => self.ld_n_a(LoadReg::B),
            0x4F => self.ld_n_a(LoadReg::C),
            0x57 => self.ld_n_a(LoadReg::D),
            0x5F => self.ld_n_a(LoadReg::E),
            0x67 => self.ld_n_a(LoadReg::H),
            0x6F => self.ld_n_a(LoadReg::L),
            0x02 => self.ld_n_a(LoadReg::MemBC),
            0x12 => self.ld_n_a(LoadReg::MemDE),
            0x77 => self.ld_n_a(LoadReg::MemHL),
            0xEA => self.ld_n_a(LoadReg::MemNN),
            0xE0 => self.ld_ff00_a(),
            0xF0 => self.ld_a_ff00(),
            0x40 => self.ld_r_r(StdReg::B, StdReg::B),
            0x41 => self.ld_r_r(StdReg::B, StdReg::C),
            0x42 => self.ld_r_r(StdReg::B, StdReg::D),
            0x43 => self.ld_r_r(StdReg::B, StdReg::E),
            0x44 => self.ld_r_r(StdReg::B, StdReg::H),
            0x45 => self.ld_r_r(StdReg::B, StdReg::L),
            0x46 => self.ld_r_r(StdReg::B, StdReg::HL),
            0x48 => self.ld_r_r(StdReg::C, StdReg::B),
            0x49 => self.ld_r_r(StdReg::C, StdReg::C),
            0x4A => self.ld_r_r(StdReg::C, StdReg::D),
            0x4B => self.ld_r_r(StdReg::C, StdReg::E),
            0x4C => self.ld_r_r(StdReg::C, StdReg::H),
            0x4D => self.ld_r_r(StdReg::C, StdReg::L),
            0x4E => self.ld_r_r(StdReg::C, StdReg::HL),
            0x50 => self.ld_r_r(StdReg::D, StdReg::B),
            0x51 => self.ld_r_r(StdReg::D, StdReg::C),
            0x52 => self.ld_r_r(StdReg::D, StdReg::D),
            0x53 => self.ld_r_r(StdReg::D, StdReg::E),
            0x54 => self.ld_r_r(StdReg::D, StdReg::H),
            0x55 => self.ld_r_r(StdReg::D, StdReg::L),
            0x56 => self.ld_r_r(StdReg::D, StdReg::HL),
            0x58 => self.ld_r_r(StdReg::E, StdReg::B),
            0x59 => self.ld_r_r(StdReg::E, StdReg::C),
            0x5A => self.ld_r_r(StdReg::E, StdReg::D),
            0x5B => self.ld_r_r(StdReg::E, StdReg::E),
            0x5C => self.ld_r_r(StdReg::E, StdReg::H),
            0x5D => self.ld_r_r(StdReg::E, StdReg::L),
            0x5E => self.ld_r_r(StdReg::E, StdReg::HL),
            0x60 => self.ld_r_r(StdReg::H, StdReg::B),
            0x61 => self.ld_r_r(StdReg::H, StdReg::C),
            0x62 => self.ld_r_r(StdReg::H, StdReg::D),
            0x63 => self.ld_r_r(StdReg::H, StdReg::E),
            0x64 => self.ld_r_r(StdReg::H, StdReg::H),
            0x65 => self.ld_r_r(StdReg::H, StdReg::L),
            0x66 => self.ld_r_r(StdReg::H, StdReg::HL),
            0x68 => self.ld_r_r(StdReg::L, StdReg::B),
            0x69 => self.ld_r_r(StdReg::L, StdReg::C),
            0x6A => self.ld_r_r(StdReg::L, StdReg::D),
            0x6B => self.ld_r_r(StdReg::L, StdReg::E),
            0x6C => self.ld_r_r(StdReg::L, StdReg::H),
            0x6D => self.ld_r_r(StdReg::L, StdReg::L),
            0x6E => self.ld_r_r(StdReg::L, StdReg::HL),
            0x70 => self.ld_r_r(StdReg::HL, StdReg::B),
            0x71 => self.ld_r_r(StdReg::HL, StdReg::C),
            0x72 => self.ld_r_r(StdReg::HL, StdReg::D),
            0x73 => self.ld_r_r(StdReg::HL, StdReg::E),
            0x74 => self.ld_r_r(StdReg::HL, StdReg::H),
            0x75 => self.ld_r_r(StdReg::HL, StdReg::L),
            0x36 => self.ld_r_r(StdReg::HL, StdReg::HL),
            0x2A => self.ldi_a_memhl(),
            0x06 => self.ld_b_n(),
            0x0E => self.ld_c_n(),
            0x32 => self.ld_mem_hl_a(),
            0xC3 => self.jp_nn(),
            0xAF => self.xor_aa(),
            0xC7 => self.rst_00(),
            0xCF => self.rst_08(),
            0xD7 => self.rst_10(),
            0xDF => self.rst_18(),
            0xE7 => self.rst_20(),
            0xEF => self.rst_28(),
            0xF7 => self.rst_30(),
            0xFF => self.rst_38(),
            0x1F => self.rr_n(StdReg::A),
            0x8F => self.adc_a_n(StdRegN::A),
            0x88 => self.adc_a_n(StdRegN::B),
            0x89 => self.adc_a_n(StdRegN::C),
            0x8A => self.adc_a_n(StdRegN::D),
            0x8B => self.adc_a_n(StdRegN::E),
            0x8C => self.adc_a_n(StdRegN::H),
            0x8D => self.adc_a_n(StdRegN::L),
            0x8E => self.adc_a_n(StdRegN::HL),
            0xCE => self.adc_a_n(StdRegN::N),
            0x3C => self.inc_reg(StdReg::A),
            0x04 => self.inc_reg(StdReg::B),
            0x0C => self.inc_reg(StdReg::C),
            0x14 => self.inc_reg(StdReg::D),
            0x1C => self.inc_reg(StdReg::E),
            0x24 => self.inc_reg(StdReg::H),
            0x2C => self.inc_reg(StdReg::L),
            0x34 => self.inc_reg(StdReg::HL),
            0xFE => self.cp_a_n(),

            0xF3 => {
                println!("(0F3) DI => Not implemented");
                self.pc = self.pc.wrapping_add(1);
                0
            }
            0xCB => {
                self.pc = self.pc.wrapping_add(1);
                let opcode = self.mem.read_byte(self.pc).unwrap();
                match opcode {
                    0x1F => self.rr_n(StdReg::A),
                    0x18 => self.rr_n(StdReg::B),
                    0x19 => self.rr_n(StdReg::C),
                    0x1A => self.rr_n(StdReg::D),
                    0x1B => self.rr_n(StdReg::E),
                    0x1C => self.rr_n(StdReg::H),
                    0x1D => self.rr_n(StdReg::L),
                    0x1E => self.rr_n(StdReg::HL),
                    _ => {
                        println!("Opcode not implmented : CB {:#04X}", opcode);
                        std::process::abort()
                    }
                }
            }
            _ => {
                println!("Opcode not implmented : {:#04X}", opcode);
                std::process::abort()
            }
        }
    }

    fn nop(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        4
    }

    fn rrca(&mut self) -> u8 {
        let carry = self.reg.a & 0b0000_0001;
        self.reg.a = self.reg.a.rotate_right(1);
        self.reg.unset_all_flags();
        if carry == 0b0000_0001 {
            self.reg.set_c();
        } else {
            self.reg.unset_c();
        }
        self.pc = self.pc.wrapping_add(1);
        4
    }

    fn adc_a_n(&mut self, reg: StdRegN) -> u8 {
        let cycles = 4;
        self.pc = self.pc.wrapping_add(1);

        let carry = self.reg.get_carry();

        macro_rules! adc {
            ($a:expr,$b:expr) => {{
                if add_will_half_carry($a, $b) {
                    self.reg.set_h();
                };
                if will_carry($a, $b) {
                    self.reg.set_c();
                };
                $a += $b + carry;
            }};
        }

        match reg {
            StdRegN::A => adc!(self.reg.a, self.reg.a),
            StdRegN::B => adc!(self.reg.a, self.reg.b),
            StdRegN::C => adc!(self.reg.a, self.reg.c),
            StdRegN::D => adc!(self.reg.a, self.reg.d),
            StdRegN::E => adc!(self.reg.a, self.reg.e),
            StdRegN::H => adc!(self.reg.a, self.reg.h),
            StdRegN::L => adc!(self.reg.a, self.reg.l),
            StdRegN::HL => todo!(),
            StdRegN::N => todo!(),
        }

        cycles
    }

    fn inc_reg(&mut self, reg: StdReg) -> u8 {
        let mut cycles = 4;
        let val: (u8, bool) = match reg {
            StdReg::A => {
                let inc = inc(self.reg.a, 0x01);
                self.reg.a = inc.0;
                inc
            }
            StdReg::B => {
                let inc = inc(self.reg.b, 0x01);
                self.reg.b = inc.0;
                inc
            }
            StdReg::C => {
                let inc = inc(self.reg.c, 0x01);
                self.reg.c = inc.0;
                inc
            }
            StdReg::D => {
                println!("DEC D {:#4X}", self.reg.d);
                let inc = inc(self.reg.d, 0x01);
                self.reg.d = inc.0;
                inc
            }
            StdReg::E => {
                let inc = inc(self.reg.e, 0x01);
                self.reg.e = inc.0;
                inc
            }
            StdReg::H => {
                let inc = inc(self.reg.h, 0x01);
                self.reg.h = inc.0;
                inc
            }
            StdReg::L => {
                let inc = inc(self.reg.l, 0x01);
                self.reg.l = inc.0;
                inc
            }
            StdReg::HL => todo!(),
        };

        if val.0 == 0x00 {
            self.reg.set_z();
        }

        if val.1 {
            self.reg.set_h();
        }

        self.reg.unset_n();

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn dec_reg(&mut self, reg: StdReg) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        let val: (u8, bool) = match reg {
            StdReg::A => {
                let dec = dec(self.reg.a, 0x01);
                self.reg.a = dec.0;
                dec
            }
            StdReg::B => {
                let dec = dec(self.reg.b, 0x01);
                self.reg.b = dec.0;
                dec
            }
            StdReg::C => {
                let dec = dec(self.reg.c, 0x01);
                self.reg.c = dec.0;
                dec
            }
            StdReg::D => {
                let dec = dec(self.reg.d, 0x01);
                self.reg.d = dec.0;
                dec
            }
            StdReg::E => {
                let dec = dec(self.reg.e, 0x01);
                self.reg.e = dec.0;
                dec
            }
            StdReg::H => {
                let dec = dec(self.reg.h, 0x01);
                self.reg.h = dec.0;
                dec
            }
            StdReg::L => {
                let dec = dec(self.reg.l, 0x01);
                self.reg.l = dec.0;
                dec
            }
            StdReg::HL => todo!(),
        };

        if val.0 == 0x00 {
            println!("SET ZERO");
            self.reg.set_z();
        } else {
            self.reg.unset_z();
        }

        if val.1 {
            self.reg.set_h();
        }

        self.reg.set_n();

        match reg {
            StdReg::HL => 12,
            _ => 4,
        }
    }

    fn jr(&mut self) -> u8 {
        let mut cycles = 8;
        cycles = 12;
        self.pc = self.pc.wrapping_add(1);
        let v = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);

        let sig: u16;
        let is_neg: bool;

        if (v & 0b1000_0000) == 0b1000_0000 {
            is_neg = true;
            sig = !v as u16 + 1;
        } else {
            is_neg = false;
            sig = v as u16;
        }

        if is_neg {
            self.pc -= sig;
        } else {
            self.pc += sig;
        }

        cycles
    }
    fn jr_cond(&mut self, cond: FlagCond) -> u8 {
        let cycles: u8 = 8;
        if cond.check(self.reg.f) {
            self.jr();
        } else {
            self.pc = self.pc.wrapping_add(1);
            self.pc = self.pc.wrapping_add(1);
        }

        cycles
    }

    fn jp_nn(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        let lo = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);
        let hi = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);
        let jp_loc = ((hi as u16) << 8) | lo as u16;
        println!("jp {:X?}", jp_loc);
        self.pc = jp_loc;
        16
    }

    fn xor_aa(&mut self) -> u8 {
        self.reg.a = self.reg.a ^ self.reg.a;
        self.reg.unset_all_flags();
        self.reg.set_z();
        self.pc = self.pc.wrapping_add(1);
        println!("XOR A",);
        4
    }

    fn ld_ff00_a(&mut self) -> u8 {
        //Must be between $FF00 and $FFFF
        let cycles = 12;

        let b = 0xFF00 + self.read_u8() as u16;

        println!("LD {:#6X} A", b);

        if b >= 0xFF00 && b < 0xFFFF {
            self.mem.write_byte(b, self.reg.a).unwrap();
        }

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn ld_a_ff00(&mut self) -> u8 {
        //Must be between $FF00 and $FFFF
        let cycles = 12;

        let b = 0xFF00 + self.read_u8() as u16;

        println!("LD A {:#6X}", b);

        if b >= 0xFF00 && b < 0xFFFF {
            self.reg.a = self.mem.read_byte(b).unwrap();
            println!("{:#4X}", self.mem.read_byte(b).unwrap());
        }

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn ld_r_r(&mut self, r1: StdReg, r2: StdReg) -> u8 {
        println!("LD {:?} {:?}", r1, r2);
        let mut cycles = 4;
        let ret = match r1 {
            StdReg::A => match r2 {
                StdReg::A => self.reg.a = self.reg.a,
                StdReg::B => self.reg.a = self.reg.b,
                StdReg::C => self.reg.a = self.reg.c,
                StdReg::D => self.reg.a = self.reg.d,
                StdReg::E => self.reg.a = self.reg.e,
                StdReg::H => self.reg.a = self.reg.h,
                StdReg::L => self.reg.a = self.reg.l,
                StdReg::HL => {
                    cycles = cycles + 4;
                    self.reg.a = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::B => match r2 {
                StdReg::A => self.reg.b = self.reg.a,
                StdReg::B => self.reg.b = self.reg.b,
                StdReg::C => self.reg.b = self.reg.c,
                StdReg::D => self.reg.b = self.reg.d,
                StdReg::E => self.reg.b = self.reg.e,
                StdReg::H => self.reg.b = self.reg.h,
                StdReg::L => self.reg.b = self.reg.l,
                StdReg::HL => {
                    cycles = cycles + 4;
                    self.reg.b = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::C => match r2 {
                StdReg::A => self.reg.c = self.reg.a,
                StdReg::B => self.reg.c = self.reg.b,
                StdReg::C => self.reg.c = self.reg.c,
                StdReg::D => self.reg.c = self.reg.d,
                StdReg::E => self.reg.c = self.reg.e,
                StdReg::H => self.reg.c = self.reg.h,
                StdReg::L => self.reg.c = self.reg.l,
                StdReg::HL => {
                    cycles = cycles + 4;
                    self.reg.c = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::D => match r2 {
                StdReg::A => self.reg.d = self.reg.a,
                StdReg::B => self.reg.d = self.reg.b,
                StdReg::C => self.reg.d = self.reg.c,
                StdReg::D => self.reg.d = self.reg.d,
                StdReg::E => self.reg.d = self.reg.e,
                StdReg::H => self.reg.d = self.reg.h,
                StdReg::L => self.reg.d = self.reg.l,
                StdReg::HL => {
                    cycles = cycles + 4;
                    self.reg.d = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::E => match r2 {
                StdReg::A => self.reg.e = self.reg.a,
                StdReg::B => self.reg.e = self.reg.b,
                StdReg::C => self.reg.e = self.reg.c,
                StdReg::D => self.reg.e = self.reg.d,
                StdReg::E => self.reg.e = self.reg.e,
                StdReg::H => self.reg.e = self.reg.h,
                StdReg::L => self.reg.e = self.reg.l,
                StdReg::HL => {
                    cycles = cycles + 4;
                    self.reg.e = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::H => match r2 {
                StdReg::A => self.reg.h = self.reg.a,
                StdReg::B => self.reg.h = self.reg.b,
                StdReg::C => self.reg.h = self.reg.c,
                StdReg::D => self.reg.h = self.reg.d,
                StdReg::E => self.reg.h = self.reg.e,
                StdReg::H => self.reg.h = self.reg.h,
                StdReg::L => self.reg.h = self.reg.l,
                StdReg::HL => {
                    cycles = cycles + 4;
                    self.reg.h = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::L => match r2 {
                StdReg::A => self.reg.l = self.reg.a,
                StdReg::B => self.reg.l = self.reg.b,
                StdReg::C => self.reg.l = self.reg.c,
                StdReg::D => self.reg.l = self.reg.d,
                StdReg::E => self.reg.l = self.reg.e,
                StdReg::H => self.reg.l = self.reg.h,
                StdReg::L => self.reg.l = self.reg.l,
                StdReg::HL => {
                    cycles = cycles + 4;
                    self.reg.l = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::HL => {
                cycles = cycles + 4;
                match r2 {
                    StdReg::A => self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap(),
                    StdReg::B => self.mem.write_byte(self.reg.get_hl(), self.reg.b).unwrap(),
                    StdReg::C => self.mem.write_byte(self.reg.get_hl(), self.reg.c).unwrap(),
                    StdReg::D => self.mem.write_byte(self.reg.get_hl(), self.reg.d).unwrap(),
                    StdReg::E => self.mem.write_byte(self.reg.get_hl(), self.reg.e).unwrap(),
                    StdReg::H => self.mem.write_byte(self.reg.get_hl(), self.reg.h).unwrap(),
                    StdReg::L => self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap(),
                    StdReg::HL => {
                        self.pc = self.pc.wrapping_add(1);
                        self.mem
                            .write_byte(self.reg.get_hl(), self.mem.read_byte(self.pc).unwrap())
                            .unwrap();
                        0x00
                    }
                };
            }
        };
        self.pc = self.pc.wrapping_add(1);
        cycles
    }

    fn ldi_a_memhl(&mut self) -> u8 {
        println!("LD A [HL++]");
        let mut hl = self.reg.get_hl();

        self.reg.a = self.mem.read_byte(hl).unwrap();
        hl = hl.wrapping_add(1);
        self.reg.set_hl(hl);

        self.pc = self.pc.wrapping_add(1);

        8
    }

    fn ld_mem_hl_a(&mut self) -> u8 {
        println!("LD [HL-] A");
        println!("{:#6X?}", self.reg.get_hl());
        self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap();
        self.reg.set_hl(self.reg.get_hl().overflowing_sub(1).0);
        self.pc = self.pc.wrapping_add(1);

        8
    }

    fn ld_a_n(&mut self, reg: LoadReg) -> u8 {
        let mut cycles = 4;
        println!("LD A {:?}", reg);

        match reg {
            LoadReg::A => self.reg.a = self.reg.a,
            LoadReg::B => self.reg.a = self.reg.b,
            LoadReg::C => self.reg.a = self.reg.c,
            LoadReg::D => self.reg.a = self.reg.d,
            LoadReg::E => self.reg.a = self.reg.e,
            LoadReg::H => self.reg.a = self.reg.h,
            LoadReg::L => self.reg.a = self.reg.l,
            LoadReg::MemBC => {
                cycles = 8;
                self.reg.a = self.mem.read_byte(self.reg.get_bc()).unwrap();
            }
            LoadReg::MemDE => {
                cycles = 8;
                self.reg.a = self.mem.read_byte(self.reg.get_de()).unwrap();
            }
            LoadReg::MemHL => {
                cycles = 8;
                self.reg.a = self.mem.read_byte(self.reg.get_hl()).unwrap();
            }
            LoadReg::MemNN => {
                cycles = 16;
                let loc = self.read_u16();
                self.reg.a = self.mem.read_byte(loc).unwrap();
            }
            LoadReg::N => {
                cycles = 8;
                self.reg.a = self.read_u8();
                println!("{:#4X}", self.reg.a);
            }
        }

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn ld_n_a(&mut self, reg: LoadReg) -> u8 {
        let mut cycles = 4;
        println!("LD {:?} A", reg);
        match reg {
            LoadReg::A => self.reg.a = self.reg.a,
            LoadReg::B => self.reg.b = self.reg.a,
            LoadReg::C => self.reg.c = self.reg.a,
            LoadReg::D => self.reg.d = self.reg.a,
            LoadReg::E => self.reg.e = self.reg.a,
            LoadReg::H => self.reg.h = self.reg.a,
            LoadReg::L => self.reg.l = self.reg.a,
            LoadReg::MemBC => {
                cycles = 8;
                self.mem.write_byte(self.reg.get_bc(), self.reg.a).unwrap();
            }
            LoadReg::MemDE => {
                cycles = 8;
                self.mem.write_byte(self.reg.get_de(), self.reg.a).unwrap();
            }
            LoadReg::MemHL => {
                cycles = 8;
                self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap();
            }
            LoadReg::MemNN => {
                cycles = 16;
                self.pc = self.pc.wrapping_add(1);
                let low = self.mem.read_byte(self.pc).unwrap();
                self.pc = self.pc.wrapping_add(1);
                let high = self.mem.read_byte(self.pc).unwrap();
                self.mem
                    .write_byte(self.reg.get_nn(low, high), self.reg.a)
                    .unwrap();
            }
            LoadReg::N => (),
        };

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn ld_n_nn(&mut self, reg: LoadRegnnn) -> u8 {
        let cycles = 12;

        let nn = self.read_u16();
        println!("LD {:?} {:#6X?}", reg, nn);

        match reg {
            LoadRegnnn::BC => self.reg.set_bc(nn),
            LoadRegnnn::DE => self.reg.set_de(nn),
            LoadRegnnn::HL => self.reg.set_hl(nn),
            LoadRegnnn::SP => self.sp = nn,
        }

        self.pc = self.pc.wrapping_add(1);
        cycles
    }

    fn ld_c_n(&mut self) -> u8 {
        self.reg.c = self.read_u8();

        println!("LD C {:#4X?}", self.reg.c);

        self.pc = self.pc.wrapping_add(1);

        8
    }

    fn ld_b_n(&mut self) -> u8 {
        self.reg.b = self.read_u8();
        println!("LD B {:#6X?}", self.reg.b);
        self.pc = self.pc.wrapping_add(1);

        8
    }

    fn rst(&mut self, pc: u16) -> u8 {
        self.pc += 1;
        let lo = self.mem.read_byte(self.pc).unwrap();
        self.pc += 1;
        let hi = self.mem.read_byte(self.pc).unwrap();
        self.push(lo, hi);
        self.pc = pc;
        32
    }
    fn rst_00(&mut self) -> u8 {
        self.rst(0x0000)
    }
    fn rst_08(&mut self) -> u8 {
        self.rst(0x0008)
    }
    fn rst_10(&mut self) -> u8 {
        self.rst(0x0010)
    }
    fn rst_18(&mut self) -> u8 {
        self.rst(0x0018)
    }
    fn rst_20(&mut self) -> u8 {
        self.rst(0x0020)
    }
    fn rst_28(&mut self) -> u8 {
        self.rst(0x0028)
    }
    fn rst_30(&mut self) -> u8 {
        self.rst(0x0030)
    }
    fn rst_38(&mut self) -> u8 {
        self.rst(0x0038)
    }

    fn rr_n(&mut self, reg: StdReg) -> u8 {
        let mut cycles = 8;
        let c;

        macro_rules! rr {
            ($a:expr) => {{
                c = $a & 0x01;
                $a = $a.rotate_right(1);
                if self.reg.is_carry() {
                    $a = $a | 0x80;
                } else {
                    $a = $a & 0x7F;
                }
            }};
        }

        match reg {
            StdReg::A => rr!(self.reg.a),
            StdReg::B => rr!(self.reg.b),
            StdReg::C => rr!(self.reg.c),
            StdReg::D => rr!(self.reg.d),
            StdReg::E => rr!(self.reg.e),
            StdReg::H => rr!(self.reg.h),
            StdReg::L => rr!(self.reg.l),
            StdReg::HL => {
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
            self.reg.set_c();
        } else {
            self.reg.unset_c();
        }

        self.pc = self.pc.wrapping_add(1);

        cycles // HL = 16
    }

    fn push_hl(&mut self) -> u8 {
        let cycles: u8 = 16;
        self.push(self.reg.h, self.reg.l);
        cycles
    }

    fn push(&mut self, lo: u8, hi: u8) {
        self.sp = self.sp.wrapping_sub(1);
        self.mem.write_byte(self.sp, lo).unwrap();
        self.sp = self.sp.wrapping_sub(1);
        self.mem.write_byte(self.sp, hi).unwrap();

        println!("\x1b[93mSP == {:#6X}\x1b[0m", self.sp);
    }

    fn pop(&mut self) -> u16 {
        println!("\x1b[93mSP == {:#6X}\x1b[0m", self.sp);
        let hi = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);
        let lo = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);

        ((hi as u16) << 8) | lo as u16
    }

    fn read_u8(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.mem.read_byte(self.pc).unwrap()
    }

    fn read_u16(&mut self) -> u16 {
        self.pc = self.pc.wrapping_add(1);
        let low = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);
        let high = self.mem.read_byte(self.pc).unwrap();
        self.reg.get_nn(low, high)
    }

    fn cp_a_n(&mut self) -> u8 {
        let cycles = 4;

        let n = self.read_u8();
        println!("{} - {}", self.reg.a, n);
        if (self.reg.a & 0xf).wrapping_sub(n & 0xf) & 0x10 == 0x10 {
            self.reg.set_h();
        }

        if self.reg.a.wrapping_sub(n) == 0 {
            self.reg.set_z();
        }

        if n > self.reg.a {
            self.reg.set_c();
        }

        self.reg.set_n();

        self.pc += self.pc.wrapping_add(1);

        cycles
    }

    fn ret(&mut self) -> u8 {
        let cycles = 16;
        self.pc = self.pop();
        cycles
    }

    fn ret_cc(&mut self, cond: FlagCond) -> u8 {
        let mut cycles = 8;

        if cond.check(self.reg.f) {
            cycles = 20;
            println!("RET {:?}", cond);
            self.pc = self.pop();
        } else {
            self.pc = self.pc.wrapping_add(1);
        }

        cycles
    }

    fn call_cc(&mut self, cond: FlagCond) -> u8 {
        let mut cycles = 12;
        if cond.check(self.reg.f) {
            cycles = 24;
            self.pc = self.pop();
        } else {
            self.pc = self.pc.wrapping_add(1);
        }
        cycles
    }

    fn call(&mut self) -> u8 {
        let cycles = 24;

        let jp = self.read_u16();

        let pos = self.pc.to_le_bytes();
        self.push(pos[0], pos[1]);

        self.pc = jp;

        cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nop() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let pc1 = cpu.pc;
        let cycles = cpu.nop();
        let pc2 = cpu.pc;
        assert_eq!(pc2, pc1 + 1);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn rrca() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.reg.a = 0b0000_0001;
        let cycles = cpu.rrca();

        assert_eq!(cpu.reg.a, 0b1000_0000);
        assert_eq!(cpu.reg.is_carry(), true);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn ret() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let lo = 0x50;
        let hi = 0x80;

        cpu.push(lo, hi);
        let cycles = cpu.ret();

        assert_eq!(cpu.pc, 0x8050);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn ld_a_n() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.reg.a = 0x00;
        cpu.reg.b = 0xFF;
        let cycles = cpu.ld_a_n(LoadReg::B);
        assert_eq!(cycles, 4);
        assert_eq!(cpu.reg.a, cpu.reg.b);

        cpu.reg.b = 0x80;
        cpu.reg.c = 0x80;
        cpu.mem.write_byte(0x8080, 0xFF).unwrap();
        cpu.reg.a = 0x00;
        let cycles = cpu.ld_a_n(LoadReg::MemBC);
        assert_eq!(cycles, 8);
        assert_eq!(cpu.reg.a, 0xFF);

        cpu.mem.write_byte(0x8080, 0xFF).unwrap();
        cpu.reg.a = 0x00;
        let cycles = cpu.ld_a_n(LoadReg::MemNN);
        assert_eq!(cycles, 16);
        assert_eq!(cpu.reg.a, 0x00);
    }

    #[test]
    fn dec_reg() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.reg.unset_z();
        cpu.reg.unset_n();
        cpu.reg.a = 0x01;
        assert_eq!(cpu.dec_reg(StdReg::A), 4);
        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.is_z(), true);
        assert_eq!(cpu.reg.is_n(), true);

        cpu.reg.a = 0b0001_0000;
        assert_eq!(cpu.dec_reg(StdReg::A), 4);
        assert_eq!(cpu.reg.is_h(), true);
    }

    #[test]
    fn ret_cc() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let lo = 0x50;
        let hi = 0x80;
        cpu.push(lo, hi);

        cpu.pc = 0x0000;
        cpu.reg.set_z();
        let cycles = cpu.ret_cc(FlagCond::NZ);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cycles, 8);

        cpu.pc = 0x0000;
        cpu.reg.unset_z();
        let cycles = cpu.ret_cc(FlagCond::NZ);
        assert_eq!(cpu.pc, 0x8050);
        assert_eq!(cycles, 20);

        let lo = 0x50;
        let hi = 0x80;
        cpu.push(lo, hi);

        cpu.pc = 0x0000;
        cpu.reg.unset_z();
        let cycles = cpu.ret_cc(FlagCond::Z);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cycles, 8);

        cpu.pc = 0x0000;
        cpu.reg.set_z();
        let cycles = cpu.ret_cc(FlagCond::Z);
        assert_eq!(cpu.pc, 0x8050);
        assert_eq!(cycles, 20);

        let lo = 0x50;
        let hi = 0x80;
        cpu.push(lo, hi);

        cpu.pc = 0x0000;
        cpu.reg.set_c();
        let cycles = cpu.ret_cc(FlagCond::NC);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cycles, 8);

        cpu.pc = 0x0000;
        cpu.reg.unset_c();
        let cycles = cpu.ret_cc(FlagCond::NC);
        assert_eq!(cpu.pc, 0x8050);
        assert_eq!(cycles, 20);

        let lo = 0x50;
        let hi = 0x80;
        cpu.push(lo, hi);

        cpu.pc = 0x0000;
        cpu.reg.unset_c();
        let cycles = cpu.ret_cc(FlagCond::C);
        assert_eq!(cpu.pc, 0x0001);
        assert_eq!(cycles, 8);

        cpu.pc = 0x0000;
        cpu.reg.set_c();
        let cycles = cpu.ret_cc(FlagCond::C);
        assert_eq!(cpu.pc, 0x8050);
        assert_eq!(cycles, 20);
    }

    #[test]
    fn pop_from_stack() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let lo = 0xFF;
        let hi = 0xDD;

        let val = ((hi as u16) << 8) | lo as u16;

        cpu.push(lo, hi);
        let pop = cpu.pop();

        assert_eq!(pop, val);
    }

    #[test]
    fn push_to_stack() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let pc1 = cpu.mem.read_byte(cpu.pc).unwrap();
        let pc2 = cpu.mem.read_byte(cpu.pc + 1).unwrap();

        cpu.push(pc1, pc2);

        let sp1 = cpu.mem.read_byte(cpu.sp - 1).unwrap();
        let sp2 = cpu.mem.read_byte(cpu.sp - 2).unwrap();

        assert_eq!(pc1, sp1);
        assert_eq!(pc2, sp2);
    }
}
