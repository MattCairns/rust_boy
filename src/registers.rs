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
pub enum IncDecReg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    BC,
    DE,
    SP,
    MemHL,
}

#[derive(Debug)]
pub enum LoadReg {
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
    N,
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
    /// use rust_boy::registers::FlagCond;
    /// let flags : u8 = 0b11110000;
    /// let cond = FlagCond::NZ;
    /// assert_eq!(cond.check(flags), false);
    ///
    /// let cond = FlagCond::Z;
    /// assert_eq!(cond.check(flags), true);
    ///
    /// let cond = FlagCond::NC;
    /// assert_eq!(cond.check(flags), false);
    ///
    /// let cond = FlagCond::C;
    /// assert_eq!(cond.check(flags), true);
    /// ```
    pub fn check(&self, flags: u8) -> bool {
        match self {
            FlagCond::NZ => 0b00000000 == 0b10000000 & flags,
            FlagCond::Z => 0b10000000 == 0b10000000 & flags,
            FlagCond::NC => 0b00000000 == 0b00010000 & flags,
            FlagCond::C => 0b00010000 == 0b00010000 & flags,
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
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }

    pub fn set_af(&mut self, val: u16) {
        let bytes = val.to_be_bytes();
        self.a = bytes[0];
        self.f = bytes[1];
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

    pub fn is_z(&self) -> bool {
        self.f & 0b1000_0000 == 0b1000_0000
    }

    pub fn is_n(&self) -> bool {
        self.f & 0b0100_0000 == 0b0100_0000
    }

    pub fn is_h(&self) -> bool {
        self.f & 0b0010_0000 == 0b0010_0000
    }

    pub fn is_c(&self) -> bool {
        self.f & 0b0001_0000 == 0b0001_0000
    }

    pub fn get_carry(&self) -> u8 {
        if self.is_c() {
            0x01
        } else {
            0x00
        }
    }

    pub fn set_z(&mut self) {
        self.f = set_flag(self.f, 0);
    }

    pub fn unset_z(&mut self) {
        self.f = clear_flag(self.f, 0);
    }

    pub fn set_n(&mut self) {
        self.f = set_flag(self.f, 1);
    }

    pub fn unset_n(&mut self) {
        self.f = clear_flag(self.f, 1);
    }

    pub fn set_h(&mut self) {
        self.f = set_flag(self.f, 2);
    }

    pub fn unset_h(&mut self) {
        self.f = clear_flag(self.f, 2);
    }

    pub fn set_c(&mut self) {
        self.f = set_flag(self.f, 3);
    }

    pub fn unset_c(&mut self) {
        self.f = clear_flag(self.f, 3);
    }

    pub fn unset_all_flags(&mut self) {
        self.f = 0x00;
    }
}

/// Return the new value of the flag
/// with the bit at pos set
///
/// # Examples
///
/// ```
/// use rust_boy::registers::set_flag;
///
/// let flag = 0b00000000;
/// let pos = 0;
/// assert_eq!(set_flag(flag, pos), 0b10000000);
/// let pos = 1;
/// assert_eq!(set_flag(flag, pos), 0b01000000);
/// let pos = 2;
/// assert_eq!(set_flag(flag, pos), 0b00100000);
/// let pos = 3;
/// assert_eq!(set_flag(flag, pos), 0b00010000);
/// ```
pub fn set_flag(flag: u8, pos: u32) -> u8 {
    let byte: u8 = 0b1000_0000;
    flag | byte.rotate_right(pos)
}

/// .
///
/// # Examples
///
/// ```
/// use rust_boy::registers::clear_flag;
/// let flag = 0b00000001;
/// let pos = 7;
/// assert_eq!(clear_flag(flag, pos), 0b00000000);
/// ```
pub fn clear_flag(flag: u8, pos: u32) -> u8 {
    let byte: u8 = 0b1000_0000;
    flag & !byte.rotate_right(pos)
}

/// Returns true if the half carry bit will
/// be set when adding v1 and v2.
///
/// # Examples
///
/// ```
/// use rust_boy::registers::will_half_carry;
///
/// let v1 = 0b0000_1000;
/// let v2 = 0b0000_1000;
/// assert_eq!(will_half_carry(v1, v2), true);
///
/// let v1 = 0b0000_0000;
/// let v2 = 0b0000_0000;
/// assert_eq!(will_half_carry(v1, v2), false);
///
/// let v1 = 0b0000_1111;
/// let v2 = 0b0000_0001;
/// assert_eq!(will_half_carry(v1, v2), true);
/// ```
pub fn will_half_carry(v1: u8, v2: u8) -> bool {
    (v1 & 0xf).wrapping_add(v2 & 0xf) & 0x10 == 0x10
}

/// Returns true if the half carry bit will
/// be set when subtracting v1 and v2.
///
/// # Examples
///
/// ```
/// use rust_boy::registers::will_half_borrow;
///
/// let v1 = 0b00010000;
/// let v2 = 0b00001000;
/// assert_eq!(will_half_borrow(v1, v2), true);
///
/// let v1 = 0b00000000;
/// let v2 = 0b00000000;
/// assert_eq!(will_half_borrow(v1, v2), false);
/// ```
pub fn will_half_borrow(v1: u8, v2: u8) -> bool {
    (v1 & 0xf).wrapping_sub(v2 & 0xf) & 0x10 == 0x10
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
    ((v1 & 0x01) + (v2 & 0x01)) & 0x02 == 0x02
}

/// Returns true if the carry bit will be set
/// when subtracting v1 and v2.
///
/// # Examples
///
/// ```
/// use rust_boy::registers::will_borrow;
///
/// let v1 = 0b00000001;
/// let v2 = 0b00000010;
/// assert_eq!(will_borrow(v1, v2), true);
///
/// let v1 = 0b00000000;
/// let v2 = 0b00000000;
/// assert_eq!(will_borrow(v1, v2), false);
/// ```
pub fn will_borrow(v1: u8, v2: u8) -> bool {
   v1 < v2 
}

pub fn dec(value: u8, amt: u8) -> (u8, bool) {
    (value.wrapping_sub(amt), will_half_borrow(value, amt))
}
