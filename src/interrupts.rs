use crate::memorymap::MemoryMap;

#[derive(Default, Debug)]
pub struct Interrupt {
    vblank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl Interrupt {
    pub fn is_hot(&self, byte: &u8, mask: u8) -> bool {
        byte & mask == mask
    }

    pub fn decode(&mut self, byte: &u8) {
        let mask = 0b0000_0001;
        self.vblank = self.is_hot(byte, mask << 0);
        self.lcd_stat = self.is_hot(byte, mask << 1);
        self.timer = self.is_hot(byte, mask << 2);
        self.serial = self.is_hot(byte, mask << 3);
        self.joypad = self.is_hot(byte, mask << 4);
    }
}

pub struct InterruptHandler<'m> {
    mem: &'m MemoryMap,
    IE: Interrupt,
    IF: Interrupt,
}

impl<'m> InterruptHandler<'m> {
    pub fn new(mem: &'m MemoryMap) -> Self {
        Self {
            mem,
            IE: Interrupt::default(),
            IF: Interrupt::default(),
        }
    }

    pub fn service(&self) -> u16 {}

    pub fn update_ie(&mut self) {
        self.IE.decode(&self.mem.read_byte(0xFFFF).unwrap());
        println!("IE == {:?}", self.IE);
    }

    pub fn update_if(&mut self) {
        self.IF.decode(&self.mem.read_byte(0xFF0F).unwrap());
        println!("IF == {:?}", self.IF);
    }
}
