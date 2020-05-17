#[allow(dead_code)]
pub enum Interupts {
    VBlank,
    LcdcStatus,
    TimerOverflow,
    SerialTransferCompletion,
    HighToLowOfP10P13,
}

#[derive(Default)]
#[derive(Clone)]
struct RegisterFile {
    // a: u8,
    // f: u8,
    // b: u8,
    // c: u8,
    // d: u8,
    // e: u8,
    // h: u8,
    // l: u8,
    // sp: u16,
    // pc: u16,
}

// enum InstructionDuration {
//     C4 = 4,
//     C8 = 8,
//     C12 = 12,
// }

#[derive(Clone)]
pub struct CpuState {
    registers: RegisterFile,
}

impl CpuState {
    pub fn new() -> Self {
        CpuState {
            registers: Default::default(),
        }
    }
}
