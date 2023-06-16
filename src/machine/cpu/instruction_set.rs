use super::Reg16;
use super::Reg8;

/// An input or output of and instruction working on an 8-bit value
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Operand8 {
    /// 8-bit immediate value. n
    Immediate8(u8),

    /// Indirect addressing through 16-bit register. (BC), (DE), (HL), (SP)
    Indirect8(Reg16),

    /// Indirect addressing through 16-bit register with post decrement. (HL-)
    Indirect8PostDec(Reg16),

    /// Indirect addressing through 16-bit register with post increment. (HL+)
    Indirect8PostInc(Reg16),

    /// Indirect addressing of high 256 bytes through 8-bit register. (0xFF00 + A), (0xFF00 + F), (0xFF00 + B), (0xFF00 + C), (0xFF00 + D), (0xFF00 + E), (0xFF00 + H), (0xFF00 + L)
    IndirectHigh8(Reg8),

    /// Indirect addressing of high 256 bytes through 8-bit immediate offset. (0xFF00 + n)
    IndirectImmediateHigh8(u8),

    /// Indirect addressing of 8-bit byte through 16-bit immediate address. (nn)
    IndirectImmediate8(u16),

    /// 8-bit register. A, F, B, C, D, E, H, L
    R8(Reg8),
}

/// An input or output of and instruction working on an 8-bit value
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Operand16 {
    /// 16-bit immediate value. nn
    Immediate16(u16),

    /// Indirect addressing through 16-bit register. (BC), (DE), (HL), (SP)
    Indirect16(Reg16),

    /// Indirect addressing of 16-bit byte through 16-bit immediate address. (nn)
    IndirectImmediate16(u16),

    /// 16-bit register. AF, BC, DE, HL, SP, PC
    R16(Reg16),
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
    /// Copies an 8-bit value into register or address
    LD8 { dest: Operand8, src: Operand8 },

    /// Load value into register
    LD16 { dest: Operand16, src: Operand16 },


    /// Load value into register
    LDHL { dest: Reg16, src: Reg16, n: i8 },

    /// Push 16-bit value onto stack
    PUSH { src: Reg16 },

    /// Pop 16-bit value from stack into register pair
    POP { dest: Reg16 },

    /// Add 8-bit value to accumulator
    ADD8 { rhs: Operand8 },

    /// Add 16-bit value to HL
    ADD16 { dest: Operand16, rhs: Operand16 },

    /// Add 8-bit value  to accumulator with carry
    ADC8 { rhs: Operand8 },

    /// Subtract 8-bit value to accumulator
    SUB8 { rhs: Operand8 },

    /// Subtract 8-bit value  to accumulator with carry
    SBC8 { rhs: Operand8 },

    /// Bitwise AND 8-bit value to accumulator
    AND8 { rhs: Operand8 },

    /// Bitwise OR 8-bit value to accumulator
    OR8 { rhs: Operand8 },

    /// Bitwise XOR 8-bit value to accumulator
    XOR8 { rhs: Operand8 },

    /// Compare 8-bit value to accumulator
    CP8 { rhs: Operand8 },

    /// Increment 8-bit operand
    INC8 { target: Operand8 },

    /// Increment 16-bit operand
    INC16 { target: Reg16 },

    /// Decrement 8-bit operand
    DEC8 { target: Operand8 },

    /// Decrement 15-bit operand
    DEC16 { target: Reg16 },

    /// Swap upper and lower nibbles of operand
    SWAP8 { target: Operand8 },

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
    RLC { target: Operand8 },

    /// Rotate operand left through carry
    RL { target: Operand8 },

    /// Rotate operand right
    RRC { target: Operand8 },

    /// Rotate operand right through carry
    RR { target: Operand8 },

    /// Shift operand left into carry. LSB of n set to 0.
    SLA { target: Operand8 },

    /// Shift operand right into carry. MSB doesn't change.
    SRA { target: Operand8 },

    /// Shift operand right into carry. MSB set to 0.
    SRL { target: Operand8 },

    /// Test bit in operand
    BIT { bit: u8, src: Operand8 },

    /// Set bit in operand
    SET { bit: u8, dest: Operand8 },

    /// Reset bit in operand
    RES { bit: u8, dest: Operand8 },

    /// Jump to address
    JP { addr: Operand16, cond: Flags },

    /// Jump to address relative to current address
    JR { cond: Flags, offset: i8 },

    /// Call subroutine at address
    CALL { addr: Operand16, cond: Flags },

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
