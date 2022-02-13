use crate::memorymap::MemoryMap;

struct SpriteFlags {
    bg_over_obj: bool,
    y_flip: bool,
    x_flip: bool,
    palette: bool,
}

impl SpriteFlags {
    pub fn new(flags: u8) -> Self {
        Self {
            bg_over_obj: true,
            y_flip: true,
            x_flip: true,
            palette: true,
        }
    }
}

struct SpriteAttribute {
    y_pos: u8,
    x_pos: u8,
    index: u8,
    flags: SpriteFlags,
}

pub struct Oam<'m> {
    mem: &'m MemoryMap,
    attrs: Vec<SpriteAttribute>,
}

impl<'m> Oam<'m> {
    pub fn new(mem: &'m MemoryMap) -> Self {
        Self {
            mem,
            attrs: Vec::new(),
        }
    }

    pub fn load(&mut self) {
        for x in (0xFE00..0xFE9F).step_by(4) {
            let y_pos = self.mem.read_byte(x + 0).unwrap();
            let x_pos = self.mem.read_byte(x + 1).unwrap();
            let index = self.mem.read_byte(x + 2).unwrap();
            let f = self.mem.read_byte(x + 3).unwrap();

            println!("Sprite index == {}", index);

            let flags = SpriteFlags::new(f);

            let attr = SpriteAttribute {
                y_pos,
                x_pos,
                index,
                flags,
            };
        }
    }
}
