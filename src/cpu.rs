use crate::memorymap::MemoryMap;
use crate::registers::*;

pub struct CpuDataDebug {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub mem0: u8,
    pub mem1: u8,
    pub mem2: u8,
    pub mem3: u8,
}

pub struct Cpu<'m> {
    reg: Registers,
    sp: u16,
    pc: u16,
    mem: &'m MemoryMap,
}

impl<'m> Cpu<'m> {
    pub fn load(mem: &'m MemoryMap) -> Self {
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

    pub fn get_cpu_data_debug(&self) -> CpuDataDebug {
        CpuDataDebug {
            a: self.reg.a,
            f: self.reg.f,
            b: self.reg.b,
            c: self.reg.c,
            d: self.reg.d,
            e: self.reg.e,
            h: self.reg.h,
            l: self.reg.l,
            sp: self.sp,
            pc: self.pc,
            mem0: self.mem.read_byte(self.pc + 0).unwrap(),
            mem1: self.mem.read_byte(self.pc + 1).unwrap(),
            mem2: self.mem.read_byte(self.pc + 2).unwrap(),
            mem3: self.mem.read_byte(self.pc + 3).unwrap(),
        }
    }

    pub fn step(&mut self) -> u8 {
        /* let mut start = 0x8000;
        for i in 0..10 {
            self.mem.print_tile(start);
            start = start + (i * 16);
        } */
        let opcode = self.mem.read_byte(self.pc).unwrap();
        // println!(
        //     "C{} H{} N{} Z{}",
        //     self.reg.is_c(),
        //     self.reg.is_h(),
        //     self.reg.is_n(),
        //     self.reg.is_z()
        // );
        match opcode {
            0x00 => self.nop(),                 //tested
            0x0F => self.rrca(),                //tested
            0xC9 => self.ret(),                 //tested
            0xC0 => self.ret_cc(FlagCond::NZ),  //tested
            0xC8 => self.ret_cc(FlagCond::Z),   //tested
            0xD0 => self.ret_cc(FlagCond::NC),  //tested
            0xD8 => self.ret_cc(FlagCond::C),   //tested
            0xCD => self.call(),                //tested
            0xC4 => self.call_cc(FlagCond::NZ), //tested
            0xCC => self.call_cc(FlagCond::Z),  //tested
            0xD4 => self.call_cc(FlagCond::NC), //tested
            0xDC => self.call_cc(FlagCond::C),  //tested
            0xC1 => self.pop_bc(),
            0xD1 => self.pop_de(),
            0xE1 => self.pop_hl(),
            0xF1 => self.pop_af(),
            0xC5 => self.push_bc(),
            0xD5 => self.push_de(),
            0xE5 => self.push_hl(),
            0xF5 => self.push_af(),
            0x3D => self.dec_r(StdReg::A), //tested
            0x05 => self.dec_r(StdReg::B), //tested
            0x0D => self.dec_r(StdReg::C), //tested
            0x15 => self.dec_r(StdReg::D), //tested
            0x1D => self.dec_r(StdReg::E), //tested
            0x25 => self.dec_r(StdReg::H), //tested
            0x2D => self.dec_r(StdReg::L), //tested
            0xB8 => self.cp_a_r(StdReg::A),
            0xB9 => self.cp_a_r(StdReg::B),
            0xBA => self.cp_a_r(StdReg::C),
            0xBB => self.cp_a_r(StdReg::D),
            0xBC => self.cp_a_r(StdReg::E),
            0xBD => self.cp_a_r(StdReg::H),
            0xBF => self.cp_a_r(StdReg::L),
            0xFE => self.cp_a_n(),
            0x18 => self.jr(),
            0x20 => self.jr_cond(FlagCond::NZ),
            0x28 => self.jr_cond(FlagCond::Z),
            0x30 => self.jr_cond(FlagCond::NC),
            0x38 => self.jr_cond(FlagCond::C),
            0x01 => self.ld_n_nn(LoadRegnnn::BC), //tested
            0x11 => self.ld_n_nn(LoadRegnnn::DE), //tested
            0x21 => self.ld_n_nn(LoadRegnnn::HL), //tested
            0x31 => self.ld_n_nn(LoadRegnnn::SP), //tested
            0x7F => self.ld_a_n(LoadReg::A),      //tested
            0x78 => self.ld_a_n(LoadReg::B),      //tested
            0x79 => self.ld_a_n(LoadReg::C),      //tested
            0x7A => self.ld_a_n(LoadReg::D),      //tested
            0x7B => self.ld_a_n(LoadReg::E),      //tested
            0x7C => self.ld_a_n(LoadReg::H),      //tested
            0x7D => self.ld_a_n(LoadReg::L),      //tested
            0x0A => self.ld_a_n(LoadReg::MemBC),  //tested
            0x1A => self.ld_a_n(LoadReg::MemDE),  //tested
            0x7E => self.ld_a_n(LoadReg::MemHL),  //tested
            0xFA => self.ld_a_n(LoadReg::MemNN),  //tested
            0x3E => self.ld_a_n(LoadReg::N),      //tested
            0x47 => self.ld_n_a(LoadReg::B),      //tested
            0x4F => self.ld_n_a(LoadReg::C),      //tested
            0x57 => self.ld_n_a(LoadReg::D),      //tested
            0x5F => self.ld_n_a(LoadReg::E),      //tested
            0x67 => self.ld_n_a(LoadReg::H),      //tested
            0x6F => self.ld_n_a(LoadReg::L),      //tested
            0x02 => self.ld_n_a(LoadReg::MemBC),  //tested
            0x12 => self.ld_n_a(LoadReg::MemDE),  //tested
            0x77 => self.ld_n_a(LoadReg::MemHL),  //tested
            0xEA => self.ld_n_a(LoadReg::MemNN),  //tested
            0xE0 => self.ld_ff00_a(),
            0xF0 => self.ld_a_ff00(),
            0x40 => self.ld_r_r(StdReg::B, StdReg::B), //tested
            0x41 => self.ld_r_r(StdReg::B, StdReg::C), //tested
            0x42 => self.ld_r_r(StdReg::B, StdReg::D), //tested
            0x43 => self.ld_r_r(StdReg::B, StdReg::E), //tested
            0x44 => self.ld_r_r(StdReg::B, StdReg::H), //tested
            0x45 => self.ld_r_r(StdReg::B, StdReg::L), //tested
            0x46 => self.ld_r_r(StdReg::B, StdReg::HL), //tested
            0x48 => self.ld_r_r(StdReg::C, StdReg::B), //tested
            0x49 => self.ld_r_r(StdReg::C, StdReg::C), //tested
            0x4A => self.ld_r_r(StdReg::C, StdReg::D), //tested
            0x4B => self.ld_r_r(StdReg::C, StdReg::E), //tested
            0x4C => self.ld_r_r(StdReg::C, StdReg::H), //tested
            0x4D => self.ld_r_r(StdReg::C, StdReg::L), //tested
            0x4E => self.ld_r_r(StdReg::C, StdReg::HL), //tested
            0x50 => self.ld_r_r(StdReg::D, StdReg::B), //tested
            0x51 => self.ld_r_r(StdReg::D, StdReg::C), //tested
            0x52 => self.ld_r_r(StdReg::D, StdReg::D), //tested
            0x53 => self.ld_r_r(StdReg::D, StdReg::E), //tested
            0x54 => self.ld_r_r(StdReg::D, StdReg::H), //tested
            0x55 => self.ld_r_r(StdReg::D, StdReg::L), //tested
            0x56 => self.ld_r_r(StdReg::D, StdReg::HL), //tested
            0x58 => self.ld_r_r(StdReg::E, StdReg::B), //tested
            0x59 => self.ld_r_r(StdReg::E, StdReg::C), //tested
            0x5A => self.ld_r_r(StdReg::E, StdReg::D), //tested
            0x5B => self.ld_r_r(StdReg::E, StdReg::E), //tested
            0x5C => self.ld_r_r(StdReg::E, StdReg::H), //tested
            0x5D => self.ld_r_r(StdReg::E, StdReg::L), //tested
            0x5E => self.ld_r_r(StdReg::E, StdReg::HL), //tested
            0x60 => self.ld_r_r(StdReg::H, StdReg::B), //tested
            0x61 => self.ld_r_r(StdReg::H, StdReg::C), //tested
            0x62 => self.ld_r_r(StdReg::H, StdReg::D), //tested
            0x63 => self.ld_r_r(StdReg::H, StdReg::E), //tested
            0x64 => self.ld_r_r(StdReg::H, StdReg::H), //tested
            0x65 => self.ld_r_r(StdReg::H, StdReg::L), //tested
            0x66 => self.ld_r_r(StdReg::H, StdReg::HL), //tested
            0x68 => self.ld_r_r(StdReg::L, StdReg::B), //tested
            0x69 => self.ld_r_r(StdReg::L, StdReg::C), //tested
            0x6A => self.ld_r_r(StdReg::L, StdReg::D), //tested
            0x6B => self.ld_r_r(StdReg::L, StdReg::E), //tested
            0x6C => self.ld_r_r(StdReg::L, StdReg::H), //tested
            0x6D => self.ld_r_r(StdReg::L, StdReg::L), //tested
            0x6E => self.ld_r_r(StdReg::L, StdReg::HL), //tested
            0x70 => self.ld_r_r(StdReg::HL, StdReg::B), //tested
            0x71 => self.ld_r_r(StdReg::HL, StdReg::C), //tested
            0x72 => self.ld_r_r(StdReg::HL, StdReg::D), //tested
            0x73 => self.ld_r_r(StdReg::HL, StdReg::E), //tested
            0x74 => self.ld_r_r(StdReg::HL, StdReg::H), //tested
            0x75 => self.ld_r_r(StdReg::HL, StdReg::L), //tested
            0x36 => self.ld_r_r(StdReg::HL, StdReg::HL), //tested
            0x2A => self.ldi_a_memhl(),
            // 0x06 => self.ld_b_n(),
            0x0E => self.ld_r_n(StdReg::C),
            0x06 => self.ld_r_n(StdReg::B),
            // 0x0E => self.ld_r_n(StdReg::C),
            0x16 => self.ld_r_n(StdReg::D),
            0x1E => self.ld_r_n(StdReg::E),
            0x26 => self.ld_r_n(StdReg::H),
            0x2E => self.ld_r_n(StdReg::L),
            0x22 => self.ld_mem_hl_a_inc(), // tested (!SHEET)
            0x32 => self.ld_mem_hl_a_dec(), // tested (!SHEET)
            0xC3 => self.jp_nn(),
            0xB1 => self.or_a_r(StdReg::C),
            0xB2 => self.or_a_r(StdReg::D),
            0xB3 => self.or_a_r(StdReg::E),
            0xB4 => self.or_a_r(StdReg::H),
            0xB5 => self.or_a_r(StdReg::L),
            0xB7 => self.or_a_r(StdReg::A),
            0xAF => self.xor_r(StdReg::A),
            0xA8 => self.xor_r(StdReg::B),
            0xA9 => self.xor_r(StdReg::C),
            0xAA => self.xor_r(StdReg::D),
            0xAB => self.xor_r(StdReg::E),
            0xAC => self.xor_r(StdReg::H),
            0xAD => self.xor_r(StdReg::L),
            0xC7 => self.rst_00(),        //tested
            0xCF => self.rst_08(),        //tested
            0xD7 => self.rst_10(),        //tested
            0xDF => self.rst_18(),        //tested
            0xE7 => self.rst_20(),        //tested
            0xEF => self.rst_28(),        //tested
            0xF7 => self.rst_30(),        //tested
            0xFF => self.rst_38(),        //tested
            0x1F => self.rr_n(StdReg::A), //tested
            0x80 => self.add_a_r(StdReg::B),
            0x81 => self.add_a_r(StdReg::C),
            0x82 => self.add_a_r(StdReg::D),
            0x83 => self.add_a_r(StdReg::E),
            0x84 => self.add_a_r(StdReg::H),
            0x85 => self.add_a_r(StdReg::L),
            0x86 => self.add_a_r(StdReg::HL), //TODO
            0x87 => self.add_a_r(StdReg::A),
            0x8F => self.adc_a_n(StdRegN::A),
            0x88 => self.adc_a_n(StdRegN::B),
            0x89 => self.adc_a_n(StdRegN::C),
            0x8A => self.adc_a_n(StdRegN::D),
            0x8B => self.adc_a_n(StdRegN::E),
            0x8C => self.adc_a_n(StdRegN::H),
            0x8D => self.adc_a_n(StdRegN::L),
            0x8E => self.adc_a_n(StdRegN::HL),
            0xCE => self.adc_a_n(StdRegN::N),
            0x3C => self.inc_reg(IncDecReg::A), //tested
            0x04 => self.inc_reg(IncDecReg::B), //tested
            0x0C => self.inc_reg(IncDecReg::C), //tested
            0x14 => self.inc_reg(IncDecReg::D), //tested
            0x1C => self.inc_reg(IncDecReg::E), //tested
            0x24 => self.inc_reg(IncDecReg::H), //tested
            0x2C => self.inc_reg(IncDecReg::L), //tested
            0x23 => self.inc_reg(IncDecReg::HL),
            0x03 => self.inc_reg(IncDecReg::BC),
            0x13 => self.inc_reg(IncDecReg::DE),
            0x33 => self.inc_reg(IncDecReg::SP),
            0x34 => self.inc_reg(IncDecReg::MemHL),
            0xF3 => self.di(),
            0xFB => self.ei(),
            0xCB => {
                self.pc = self.pc.wrapping_add(1);
                let opcode = self.mem.read_byte(self.pc).unwrap();
                match opcode {
                    0x1F => self.rr_n(StdReg::A), //tested
                    0x18 => self.rr_n(StdReg::B), //tested
                    0x19 => self.rr_n(StdReg::C), //tested
                    0x1A => self.rr_n(StdReg::D), //tested
                    0x1B => self.rr_n(StdReg::E), //tested
                    0x1C => self.rr_n(StdReg::H), //tested
                    0x1D => self.rr_n(StdReg::L), //tested
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

    fn di(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.mem.write_byte(0xFFFF, 0x00).unwrap();
        4
    }

    fn ei(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.mem.write_byte(0xFFFF, 0xFF).unwrap();
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

    fn add_a_r(&mut self, reg: StdReg) -> u8 {
        let cycles = 4;

        macro_rules! add {
            ($a:expr,$b:expr) => {{
                if will_half_carry($a, $b) {
                    self.reg.set_h();
                };
                if will_carry($a, $b) {
                    self.reg.set_c();
                };
                $a = $a.wrapping_add($b);
            }};
        }

        match reg {
            StdReg::A => add!(self.reg.a, self.reg.a),
            StdReg::B => add!(self.reg.a, self.reg.b),
            StdReg::C => add!(self.reg.a, self.reg.c),
            StdReg::D => add!(self.reg.a, self.reg.d),
            StdReg::E => add!(self.reg.a, self.reg.e),
            StdReg::H => add!(self.reg.a, self.reg.h),
            StdReg::L => add!(self.reg.a, self.reg.l),
            StdReg::HL => todo!(),
        }

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn adc_a_n(&mut self, reg: StdRegN) -> u8 {
        let cycles = 4;

        let carry = self.reg.get_carry();

        macro_rules! adc {
            ($a:expr,$b:expr) => {{
                if will_half_carry($a, $b) {
                    self.reg.set_h();
                };
                if will_carry($a, $b) {
                    self.reg.set_c();
                };
                $a = $a.wrapping_add($b.wrapping_add(carry));
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

        self.pc = self.pc.wrapping_add(1);
        cycles
    }

    fn inc_reg(&mut self, reg: IncDecReg) -> u8 {
        let mut cycles = 4;

        macro_rules! inc {
            ($a:expr) => {{
                if will_half_carry($a, 1) {
                    self.reg.set_h();
                } else {
                    self.reg.unset_h();
                }
                $a = $a.wrapping_add(1);
                if $a == 0x00 {
                    self.reg.set_z();
                } else {
                    self.reg.unset_z();
                }
                self.reg.unset_n();
            }};
        }

        match reg {
            IncDecReg::A => inc!(self.reg.a),
            IncDecReg::B => inc!(self.reg.b),
            IncDecReg::C => inc!(self.reg.c),
            IncDecReg::D => inc!(self.reg.d),
            IncDecReg::E => inc!(self.reg.e),
            IncDecReg::H => inc!(self.reg.h),
            IncDecReg::L => inc!(self.reg.l),
            IncDecReg::HL => {
                cycles = 8;
                self.reg.set_hl(self.reg.get_hl().wrapping_add(1));
            }
            IncDecReg::BC => {
                cycles = 8;
                self.reg.set_bc(self.reg.get_bc().wrapping_add(1));
            }
            IncDecReg::DE => {
                cycles = 8;
                self.reg.set_de(self.reg.get_de().wrapping_add(1));
            }
            IncDecReg::SP => {
                cycles = 8;
                self.sp = self.sp.wrapping_add(1)
            }
            IncDecReg::MemHL => {
                cycles = 12;
                let loc = self.read_u16();
                let v: u8 = self.mem.read_byte(loc).unwrap();

                if will_half_carry(v, 1) {
                    self.reg.set_h();
                }
                self.mem.write_byte(loc, v.wrapping_add(1)).unwrap();
                if v.wrapping_add(1) == 0x00 {
                    self.reg.set_z();
                }
                self.reg.unset_n();
            }
        };

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn substract_u8_from_u8(&mut self, value: u8, amt: u8) -> u8 {
        let v = value.wrapping_sub(amt);

        if v == 0x00 {
            self.reg.set_z();
        } else {
            self.reg.unset_z();
        }

        if will_half_borrow(value, amt) {
            self.reg.set_h();
        } else {
            self.reg.unset_h();
        }

        if amt > value {
            self.reg.set_c();
        } else {
            self.reg.unset_c();
        }

        self.reg.set_n();

        v
    }

    fn dec_r(&mut self, reg: StdReg) -> u8 {
        let cycles = 4;
        macro_rules! dec {
            ($a:expr) => {{
                if will_half_borrow($a, 0x01) {
                    self.reg.set_h();
                } else {
                    self.reg.unset_h();
                }

                $a = $a.wrapping_sub(0x01);

                if $a == 0x00 {
                    self.reg.set_z();
                } else {
                    self.reg.unset_z();
                }

                self.reg.set_n();
            }};
        }
        match reg {
            StdReg::A => dec!(self.reg.a),
            StdReg::B => dec!(self.reg.b),
            StdReg::C => dec!(self.reg.c),
            StdReg::D => dec!(self.reg.d),
            StdReg::E => dec!(self.reg.e),
            StdReg::H => dec!(self.reg.h),
            StdReg::L => dec!(self.reg.l),
            StdReg::HL => todo!(),
        }

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn cp_a_r(&mut self, reg: StdReg) -> u8 {
        let cycles = 4;
        self.pc = self.pc.wrapping_add(1);
        match reg {
            StdReg::A => self.substract_u8_from_u8(self.reg.a, 0x01),
            StdReg::B => self.substract_u8_from_u8(self.reg.b, 0x01),
            StdReg::C => self.substract_u8_from_u8(self.reg.c, 0x01),
            StdReg::D => self.substract_u8_from_u8(self.reg.d, 0x01),
            StdReg::E => self.substract_u8_from_u8(self.reg.e, 0x01),
            StdReg::H => self.substract_u8_from_u8(self.reg.h, 0x01),
            StdReg::L => self.substract_u8_from_u8(self.reg.l, 0x01),
            StdReg::HL => todo!(),
        };
        cycles
    }

    fn cp_a_n(&mut self) -> u8 {
        let n = self.read_u8();
        self.substract_u8_from_u8(self.reg.a, n);

        4
    }

    fn jr(&mut self) -> u8 {
        let cycles = 12;
        let v = self.read_u8();

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
        let mut cycles: u8 = 8;
        if cond.check(self.reg.f) {
            cycles = self.jr();
        } else {
            self.pc = self.pc.wrapping_add(2);
        }

        cycles
    }

    fn jp_nn(&mut self) -> u8 {
        let loc = self.read_u16();
        self.pc = loc;
        16
    }

    fn or_a_r(&mut self, reg: StdReg) -> u8 {
        macro_rules! xor {
            ($a:expr) => {{
                self.reg.a = self.reg.a | $a;
                self.reg.unset_all_flags();
                if $a == 0x00 {
                    self.reg.set_z();
                }
            }};
        }

        match reg {
            StdReg::A => xor!(self.reg.a),
            StdReg::B => xor!(self.reg.b),
            StdReg::C => xor!(self.reg.c),
            StdReg::D => xor!(self.reg.d),
            StdReg::E => xor!(self.reg.e),
            StdReg::H => xor!(self.reg.h),
            StdReg::L => xor!(self.reg.l),
            StdReg::HL => todo!(),
        }

        self.pc = self.pc.wrapping_add(1);
        4
    }

    fn xor_r(&mut self, reg: StdReg) -> u8 {
        macro_rules! xor {
            ($a:expr) => {{
                self.reg.a = self.reg.a ^ $a;
                self.reg.unset_all_flags();
                if self.reg.a == 0x00 {
                    self.reg.set_z();
                }
            }};
        }

        match reg {
            StdReg::A => xor!(self.reg.a),
            StdReg::B => xor!(self.reg.b),
            StdReg::C => xor!(self.reg.c),
            StdReg::D => xor!(self.reg.d),
            StdReg::E => xor!(self.reg.e),
            StdReg::H => xor!(self.reg.h),
            StdReg::L => xor!(self.reg.l),
            StdReg::HL => todo!(),
        }

        self.pc = self.pc.wrapping_add(1);
        4
    }

    fn ld_ff00_a(&mut self) -> u8 {
        //Must be between $FF00 and $FFFF
        let cycles = 12;

        let b = 0xFF00 + self.read_u8() as u16;

        if (0xFF00..0xFFFF).contains(&b) {
            self.mem.write_byte(b, self.reg.a).unwrap();
        }

        cycles
    }

    fn ld_a_ff00(&mut self) -> u8 {
        //Must be between $FF00 and $FFFF
        let cycles = 12;

        let b = 0xFF00 + self.read_u8() as u16;

        if (0xFF00..0xFFFF).contains(&b) {
            self.reg.a = self.mem.read_byte(b).unwrap();
        }

        cycles
    }

    fn ld_r_r(&mut self, r1: StdReg, r2: StdReg) -> u8 {
        let mut cycles = 4;
        let _ret = match r1 {
            StdReg::A => match r2 {
                StdReg::A => self.reg.a = self.reg.a,
                StdReg::B => self.reg.a = self.reg.b,
                StdReg::C => self.reg.a = self.reg.c,
                StdReg::D => self.reg.a = self.reg.d,
                StdReg::E => self.reg.a = self.reg.e,
                StdReg::H => self.reg.a = self.reg.h,
                StdReg::L => self.reg.a = self.reg.l,
                StdReg::HL => {
                    cycles += 4;
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
                    cycles += 4;
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
                    cycles += 4;
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
                    cycles += 4;
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
                    cycles += 4;
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
                    cycles += 4;
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
                    cycles += 4;
                    self.reg.l = self.mem.read_byte(self.reg.get_hl()).unwrap();
                }
            },
            StdReg::HL => {
                cycles += 4;
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
        let hl = self.reg.get_hl();

        self.reg.a = self.mem.read_byte(hl).unwrap();
        self.reg.set_hl(hl.wrapping_add(1));

        self.pc = self.pc.wrapping_add(1);

        8
    }

    fn ld_mem_hl_a(&mut self) -> u8 {
        self.mem.write_byte(self.reg.get_hl(), self.reg.a).unwrap();
        self.pc = self.pc.wrapping_add(1);

        8
    }

    fn ld_mem_hl_a_dec(&mut self) -> u8 {
        let cycles = self.ld_mem_hl_a();
        self.reg.set_hl(self.reg.get_hl().overflowing_sub(1).0);

        cycles
    }

    fn ld_mem_hl_a_inc(&mut self) -> u8 {
        let cycles = self.ld_mem_hl_a();
        self.reg.set_hl(self.reg.get_hl().overflowing_add(1).0);

        cycles
    }

    fn ld_a_n(&mut self, reg: LoadReg) -> u8 {
        let mut cycles = 4;

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
                self.pc = self.pc.wrapping_sub(1);
            }
        }

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn ld_n_a(&mut self, reg: LoadReg) -> u8 {
        let mut cycles = 4;
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
                let v = self.read_u16();
                self.mem.write_byte(v, self.reg.a).unwrap();
                self.pc = self.pc.wrapping_sub(1);
            }
            LoadReg::N => (),
        };

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn ld_n_nn(&mut self, reg: LoadRegnnn) -> u8 {
        let cycles = 12;

        let nn = self.read_u16();

        match reg {
            LoadRegnnn::BC => self.reg.set_bc(nn),
            LoadRegnnn::DE => self.reg.set_de(nn),
            LoadRegnnn::HL => self.reg.set_hl(nn),
            LoadRegnnn::SP => self.sp = nn,
        }

        cycles
    }

    //TODO Make this do all the Regs
    fn ld_r_n(&mut self, reg: StdReg) -> u8 {
        macro_rules! ld {
            ($a:expr) => {{
                $a = self.read_u8();
            }};
        }

        match reg {
            StdReg::A => ld!(self.reg.a),
            StdReg::B => ld!(self.reg.b),
            StdReg::C => ld!(self.reg.c),
            StdReg::D => ld!(self.reg.d),
            StdReg::E => ld!(self.reg.e),
            StdReg::H => ld!(self.reg.h),
            StdReg::L => ld!(self.reg.l),
            StdReg::HL => todo!(),
        }

        8
    }

    fn rst(&mut self, pc: u16) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        let lo = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);
        let hi = self.mem.read_byte(self.pc).unwrap();
        self.push(lo, hi);
        self.pc = pc;
        16
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
        let mut cycles = 4;
        let c;

        macro_rules! rr {
            ($a:expr) => {{
                c = $a & 0x01;
                $a = $a.rotate_right(1);
                if self.reg.is_c() {
                    $a |= 0x80;
                } else {
                    $a &= 0x7F;
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
                cycles += 8;
                let mut val = self.mem.read_byte(self.reg.get_hl()).unwrap();
                c = val & 0x01;
                val = val.rotate_right(1);
                if self.reg.is_c() {
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

    fn pop(&mut self) -> u16 {
        let lo = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);
        let hi = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);

        ((hi as u16) << 8) | lo as u16
    }

    fn pop_bc(&mut self) -> u8 {
        let cycles: u8 = 12;
        self.reg.c = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);
        self.reg.b = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn pop_de(&mut self) -> u8 {
        let cycles: u8 = 12;
        self.reg.e = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);
        self.reg.d = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn pop_af(&mut self) -> u8 {
        let cycles: u8 = 12;
        self.reg.f = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);
        self.reg.a = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn pop_hl(&mut self) -> u8 {
        let cycles: u8 = 12;
        self.reg.l = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);
        self.reg.h = self.mem.read_byte(self.sp).unwrap();
        self.sp = self.sp.wrapping_add(1);

        self.pc = self.pc.wrapping_add(1);

        cycles
    }

    fn push(&mut self, lo: u8, hi: u8) {
        self.sp = self.sp.wrapping_sub(1);
        self.mem.write_byte(self.sp, lo).unwrap();
        self.sp = self.sp.wrapping_sub(1);
        self.mem.write_byte(self.sp, hi).unwrap();
    }

    fn push_rr(&mut self, lo: u8, hi: u8) -> u8 {
        let cycles: u8 = 16;
        self.push(lo, hi);
        self.pc = self.pc.wrapping_add(1);
        cycles
    }

    fn push_bc(&mut self) -> u8 {
        self.push_rr(self.reg.b, self.reg.c)
    }

    fn push_de(&mut self) -> u8 {
        self.push_rr(self.reg.d, self.reg.e)
    }

    fn push_hl(&mut self) -> u8 {
        self.push_rr(self.reg.h, self.reg.l)
    }

    fn push_af(&mut self) -> u8 {
        self.push_rr(self.reg.a, self.reg.f)
    }

    fn read_u8(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        let b = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);
        b
    }

    fn read_u16(&mut self) -> u16 {
        self.pc = self.pc.wrapping_add(1);
        let low = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);
        let high = self.mem.read_byte(self.pc).unwrap();
        self.pc = self.pc.wrapping_add(1);
        self.reg.get_nn(low, high)
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
            self.pc = self.pop();
        } else {
            self.pc = self.pc.wrapping_add(1);
        }

        cycles
    }

    fn call_cc(&mut self, cond: FlagCond) -> u8 {
        let mut cycles = 12;
        if cond.check(self.reg.f) {
            cycles = self.call();
        } else {
            self.pc = self.pc.wrapping_add(3);
        }
        cycles
    }

    fn call(&mut self) -> u8 {
        let cycles = 24;
        let jp = self.read_u16();
        let pos = self.pc.to_be_bytes();
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
        assert_eq!(cpu.reg.is_c(), true);
        assert_eq!(cycles, 4);
    }

    #[test]
    fn rr_n() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        macro_rules! rr {
            ($x:expr, $reg:expr) => {
                $x = 0b0000_0001;
                assert_eq!(cpu.rr_n($reg), 4);
                assert_eq!($x, 0b1000_0000);
                assert_eq!(cpu.reg.is_c(), true);
            };
        }
        rr!(cpu.reg.a, StdReg::A);
        rr!(cpu.reg.b, StdReg::B);
        rr!(cpu.reg.c, StdReg::C);
        rr!(cpu.reg.d, StdReg::D);
        rr!(cpu.reg.e, StdReg::E);
        rr!(cpu.reg.h, StdReg::H);
        rr!(cpu.reg.l, StdReg::L);
    }

    #[test]
    fn ret() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let lo = 0x50;
        let hi = 0x80;

        cpu.push(lo, hi);
        let cycles = cpu.ret();

        assert_eq!(cpu.pc, 0x5080);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn ld_a_n() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.pc = 0x8050;
        cpu.reg.a = 0x00;
        cpu.reg.b = 0xFF;
        let cycles = cpu.ld_a_n(LoadReg::B);
        assert_eq!(cycles, 4);
        assert_eq!(cpu.reg.a, cpu.reg.b);
        assert_eq!(cpu.pc, 0x8051);

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
    fn ld_n_nn() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.pc = 0x8200;
        cpu.mem.write_byte(cpu.pc + 1, 0xFF).unwrap();
        cpu.mem.write_byte(cpu.pc + 2, 0xAA).unwrap();
        assert_eq!(cpu.ld_n_nn(LoadRegnnn::BC), 3 * 4);
        assert_eq!(cpu.reg.get_bc(), 0xAAFF);
        cpu.pc = 0x8200;
        cpu.mem.write_byte(cpu.pc + 1, 0xFF).unwrap();
        cpu.mem.write_byte(cpu.pc + 2, 0xAA).unwrap();
        assert_eq!(cpu.ld_n_nn(LoadRegnnn::DE), 3 * 4);
        assert_eq!(cpu.reg.get_de(), 0xAAFF);
        cpu.pc = 0x8200;
        cpu.mem.write_byte(cpu.pc + 1, 0xFF).unwrap();
        cpu.mem.write_byte(cpu.pc + 2, 0xAA).unwrap();
        assert_eq!(cpu.ld_n_nn(LoadRegnnn::HL), 3 * 4);
        assert_eq!(cpu.reg.get_hl(), 0xAAFF);
        cpu.pc = 0x8200;
        cpu.mem.write_byte(cpu.pc + 1, 0xFF).unwrap();
        cpu.mem.write_byte(cpu.pc + 2, 0xAA).unwrap();
        assert_eq!(cpu.ld_n_nn(LoadRegnnn::SP), 3 * 4);
        assert_eq!(cpu.sp, 0xAAFF);
    }

    #[test]
    fn ld_n_a() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        macro_rules! ld {
            ($x:expr, $reg:expr) => {
                cpu.reg.a = 0xFF;
                $x = 0xDD;
                assert_eq!(cpu.ld_n_a($reg), 4);
                assert_eq!(cpu.reg.a, $x);
            };
        }

        ld!(cpu.reg.a, LoadReg::A);
        ld!(cpu.reg.a, LoadReg::B);
        ld!(cpu.reg.a, LoadReg::C);
        ld!(cpu.reg.a, LoadReg::D);
        ld!(cpu.reg.a, LoadReg::E);
        ld!(cpu.reg.a, LoadReg::H);
        ld!(cpu.reg.a, LoadReg::L);

        macro_rules! ldmem {
            ($reg:expr, $cycles:expr) => {
                cpu.reg.a = 0xAA;
                match $reg {
                    LoadReg::MemBC => cpu.reg.set_bc(0x8100),
                    LoadReg::MemDE => cpu.reg.set_de(0x8100),
                    LoadReg::MemHL => cpu.reg.set_hl(0x8100),
                    LoadReg::MemNN => {
                        cpu.pc = 0x8500;
                        cpu.mem.write_byte(cpu.pc + 1, 0x00).unwrap();
                        cpu.mem.write_byte(cpu.pc + 2, 0x81).unwrap();
                    }
                    _ => cpu.reg.a = 0xAA,
                }
                cpu.mem.write_byte(0x8100, 0xAA).unwrap();
                assert_eq!(cpu.ld_n_a($reg), $cycles);
                assert_eq!(cpu.mem.read_byte(0x8100).unwrap(), 0xAA);
            };
        }
        ldmem!(LoadReg::MemBC, 8);
        ldmem!(LoadReg::MemDE, 8);
        ldmem!(LoadReg::MemHL, 8);
        ldmem!(LoadReg::MemNN, 16);
    }

    #[test]
    fn ld_r_r() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);
        macro_rules! ld {
            ($x:expr, $y:expr, $reg1:expr, $reg2:expr) => {
                $x = 0xDD;
                $y = 0xFF;
                assert_eq!(cpu.ld_r_r($reg1, $reg2), 4);
                assert_eq!($x, $y);
            };
        }

        ld!(cpu.reg.b, cpu.reg.b, StdReg::B, StdReg::B);
        ld!(cpu.reg.b, cpu.reg.c, StdReg::B, StdReg::C);
        ld!(cpu.reg.b, cpu.reg.d, StdReg::B, StdReg::D);
        ld!(cpu.reg.b, cpu.reg.e, StdReg::B, StdReg::E);
        ld!(cpu.reg.b, cpu.reg.h, StdReg::B, StdReg::H);
        ld!(cpu.reg.b, cpu.reg.l, StdReg::B, StdReg::L);
        ld!(cpu.reg.c, cpu.reg.b, StdReg::C, StdReg::B);
        ld!(cpu.reg.c, cpu.reg.c, StdReg::C, StdReg::C);
        ld!(cpu.reg.c, cpu.reg.d, StdReg::C, StdReg::D);
        ld!(cpu.reg.c, cpu.reg.e, StdReg::C, StdReg::E);
        ld!(cpu.reg.c, cpu.reg.h, StdReg::C, StdReg::H);
        ld!(cpu.reg.c, cpu.reg.l, StdReg::C, StdReg::L);
        ld!(cpu.reg.d, cpu.reg.b, StdReg::D, StdReg::B);
        ld!(cpu.reg.d, cpu.reg.c, StdReg::D, StdReg::C);
        ld!(cpu.reg.d, cpu.reg.d, StdReg::D, StdReg::D);
        ld!(cpu.reg.d, cpu.reg.e, StdReg::D, StdReg::E);
        ld!(cpu.reg.d, cpu.reg.h, StdReg::D, StdReg::H);
        ld!(cpu.reg.d, cpu.reg.l, StdReg::D, StdReg::L);
        ld!(cpu.reg.e, cpu.reg.b, StdReg::E, StdReg::B);
        ld!(cpu.reg.e, cpu.reg.c, StdReg::E, StdReg::C);
        ld!(cpu.reg.e, cpu.reg.d, StdReg::E, StdReg::D);
        ld!(cpu.reg.e, cpu.reg.e, StdReg::E, StdReg::E);
        ld!(cpu.reg.e, cpu.reg.h, StdReg::E, StdReg::H);
        ld!(cpu.reg.e, cpu.reg.l, StdReg::E, StdReg::L);
        ld!(cpu.reg.h, cpu.reg.b, StdReg::H, StdReg::B);
        ld!(cpu.reg.h, cpu.reg.c, StdReg::H, StdReg::C);
        ld!(cpu.reg.h, cpu.reg.d, StdReg::H, StdReg::D);
        ld!(cpu.reg.h, cpu.reg.e, StdReg::H, StdReg::E);
        ld!(cpu.reg.h, cpu.reg.h, StdReg::H, StdReg::H);
        ld!(cpu.reg.h, cpu.reg.l, StdReg::H, StdReg::L);
        ld!(cpu.reg.l, cpu.reg.b, StdReg::L, StdReg::B);
        ld!(cpu.reg.l, cpu.reg.c, StdReg::L, StdReg::C);
        ld!(cpu.reg.l, cpu.reg.d, StdReg::L, StdReg::D);
        ld!(cpu.reg.l, cpu.reg.e, StdReg::L, StdReg::E);
        ld!(cpu.reg.l, cpu.reg.h, StdReg::L, StdReg::H);
        ld!(cpu.reg.l, cpu.reg.l, StdReg::L, StdReg::L);

        macro_rules! ld_r_hl {
            ($r:expr, $reg:expr) => {
                $r = 0xAA;
                cpu.reg.set_hl(0x8100);
                cpu.mem.write_byte(cpu.reg.get_hl(), 0xAA).unwrap();
                assert_eq!(cpu.ld_r_r($reg, StdReg::HL), 8);
                assert_eq!($r, 0xAA);
            };
        }

        ld_r_hl!(cpu.reg.b, StdReg::B);
        ld_r_hl!(cpu.reg.c, StdReg::C);
        ld_r_hl!(cpu.reg.d, StdReg::D);
        ld_r_hl!(cpu.reg.e, StdReg::E);
        ld_r_hl!(cpu.reg.l, StdReg::L);
        ld_r_hl!(cpu.reg.h, StdReg::H);

        macro_rules! ld_hl_r {
            ($r:expr, $reg:expr) => {
                $r = 0xAA;
                cpu.reg.set_hl(0x8100);
                assert_eq!(cpu.ld_r_r(StdReg::HL, $reg), 8);
                assert_eq!(cpu.mem.read_byte(0x8100).unwrap(), 0xAA);
            };
        }

        ld_hl_r!(cpu.reg.b, StdReg::B);
        ld_hl_r!(cpu.reg.c, StdReg::C);
        ld_hl_r!(cpu.reg.d, StdReg::D);
        ld_hl_r!(cpu.reg.e, StdReg::E);
    }

    #[test]
    fn dec_r() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        macro_rules! dec {
            ($r:expr, $reg:expr) => {
                cpu.reg.unset_z();
                cpu.reg.unset_h();
                cpu.reg.unset_n();
                $r = 20;
                assert_eq!(cpu.dec_r($reg), 4);
                assert_eq!($r, 19);
                assert_eq!(cpu.reg.is_z(), false);
                assert_eq!(cpu.reg.is_h(), false);
                assert_eq!(cpu.reg.is_n(), true);
            };
        }

        dec!(cpu.reg.a, StdReg::A);
        dec!(cpu.reg.b, StdReg::B);
        dec!(cpu.reg.c, StdReg::C);
        dec!(cpu.reg.d, StdReg::D);
        dec!(cpu.reg.e, StdReg::E);
        dec!(cpu.reg.h, StdReg::H);
        dec!(cpu.reg.l, StdReg::L);

        cpu.reg.unset_z();
        cpu.reg.unset_h();
        cpu.reg.set_n();
        cpu.reg.a = 0x01;
        assert_eq!(cpu.dec_r(StdReg::A), 4);
        assert_eq!(cpu.reg.a, 0);
        assert_eq!(cpu.reg.is_z(), true);
        assert_eq!(cpu.reg.is_h(), false);
        assert_eq!(cpu.reg.is_n(), true);
    }

    #[test]
    fn inc_reg() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        macro_rules! inc {
            ($r:expr, $reg:expr) => {
                cpu.reg.unset_z();
                cpu.reg.unset_h();
                cpu.reg.set_n();
                $r = 0b0000_1111;
                assert_eq!(cpu.inc_reg($reg), 4);
                assert_eq!($r, 16);
                assert_eq!(cpu.reg.is_z(), false);
                assert_eq!(cpu.reg.is_h(), true);
                assert_eq!(cpu.reg.is_n(), false);
            };
        }
        inc!(cpu.reg.a, IncDecReg::A);
        inc!(cpu.reg.b, IncDecReg::B);
        inc!(cpu.reg.c, IncDecReg::C);
        inc!(cpu.reg.d, IncDecReg::D);
        inc!(cpu.reg.e, IncDecReg::E);
        inc!(cpu.reg.h, IncDecReg::H);
        inc!(cpu.reg.l, IncDecReg::L);

        cpu.reg.unset_z();
        cpu.reg.unset_h();
        cpu.reg.set_n();
        cpu.reg.a = 0b1111_1111;
        assert_eq!(cpu.inc_reg(IncDecReg::A), 4);
        assert_eq!(cpu.reg.a, 0);
        assert_eq!(cpu.reg.is_z(), true);
        assert_eq!(cpu.reg.is_h(), true);
        assert_eq!(cpu.reg.is_n(), false);

        cpu.reg.set_bc(0x0FFF);
        assert_eq!(cpu.inc_reg(IncDecReg::BC), 8);
        assert_eq!(cpu.reg.get_bc(), 0x0FFF + 1);

        cpu.reg.set_hl(0x0FFF);
        assert_eq!(cpu.inc_reg(IncDecReg::HL), 8);
        assert_eq!(cpu.reg.get_hl(), 0x0FFF + 1);

        cpu.reg.set_de(0x0FFF);
        assert_eq!(cpu.inc_reg(IncDecReg::DE), 8);
        assert_eq!(cpu.reg.get_de(), 0x0FFF + 1);

        cpu.sp = 0x0FFF;
        assert_eq!(cpu.inc_reg(IncDecReg::SP), 8);
        assert_eq!(cpu.sp, 0x0FFF + 0x0001);
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
        assert_eq!(cpu.pc, 0x5080);
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
        assert_eq!(cpu.pc, 0x5080);
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
        assert_eq!(cpu.pc, 0x5080);
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
        assert_eq!(cpu.pc, 0x5080);
        assert_eq!(cycles, 20);
    }

    #[test]
    fn pop_from_stack() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let lo = 0xFF;
        let hi = 0xDD;

        let val = ((lo as u16) << 8) | hi as u16;

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

    #[test]
    fn rst() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        assert_eq!(cpu.rst_00(), 16);
        assert_eq!(cpu.pc, 0x0000);
        assert_eq!(cpu.rst_08(), 16);
        assert_eq!(cpu.pc, 0x0008);
        assert_eq!(cpu.rst_10(), 16);
        assert_eq!(cpu.pc, 0x0010);
        assert_eq!(cpu.rst_18(), 16);
        assert_eq!(cpu.pc, 0x0018);
        assert_eq!(cpu.rst_20(), 16);
        assert_eq!(cpu.pc, 0x0020);
        assert_eq!(cpu.rst_28(), 16);
        assert_eq!(cpu.pc, 0x0028);
        assert_eq!(cpu.rst_30(), 16);
        assert_eq!(cpu.pc, 0x0030);
        assert_eq!(cpu.rst_38(), 16);
        assert_eq!(cpu.pc, 0x0038);
    }

    #[test]
    fn call() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.pc = 0x8200;
        cpu.mem.write_byte(cpu.pc + 1, 0x88).unwrap();
        cpu.mem.write_byte(cpu.pc + 2, 0x99).unwrap();

        assert_eq!(cpu.call(), 24);
        assert_eq!(cpu.pc, 0x9988);
        assert_eq!(cpu.mem.read_byte(cpu.sp + 1).unwrap(), 0x82);
        assert_eq!(cpu.mem.read_byte(cpu.sp + 2).unwrap(), 0x00);
    }

    #[test]
    fn call_cc() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        macro_rules! call_success {
            ($reg:expr) => {
                cpu.pc = 0x8200;
                cpu.sp = 0xDFFE;
                cpu.mem.write_byte(cpu.pc + 1, 0x88).unwrap();
                cpu.mem.write_byte(cpu.pc + 2, 0x99).unwrap();

                assert_eq!(cpu.call_cc($reg), 24);
                assert_eq!(cpu.pc, 0x9988);
                assert_eq!(cpu.mem.read_byte(cpu.sp + 1).unwrap(), 0x82);
                assert_eq!(cpu.mem.read_byte(cpu.sp + 2).unwrap(), 0x00);
            };
        }

        cpu.reg.set_z();
        call_success!(FlagCond::Z);
        cpu.reg.unset_z();
        call_success!(FlagCond::NZ);
        cpu.reg.set_c();
        call_success!(FlagCond::C);
        cpu.reg.unset_c();
        call_success!(FlagCond::NC);

        macro_rules! call_fail {
            ($reg:expr) => {
                cpu.pc = 0x8200;
                cpu.mem.write_byte(cpu.pc + 1, 0x88).unwrap();
                cpu.mem.write_byte(cpu.pc + 2, 0x99).unwrap();

                cpu.reg.unset_z();
                assert_eq!(cpu.call_cc(FlagCond::Z), 12);
                assert_eq!(cpu.pc, 0x8200 + 3);
            };
        }

        cpu.reg.unset_z();
        call_fail!(FlagCond::Z);
        cpu.reg.set_z();
        call_fail!(FlagCond::NZ);
        cpu.reg.unset_c();
        call_fail!(FlagCond::C);
        cpu.reg.set_c();
        call_fail!(FlagCond::NC);
    }

