use crate::cartridge::Cartridge;
use std::io;

pub struct MemoryMap {
    memory: [u8; 0xFFFF],
}

impl Default for MemoryMap {
    fn default() -> MemoryMap {
        MemoryMap {
            memory: [0; 0xFFFF],
        }
    }
}

impl MemoryMap {
    pub fn load_cartridge(&mut self, cartridge: &Cartridge) {
        (0..cartridge.data.len()).for_each(|pos| {
            self.memory[pos] = cartridge.data[pos];
        });
    }

    pub fn read_byte(&self, pos: u16) -> Result<u8, io::Error> {
        if pos > MemSectors::IE.val() {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot read from out of bounds memory.",
            ))
        } else {
            Ok(self.memory[pos as usize])
        }
    }

    pub fn write_byte(&mut self, pos: u16, byte: u8) -> Result<u8, io::Error> {
        if pos < MemSectors::RomBank1.val() {
            Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("Cannot write to ROM Banks at 0x{:X?}.", pos),
            ))
        } else {
            self.memory[pos as usize] = byte;
            Ok(byte)
        }
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
