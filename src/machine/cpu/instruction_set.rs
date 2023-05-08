use super::Reg16;
use super::Reg8;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Operand {
    /// 16-bit immediate value. nn
    Immediate16(u16),

    /// 8-bit immediate value. n
    Immediate8(u8),

    /// Indirect addressing through 16-bit register. (BC), (DE), (HL), (SP)
    Indirect8(Reg16),

    /// Indirect addressing through 16-bit immediate address. (nn)
    IndirectImmediate8(u16),

    /// 16-bit register. AF, BC, DE, HL, SP, PC
    R16(Reg16),

    /// 8-bit register. A, F, B, C, D, E, H, L
    R8(Reg8),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Flags {
    /// Not Zero
    NZ,

    /// Zero
    Z,

    /// Not Carry
    NC,

    /// Carry
    C,

    /// No flags
    Always,
}

/// This enumeration identifies each of the instructions that the CPU can execute
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    /// Load value into register
    LD { dest: Operand, src: Operand },

    /// Push 16-bit value onto stack
    PUSH { src: Reg16 },

    /// Pop 16-bit value from stack into register pair
    POP { dest: Reg16 },

    /// Add operand to accumulator
    ADD { dest: Operand, src: Operand },

    /// Add operand to accumulator with carry
    ADC { dest: Operand, src: Operand },

    /// Subtract operand from accumulator
    SUB { dest: Operand, src: Operand },

    /// Subtract with carry
    SBC { dest: Operand, src: Operand },

    /// AND operand with accumulator
    AND { dest: Operand, src: Operand },

    /// OR operand with accumulator
    OR { dest: Operand, src: Operand },

    /// Exclusive OR operand with accumulator
    XOR { dest: Operand, src: Operand },

    /// Compare operand with accumulator
    CP { dest: Operand, src: Operand },

    /// Increment operand
    INC { src: Operand },

    /// Decrement operand
    DEC { src: Operand },

    /// Swap upper and lower nibbles of operand
    SWAP { src: Operand },

    /// Decimal adjust accumulator
    DAA,

    /// Complement the accumulator
    CPL,

    /// Complement carry flag
    CCF,

    /// Set carry flag
    SCF,

    /// No operation
    NOP,

    /// Halt CPU
    HALT,

    /// Stop CPU and LCD display until button pressed
    STOP,

    /// Disable interrupts
    DI,

    /// Enable interrupts
    EI,

    /// Rotate accumulator left
    RLCA,

    /// Rotate accumulator left through carry
    RLA,

    /// Rotate accumulator right
    RRCA,

    /// Rotate accumulator right through carry
    RRA,

    /// Rotate operand left
    RLC { src: Operand },

    /// Rotate operand left through carry
    RL { src: Operand },

    /// Rotate operand right
    RRC { src: Operand },

    /// Rotate operand right through carry
    RR { src: Operand },

    /// Shift operand left into carry. LSB of n set to 0.
    SLA { src: Operand },

    /// Shift operand right into carry. MSB doesn't change.
    SRA { src: Operand },

    /// Shift operand right into carry. MSB set to 0.
    SRL { src: Operand },

    /// Test bit in operand
    BIT { bit: u8, src: Operand },

    /// Set bit in operand
    SET { bit: u8, src: Operand },

    /// Reset bit in operand
    RES { bit: u8, src: Operand },

    /// Jump to address
    JP { addr: Operand, cond: Flags },

    /// Jump to address relative to current address
    JR { cond: Flags, offset: i8 },

    /// Call subroutine at address
    CALL { addr: Operand, cond: Flags },

    /// Call subroutine at address
    RST { addr: u8 },

    /// Return from subroutine
    RET { cond: Flags },

    /// Return from subroutine and enable interrupts
    RETI,
}

// /// This enumeration identifies each of the instructions that the CPU can execute
// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub enum Instruction {
//     LD8Immediate { dest: Reg8, value: u8 },
//     LDMove8 { dest: Reg8, src: Reg8 },
//     LDLoad8 { dest: Reg8, addr: Reg16 },
//     LDStore8 { src: Reg8, addr: Reg16 },
//     LDStoreImmediateHL { value: u8 },
//     LDLoad8ImmediateAddr { dest: Reg8, addr: u16 },
//     LDStore8ImmediateAddr { addr: u16, src: Reg8 },
//     LDLoadHigh { dest: Reg8, addr: Reg8 },
//     LDStoreHigh { addr: Reg8, src: Reg8 },
//     LDDLoadDec { dest: Reg8, addr: Reg16 },
//     LDDStoreDec { addr: Reg16, src: Reg8 },
//     LDILoadInc { dest: Reg8, addr: Reg16 },
//     LDIStoreInc { addr: Reg16, src: Reg8 },
//     LDHStoreHighImmediate { addr: u8, src: Reg8 },
//     LDHLoadHighImmediate { dest: Reg8, addr: u8 },
//     LD16Immediate { dest: Reg16, value: u16 },
//     LDMove16 { dest: Reg16, src: Reg16 },
//     LDHLSP { offset: i8 },

//     LDLoad16ImmediateAddr { dest: Reg16, addr: u16 },
//     LDStore16ImmediateAddr { addr: u16, src: Reg16 },

//     PUSH{ src: Reg16 },
//     POP{ dest: Reg16 },

//     ADD8ToAccumulator{ src: Reg8 },
//     ADD8AtHLToAccumulator{},
//     ADD8Immediate{ value: u8 },

//     ADC8ToAccumulator{ src: Reg8 },
//     ADC8AtHLToAccumulator{},
//     ADC8Immediate{ value: u8 },

//     SUB8ToAccumulator{ src: Reg8 },
//     SUB8AtHLToAccumulator{},
//     SUB8Immediate{ value: u8 },

//     SBC8ToAccumulator{ src: Reg8 },
//     SBC8AtHLToAccumulator{},
//     SBC8Immediate{ value: u8 },

//     AND8ToAccumulator{ src: Reg8 },
//     AND8AtHLToAccumulator{},
//     AND8Immediate{ value: u8 },

//     OR8ToAccumulator{ src: Reg8 },
//     OR8AtHLToAccumulator{},
//     OR8Immediate{ value: u8 },

//     XOR8ToAccumulator{ src: Reg8 },
//     XOR8AtHLToAccumulator{},
//     XOR8Immediate{ value: u8 },

//     CP8ToAccumulator{ src: Reg8 },
//     CP8AtHLToAccumulator{},
//     CP8Immediate{ value: u8 },

//     INC{ src: Reg8 },
//     INCAtHL{},
// }
