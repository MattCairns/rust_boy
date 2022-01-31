#[derive(Debug)]
pub enum StdRegN {
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
pub enum StdReg {
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
pub enum LoadRegnA {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    MemBC,
    MemDE,
    MemHL,
    MemNN,
}

#[derive(Debug)]
pub enum LoadRegnnn {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug)]
pub enum FlagCond {
    NZ,
    Z,
    NC,
    C,
}

impl FlagCond {
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
            FlagCond::NZ => 0b00000000 == 0b00000001 & flags,
            FlagCond::Z => 0b00000001 == 0b00000001 & flags,
            FlagCond::NC => 0b00000000 == 0b00001000 & flags,
            FlagCond::C => 0b00001000 == 0b00001000 & flags,
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

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.b = bytes[0];
        self.c = bytes[1];
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn set_de(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.d = bytes[0];
        self.e = bytes[1];
    }

    pub fn get_nn(&self, low: u8, high: u8) -> u16 {
        ((high as u16) << 8) | low as u16
    }

    pub fn is_carry(&self) -> bool {
        if self.f & 0x08 == 0x08 {
            true
        } else {
            false
        }
    }

    pub fn get_carry(&self) -> u8 {
        if self.is_carry() {
            0x01
        } else {
            0x00
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

/// Returns true if the half carry bit will
/// be set when adding v1 and v2.
///
/// # Examples
///
/// ```
/// use rust_boy::registers::will_half_carry;
///
/// let v1 = 0b00001000;
/// let v2 = 0b00001000;
/// assert_eq!(will_half_carry(v1, v2), true);
///
/// let v1 = 0b00000000;
/// let v2 = 0b00000000;
/// assert_eq!(will_half_carry(v1, v2), false);
/// ```
pub fn will_half_carry(v1: u8, v2: u8) -> bool {
    if ((v1 & 0xf) + (v2 & 0xf)) & 0x10 == 0x10 {
        true
    } else {
        false
    }
}

/// Returns true if the carry bit will be set
/// when adding v1 and v2.
///
/// # Examples
///
/// ```
/// use rust_boy::registers::will_carry;
///
/// let v1 = 0b00000001;
/// let v2 = 0b00000001;
/// assert_eq!(will_carry(v1, v2), true);
///
/// let v1 = 0b00000000;
/// let v2 = 0b00000000;
/// assert_eq!(will_carry(v1, v2), false);
/// ```
pub fn will_carry(v1: u8, v2: u8) -> bool {
    if ((v1 & 0xfe) + (v2 & 0xfe)) & 0x02 == 0x02 {
        true
    } else {
        false
    }
}

pub fn dec(value: u8, amt: u8) -> (u8, bool) {
    //BUG Does this half carry??
    if value == 0x00 {
        (0xFF, false)
    } else {
        (value - amt, will_half_carry(value, amt))
    }
}

pub fn inc(value: u8, amt: u8) -> (u8, bool) {
    if value == 0xFF {
        (0x00, false)
    } else {
        (value + amt, will_half_carry(value, amt))
    }
}
