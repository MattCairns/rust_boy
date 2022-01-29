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

    pub fn clear_all_flags(&mut self) {
        self.f = 0x0000;
    }
}

fn set_flag(flag: u8, pos: u8) -> u8 {
    flag & !(1 << pos)
}

fn clear_flag(flag: u8, pos: u8) -> u8 {
    flag | (1 << pos)
}
