struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

enum Instruction {
    Add(AddRegister),
    LoadN(LoadNRegister),
}

enum AddRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum LoadNRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct Cpu {
    registers: Registers,
    stack_ptr: u16,
    program_ctr: u16,
}

/* impl Cpu {
    fn ld_nn_n(
} */
