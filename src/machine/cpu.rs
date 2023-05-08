mod decoder;
mod instruction_set;

pub use instruction_set::Instruction;
pub use instruction_set::Operand;
pub use instruction_set::Flags;

pub enum Interupts {
    VBlank,
    LcdcStatus,
    TimerOverflow,
    SerialTransferCompletion,
    HighToLowOfP10P13,
}

/// This struct represents the state of the CPU registers
/// 
/// | 15..8 |  7..0 |
/// | ----- | ----- |
/// |   A   |   F   |
/// |   B   |   C   |
/// |   D   |   E   |
/// |   H   |   L   |
/// |      SP       |
/// |      PC       |
#[derive(Default, Clone)]
struct RegisterFile {
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

/// This enum identifies each of the 8 bit registers
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Reg8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

/// This enum identifies all of the 16 bit registers
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl RegisterFile {
    fn get8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::F => self.f,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    fn set8(&mut self, reg: Reg8, value: u8) {
        match reg {
            Reg8::A => self.a = value,
            Reg8::F => self.f = value,
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value,
        }
    }

    fn get16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::AF => (self.a as u16) << 8 | self.f as u16,
            Reg16::BC => (self.b as u16) << 8 | self.c as u16,
            Reg16::DE => (self.d as u16) << 8 | self.e as u16,
            Reg16::HL => (self.h as u16) << 8 | self.l as u16,
            Reg16::SP => self.sp,
            Reg16::PC => self.pc,
        }
    }

    fn set16(&mut self, reg: Reg16, value: u16) {
        match reg {
            Reg16::AF => {
                self.a = (value >> 8) as u8;
                self.f = value as u8;
            }
            Reg16::BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8;
            }
            Reg16::DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8;
            }
            Reg16::HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8;
            }
            Reg16::SP => self.sp = value,
            Reg16::PC => self.pc = value,
        }
    }
}

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
