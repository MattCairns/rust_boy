pub enum DecRegisters {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

#[derive(Debug)]
pub enum AnyRegN {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    N,
}

#[derive(Debug)]
pub enum AnyReg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
}

pub enum JumpCond {
    NZ,
    Z,
    NC,
    C,
}

impl JumpCond {
    /// Check if the flags for the jump conditial meets
    /// the specified parameters..
    /// NZ, Jump if Z flag is reset.
    /// Z, Jump if Z flag is set.
    /// NC, Jump if C flag is reset.
    /// C, Jump if C flag is set.
    ///
    /// # Examples
    /// ```
    /// use rust_boy::registers::JumpCond;
    /// let flags : u8 = 0b00001111;
    /// let cond = JumpCond::NZ;
    /// assert_eq!(cond.check(flags), false);
    ///
    /// let cond = JumpCond::Z;
    /// assert_eq!(cond.check(flags), true);
    ///
    /// let cond = JumpCond::NC;
    /// assert_eq!(cond.check(flags), false);
    ///
    /// let cond = JumpCond::C;
    /// assert_eq!(cond.check(flags), true);
    /// ```
    pub fn check(&self, flags: u8) -> bool {
        match self {
            JumpCond::NZ => 0b00000000 == 0b00000001 & flags,
            JumpCond::Z => 0b00000001 == 0b00000001 & flags,
            JumpCond::NC => 0b00000000 == 0b00001000 & flags,
            JumpCond::C => 0b00001000 == 0b00001000 & flags,
        }
    }
}

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
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

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.h = bytes[0];
        self.l = bytes[1];
    }

    pub fn is_carry(&self) -> bool {
        if self.f & 0x08 == 0x08 {
            true
        } else {
            false
        }
    }

    pub fn set_zero_flag(&mut self) {
        self.f = set_flag(self.f, 7);
    }

    pub fn unset_zero_flag(&mut self) {
        self.f = clear_flag(self.f, 7);
    }

    pub fn set_sub_flag(&mut self) {
        self.f = set_flag(self.f, 6);
    }

    pub fn unset_sub_flag(&mut self) {
        self.f = clear_flag(self.f, 6);
    }

    pub fn set_half_carry_flag(&mut self) {
        self.f = set_flag(self.f, 5);
    }

    pub fn unset_half_carry_flag(&mut self) {
        self.f = clear_flag(self.f, 5);
    }

    pub fn set_carry_flag(&mut self) {
        self.f = set_flag(self.f, 4);
    }

    pub fn unset_carry_flag(&mut self) {
        self.f = clear_flag(self.f, 4);
    }

    pub fn unset_all_flags(&mut self) {
        self.f = 0x0000;
    }
}

fn set_flag(flag: u8, pos: u8) -> u8 {
    flag & !(1 << pos)
}

fn clear_flag(flag: u8, pos: u8) -> u8 {
    flag | (1 << pos)
}

pub fn dec(value: u8, amt: u8) -> (u8, bool) {
    if value == 0x00 {
        (0x00, false)
    } else {
        (
            value - amt,
            if ((value & 0xf) + (amt & 0xf)) & 0x10 == 0x10 {
                true
            } else {
                false
            },
        )
    }
}
