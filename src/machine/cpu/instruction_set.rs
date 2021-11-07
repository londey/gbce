use super::Reg16;
use super::Reg8;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    LD8Immediate { dest: Reg8, value: u8 },
    LDMove8 { dest: Reg8, src: Reg8 },
    LDLoad { dest: Reg8, addr: Reg16 },
    LDStore { src: Reg8, addr: Reg16 },
    LDStoreImmediateHL { value: u8 },
    LDLoadImmediateAddr { dest: Reg8, addr: u16 },
    LDStoreImmediateAddr { addr: u16, src: Reg8 },
    LDLoadHigh { dest: Reg8, addr: Reg8 },
    LDStoreHigh { addr: Reg8, src: Reg8 },
    LDDLoadDec { dest: Reg8, addr: Reg16 },
    LDDStoreDec { addr: Reg16, src: Reg8 },
    LDILoadInc { dest: Reg8, addr: Reg16 },
    LDIStoreInc { addr: Reg16, src: Reg8 },
    LDHStoreHighImmediate { addr: u8, src: Reg8 },
    LDHLoadHighImmediate { dest: Reg8, addr: u8 },
    LD16Immediate { dest: Reg16, value: u16 },
    LDMove16 { dest: Reg16, src: Reg16 },
}
