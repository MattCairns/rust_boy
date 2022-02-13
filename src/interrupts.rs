use crate::memorymap::MemoryMap;

/* #[derive(Default)]
pub struct Interrupts {
    ime: bool,
    ie: IE,
    if: IF,
} */

pub struct IF {
    vblank: bool,
    lcd_stat: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

pub struct Interrupt<'m> {
    mem: &'m MemoryMap,
    enabled: bool,
}

impl Interrupt {}
