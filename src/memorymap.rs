use crate::cartridge::Cartridge;
use crate::tile::Tile;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

pub struct MemoryMap {
    memory: Rc<RefCell<[u8; 0xFFFF + 1]>>,
}

impl Default for MemoryMap {
    fn default() -> MemoryMap {
        MemoryMap {
            memory: Rc::new(RefCell::new([0; 0xFFFF + 1])),
        }
    }
}

impl MemoryMap {
    pub fn load_cartridge(&self, cartridge: &Cartridge) {
        // println!("Cartridge size: {}", cartridge.data.len());
        (0..0xFFFF).for_each(|pos| {
            if pos >= cartridge.data.len() {
                self.memory.borrow_mut()[pos] = 0x00;
            } else {
                self.memory.borrow_mut()[pos] = cartridge.data[pos];
            };
        });
    }

    pub fn read_byte(&self, pos: u16) -> Result<u8, io::Error> {
        println!("READ BYTE {:X}", pos);
        if pos > MemSectors::IE.val() {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot read from out of bounds memory.",
            ))
        } else {
            // println!("read_byte --> {:X} :: {:X}", pos, self.memory.borrow_mut()[pos as usize]);
            Ok(self.memory.borrow_mut()[pos as usize])
        }
    }

    pub fn write_byte(&self, pos: u16, byte: u8) -> Result<u8, io::Error> {
        // println!("\x1b[92mWriting {:#4X?} to [{:#6X?}]\x1b[0m", byte, pos);
        self.memory.borrow_mut()[pos as usize] = byte;
        Ok(byte)
        /* if pos < MemSectors::RomBank1.val() {
            println!("\x1b[91mCan't {:#4X?} to [{:#6X?}]\x1b[0m", byte, pos);
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Cannot write to ROM Banks at 0x{:X?}.", pos),
            ))
        } else {
            println!("\x1b[92mWriting {:#4X?} to [{:#6X?}]\x1b[0m", byte, pos);
            self.memory.borrow_mut()[pos as usize] = byte;
            Ok(byte)
        } */
    }

    pub fn print_tile(&self, pos: u16) {
        let mut t: Vec<u8> = Vec::new();
        for i in 0..16 {
            t.push(self.read_byte(pos + i).unwrap())
        }
        println!("{}", Tile::new(t));
    }
}

pub enum MemSectors {
    RomBank0,
    RomBank1,
    VRam,
    ExtRam,
    WorkRam0,
    WorkRam1,
    EchoRam,
    OAM,
    Unused,
    IOReg,
    HRam,
    IE,
}

impl MemSectors {
    fn val(&self) -> u16 {
        match *self {
            MemSectors::RomBank0 => 0x0000,
            MemSectors::RomBank1 => 0x4000,
            MemSectors::VRam => 0x8000,
            MemSectors::ExtRam => 0xA000,
            MemSectors::WorkRam0 => 0xC000,
            MemSectors::WorkRam1 => 0xD000,
            MemSectors::EchoRam => 0xE000,
            MemSectors::OAM => 0xFE00,
            MemSectors::Unused => 0xFEA0,
            MemSectors::IOReg => 0xFF00,
            MemSectors::HRam => 0xFF80,
            MemSectors::IE => 0xFFFF,
        }
    }
}