    #[test]
    fn cp_a_n() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.reg.unset_h();
        cpu.reg.unset_n();
        cpu.reg.a = 0b0001_0000;
        cpu.pc = 0x8100;
        cpu.mem.write_byte(cpu.pc + 0x0001, 0b0000_0001).unwrap();

        assert_eq!(cpu.cp_a_n(), 4);
        assert_eq!(cpu.reg.is_h(), true);
        assert_eq!(cpu.reg.is_n(), true);

        cpu.reg.unset_z();
        cpu.reg.unset_n();
        cpu.reg.a = 0b0000_0001;
        cpu.pc = 0x8100;
        cpu.mem.write_byte(cpu.pc + 0x0001, 0b0000_0001).unwrap();

        assert_eq!(cpu.cp_a_n(), 4);
        assert_eq!(cpu.reg.is_z(), true);
        assert_eq!(cpu.reg.is_n(), true);

        cpu.reg.unset_c();
        cpu.reg.unset_n();
        cpu.reg.a = 0b1000_0000;
        cpu.pc = 0x8100;
        cpu.mem.write_byte(cpu.pc + 0x0001, 0b1100_0000).unwrap();

        assert_eq!(cpu.cp_a_n(), 4);
        assert_eq!(cpu.reg.is_c(), true);
        assert_eq!(cpu.reg.is_n(), true);
    }

    #[test]
    fn jr() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let relative_amt: i16 = -0x000A;
        let amt = relative_amt.to_le_bytes();
        let pc = 0x8200;

        cpu.pc = pc;
        cpu.mem.write_byte(cpu.pc + 1, amt[0]).unwrap();
        cpu.mem.write_byte(cpu.pc + 2, amt[1]).unwrap();

        assert_eq!(cpu.jr(), 4 * 3);
        println!("{} {}", cpu.pc, pc);
        assert_eq!(cpu.pc, pc - 0x000A + 2);
    }

    #[test]
    fn jr_cond() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        let relative_amt: i16 = -0x000A;
        let amt = relative_amt.to_le_bytes();
        let pc = 0x8200;

        cpu.pc = pc;
        cpu.mem.write_byte(cpu.pc + 1, amt[0]).unwrap();
        cpu.mem.write_byte(cpu.pc + 2, amt[1]).unwrap();

        cpu.reg.set_z();
        assert_eq!(cpu.jr_cond(FlagCond::NZ), 4 * 2);
        assert_eq!(cpu.pc, pc + 2);

        cpu.pc = pc;
        cpu.reg.unset_z();
        assert_eq!(cpu.jr_cond(FlagCond::NZ), 4 * 3);
        assert_eq!(cpu.pc, pc - 0x000A + 2);
    }

    #[test]
    fn ld_mem_hl_a() {
        let mut memmap = MemoryMap::default();
        let mut cpu = Cpu::load(&mut memmap);

        cpu.reg.set_hl(0x8200);
        cpu.reg.a = 0xEE;

        assert_eq!(cpu.ld_mem_hl_a_dec(), 4 * 2);
        assert_eq!(cpu.mem.read_byte(cpu.reg.get_hl() + 1).unwrap(), 0xEE);
        assert_eq!(cpu.reg.get_hl(), 0x8200 - 1);

        cpu.reg.set_hl(0x8200);
        cpu.reg.a = 0xEE;

        assert_eq!(cpu.ld_mem_hl_a_inc(), 4 * 2);
        assert_eq!(cpu.mem.read_byte(cpu.reg.get_hl() - 1).unwrap(), 0xEE);
        assert_eq!(cpu.reg.get_hl(), 0x8200 + 1);
    }
}
