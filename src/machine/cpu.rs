#[allow(dead_code)]
pub enum Interupts {
    VBlank,
    LcdcStatus,
    TimerOverflow,
    SerialTransferCompletion,
    HighToLowOfP10P13,
}

#[derive(Default)]
struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

enum InstructionDuration {
    C4 = 4,
    C8 = 8,
    C12 = 12,
}

pub struct Cpu {
    _registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            _registers: Default::default(),
        }
    }
}
