use super::Reg16;
use super::Reg8;

/// This enumeration identifies each of the instructions that the CPU can execute
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
    LDHLSP { offset: i8 },

    PUSH{ src: Reg16 },
    POP{ dest: Reg16 },

    ADD8ToAccumulator{ src: Reg8 },
    ADD8AtHLToAccumulator{},
    ADD8Immediate{ value: u8 },

    ADC8ToAccumulator{ src: Reg8 },
    ADC8AtHLToAccumulator{},
    ADC8Immediate{ value: u8 },

    SUB8ToAccumulator{ src: Reg8 },
    SUB8AtHLToAccumulator{},
    SUB8Immediate{ value: u8 },

    SBC8ToAccumulator{ src: Reg8 },
    SBC8AtHLToAccumulator{},
    SBC8Immediate{ value: u8 },

    AND8ToAccumulator{ src: Reg8 },
    AND8AtHLToAccumulator{},
    AND8Immediate{ value: u8 },

    OR8ToAccumulator{ src: Reg8 },
    OR8AtHLToAccumulator{},
    OR8Immediate{ value: u8 },

    XOR8ToAccumulator{ src: Reg8 },
    XOR8AtHLToAccumulator{},
    XOR8Immediate{ value: u8 },

    CP8ToAccumulator{ src: Reg8 },
    CP8AtHLToAccumulator{},
    CP8Immediate{ value: u8 },

    INC{ src: Reg8 },
    INCAtHL{},
}
