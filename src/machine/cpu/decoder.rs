
use crate::machine::cpu::Flags;

use super::Reg16;
use super::Reg8;
use super::instruction_set::Operand8::*;
use super::instruction_set::Operand16::*;
// use super::Flags;
// use super::Flags::*;



// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub struct Cycles(u8);

// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub struct PCAdvance(u8);

use super::Instruction;
use super::Instruction::*;


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct DecodedInstruction {
    /// Decoded instruction
    ins: Instruction,

    /// Cycles to execute
    cycles: u8,

    /// Instruction stream bytes consumed
    advance: u8,

    /// Flags register
    flags: Option<u8>,
}

#[derive(Debug)]
pub enum DecodeError {
    DecodeError,
    // Add more error types as needed.
}


#[rustfmt::skip]
pub fn decode_next_instruction(instruction_stream: &[u8]) -> Result<DecodedInstruction, DecodeError> {
    use DecodedInstruction as DI;
    use Reg8::*;
    use Reg16::*;

    match instruction_stream {

        // 3.3.1 8-Bit Loads
        // sec 3.3.1.1 Put value nn into n.
        [0x06, x, ..] => Ok(DI{ins: LD8{dest: R8(B), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None}),
        [0x0E, x, ..] => Ok(DI{ins: LD8{dest: R8(C), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None}),
        [0x16, x, ..] => Ok(DI{ins: LD8{dest: R8(D), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None}),
        [0x1E, x, ..] => Ok(DI{ins: LD8{dest: R8(E), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None}),
        [0x26, x, ..] => Ok(DI{ins: LD8{dest: R8(H), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None}),
        [0x2E, x, ..] => Ok(DI{ins: LD8{dest: R8(L), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None}),

        // sec 3.3.1.2 Put value r2 into r1.
        [0x7F, ..] => Ok(DI{ins: LD8{dest: R8(A), src: R8(A)}, cycles: 4, advance: 1, flags: None}),
        [0x78, ..] => Ok(DI{ins: LD8{dest: R8(A), src: R8(B)}, cycles: 4, advance: 1, flags: None}),
        [0x79, ..] => Ok(DI{ins: LD8{dest: R8(A), src: R8(C)}, cycles: 4, advance: 1, flags: None}),
        [0x7A, ..] => Ok(DI{ins: LD8{dest: R8(A), src: R8(D)}, cycles: 4, advance: 1, flags: None}),
        [0x7B, ..] => Ok(DI{ins: LD8{dest: R8(A), src: R8(E)}, cycles: 4, advance: 1, flags: None}),
        [0x7C, ..] => Ok(DI{ins: LD8{dest: R8(A), src: R8(H)}, cycles: 4, advance: 1, flags: None}),
        [0x7D, ..] => Ok(DI{ins: LD8{dest: R8(A), src: R8(L)}, cycles: 4, advance: 1, flags: None}),
        [0x7E, ..] => Ok(DI{ins: LD8{dest: R8(A), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),

        [0x40, ..] => Ok(DI{ins: LD8{dest: R8(B), src: R8(B)}, cycles: 4, advance: 1, flags: None}),
        [0x41, ..] => Ok(DI{ins: LD8{dest: R8(B), src: R8(C)}, cycles: 4, advance: 1, flags: None}),
        [0x42, ..] => Ok(DI{ins: LD8{dest: R8(B), src: R8(D)}, cycles: 4, advance: 1, flags: None}),
        [0x43, ..] => Ok(DI{ins: LD8{dest: R8(B), src: R8(E)}, cycles: 4, advance: 1, flags: None}),
        [0x44, ..] => Ok(DI{ins: LD8{dest: R8(B), src: R8(H)}, cycles: 4, advance: 1, flags: None}),
        [0x45, ..] => Ok(DI{ins: LD8{dest: R8(B), src: R8(L)}, cycles: 4, advance: 1, flags: None}),
        [0x46, ..] => Ok(DI{ins: LD8{dest: R8(B), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),

        [0x48, ..] => Ok(DI{ins: LD8{dest: R8(C), src: R8(B)}, cycles: 4, advance: 1, flags: None}),
        [0x49, ..] => Ok(DI{ins: LD8{dest: R8(C), src: R8(C)}, cycles: 4, advance: 1, flags: None}),
        [0x4A, ..] => Ok(DI{ins: LD8{dest: R8(C), src: R8(D)}, cycles: 4, advance: 1, flags: None}),
        [0x4B, ..] => Ok(DI{ins: LD8{dest: R8(C), src: R8(E)}, cycles: 4, advance: 1, flags: None}),
        [0x4C, ..] => Ok(DI{ins: LD8{dest: R8(C), src: R8(H)}, cycles: 4, advance: 1, flags: None}),
        [0x4D, ..] => Ok(DI{ins: LD8{dest: R8(C), src: R8(L)}, cycles: 4, advance: 1, flags: None}),
        [0x4E, ..] => Ok(DI{ins: LD8{dest: R8(C), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),

        [0x50, ..] => Ok(DI{ins: LD8{dest: R8(D), src: R8(B)}, cycles: 4, advance: 1, flags: None}),
        [0x51, ..] => Ok(DI{ins: LD8{dest: R8(D), src: R8(C)}, cycles: 4, advance: 1, flags: None}),
        [0x52, ..] => Ok(DI{ins: LD8{dest: R8(D), src: R8(D)}, cycles: 4, advance: 1, flags: None}),
        [0x53, ..] => Ok(DI{ins: LD8{dest: R8(D), src: R8(E)}, cycles: 4, advance: 1, flags: None}),
        [0x54, ..] => Ok(DI{ins: LD8{dest: R8(D), src: R8(H)}, cycles: 4, advance: 1, flags: None}),
        [0x55, ..] => Ok(DI{ins: LD8{dest: R8(D), src: R8(L)}, cycles: 4, advance: 1, flags: None}),
        [0x56, ..] => Ok(DI{ins: LD8{dest: R8(D), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),

        [0x58, ..] => Ok(DI{ins: LD8{dest: R8(E), src: R8(B)}, cycles: 4, advance: 1, flags: None}),
        [0x59, ..] => Ok(DI{ins: LD8{dest: R8(E), src: R8(C)}, cycles: 4, advance: 1, flags: None}),
        [0x5A, ..] => Ok(DI{ins: LD8{dest: R8(E), src: R8(D)}, cycles: 4, advance: 1, flags: None}),
        [0x5B, ..] => Ok(DI{ins: LD8{dest: R8(E), src: R8(E)}, cycles: 4, advance: 1, flags: None}),
        [0x5C, ..] => Ok(DI{ins: LD8{dest: R8(E), src: R8(H)}, cycles: 4, advance: 1, flags: None}),
        [0x5D, ..] => Ok(DI{ins: LD8{dest: R8(E), src: R8(L)}, cycles: 4, advance: 1, flags: None}),
        [0x5E, ..] => Ok(DI{ins: LD8{dest: R8(E), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),

        [0x60, ..] => Ok(DI{ins: LD8{dest: R8(H), src: R8(B)}, cycles: 4, advance: 1, flags: None}),
        [0x61, ..] => Ok(DI{ins: LD8{dest: R8(H), src: R8(C)}, cycles: 4, advance: 1, flags: None}),
        [0x62, ..] => Ok(DI{ins: LD8{dest: R8(H), src: R8(D)}, cycles: 4, advance: 1, flags: None}),
        [0x63, ..] => Ok(DI{ins: LD8{dest: R8(H), src: R8(E)}, cycles: 4, advance: 1, flags: None}),
        [0x64, ..] => Ok(DI{ins: LD8{dest: R8(H), src: R8(H)}, cycles: 4, advance: 1, flags: None}),
        [0x65, ..] => Ok(DI{ins: LD8{dest: R8(H), src: R8(L)}, cycles: 4, advance: 1, flags: None}),
        [0x66, ..] => Ok(DI{ins: LD8{dest: R8(H), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),

        [0x68, ..] => Ok(DI{ins: LD8{dest: R8(L), src: R8(B)}, cycles: 4, advance: 1, flags: None}),
        [0x69, ..] => Ok(DI{ins: LD8{dest: R8(L), src: R8(C)}, cycles: 4, advance: 1, flags: None}),
        [0x6A, ..] => Ok(DI{ins: LD8{dest: R8(L), src: R8(D)}, cycles: 4, advance: 1, flags: None}),
        [0x6B, ..] => Ok(DI{ins: LD8{dest: R8(L), src: R8(E)}, cycles: 4, advance: 1, flags: None}),
        [0x6C, ..] => Ok(DI{ins: LD8{dest: R8(L), src: R8(H)}, cycles: 4, advance: 1, flags: None}),
        [0x6D, ..] => Ok(DI{ins: LD8{dest: R8(L), src: R8(L)}, cycles: 4, advance: 1, flags: None}),
        [0x6E, ..] => Ok(DI{ins: LD8{dest: R8(L), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),

        [0x70, ..] => Ok(DI{ins: LD8{src: R8(B), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),
        [0x71, ..] => Ok(DI{ins: LD8{src: R8(C), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),
        [0x72, ..] => Ok(DI{ins: LD8{src: R8(D), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),
        [0x73, ..] => Ok(DI{ins: LD8{src: R8(E), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),
        [0x74, ..] => Ok(DI{ins: LD8{src: R8(H), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),
        [0x75, ..] => Ok(DI{ins: LD8{src: R8(L), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None}),
        [0x36, x, ..] => Ok(DI{ins: LD8{dest: Indirect8(HL), src: Immediate8(*x)}, cycles: 12, advance: 2, flags: None}),

        // sec 3.3.1.3 Put value n into A.
        // ignoring commands repeated from sec 3.3.1.2
        [0x0A, ..] => Ok(DI{ins: LD8{dest: R8(A), src: Indirect8(BC)}, cycles: 8, advance: 1, flags: None}),
        [0x1A, ..] => Ok(DI{ins: LD8{dest: R8(A), src: Indirect8(DE)}, cycles: 8, advance: 1, flags: None}),
        [0xFA, l, h, ..] => Ok(DI{ ins: LD8{ dest: R8(A), src: IndirectImmediate8(im16(h, l)) }, cycles: 16, advance: 3, flags: None}),
        [0x3E, x, ..] => Ok(DI{ins: LD8{dest: R8(A), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None}),

        // sec 3.3.1.4 Put value A into n.
        [0x47, ..] => Ok(DI{ins: LD8{dest: R8(B), src: R8(A)}, cycles: 4, advance: 1, flags: None}),
        [0x4F, ..] => Ok(DI{ins: LD8{dest: R8(C), src: R8(A)}, cycles: 4, advance: 1, flags: None}),
        [0x57, ..] => Ok(DI{ins: LD8{dest: R8(D), src: R8(A)}, cycles: 4, advance: 1, flags: None}),
        [0x5F, ..] => Ok(DI{ins: LD8{dest: R8(E), src: R8(A)}, cycles: 4, advance: 1, flags: None}),
        [0x67, ..] => Ok(DI{ins: LD8{dest: R8(H), src: R8(A)}, cycles: 4, advance: 1, flags: None}),
        [0x6F, ..] => Ok(DI{ins: LD8{dest: R8(L), src: R8(A)}, cycles: 4, advance: 1, flags: None}),
        [0x02, ..] => Ok(DI{ins: LD8{src: R8(A), dest: Indirect8(BC)}, cycles: 8, advance: 1, flags: None}),
        [0x12, ..] => Ok(DI{ins: LD8{src: R8(A), dest: Indirect8(DE)}, cycles: 8, advance: 1, flags: None}),
        [0xEA, l, h, ..] => Ok(DI{ins: LD8{dest: IndirectImmediate8(im16(h, l)),  src: R8(A) }, cycles: 16, advance: 3, flags: None}),

        // sec 3.3.1.5 Put value at address $FF00 + register C into A.
        [0xF2, ..] => Ok(DI{ins: LD8{dest: R8(A), src: IndirectHigh8(C)}, cycles: 8, advance: 1, flags: None}),

        // sec 3.3.1.6 Put A into address $FF00 + register C.
        [0xE2, ..] => Ok(DI{ins: LD8{src: R8(A), dest: IndirectHigh8(C)}, cycles: 8, advance: 1, flags: None}),

        // sec 3.3.1.7
        // N/A

        // sec 3.3.1.8
        // N/A

        // sec 3.3.1.9 Put value at address HL into A. Decrement HL.
        [0x3A, ..] => Ok(DI{ins: LD8 {dest: R8(A), src: Indirect8PostDec(HL)}, cycles: 8, advance: 1, flags: None}),

        // sec 3.3.1.10
        // N/A

        // sec 3.3.1.11
        // N/A

        // sec 3.3.1.12 Put A into memory address HL. Decrement HL.
        [0x32, ..] => Ok(DI{ins: LD8 {dest: Indirect8PostDec(HL), src: R8(A)}, cycles: 8, advance: 1, flags: None}),

        // sec 3.3.1.13
        // N/A

        // sec 3.3.1.14
        // N/A

        // sec 3.3.1.15 Put value at address HL into A. Increment HL.
        [0x2A, ..] => Ok(DI{ins: LD8 {dest: R8(A), src: Indirect8PostInc(HL)}, cycles: 8, advance: 1, flags: None}),

        // sec 3.3.1.16
        // N/A

        // sec 3.3.1.17
        // N/A

        // sec 3.3.1.18 Put A into memory address HL. Increment HL.
        [0x22, ..] => Ok(DI{ins: LD8 {dest: Indirect8PostInc(HL), src: R8(A)}, cycles: 8, advance: 1, flags: None}),

        // sec 3.3.1.19 Put A into memory address $FF00+n.
        [0xE0, x, ..] => Ok(DI{ins: LD8 {dest: IndirectImmediateHigh8(*x), src: R8(A), }, cycles: 12, advance: 2, flags: None}),

        // sec 3.3.1.20 Put memory address $FF00+n into A.
        [0xF0, x, ..] => Ok(DI{ins: LD8 {dest: R8(A), src: IndirectImmediateHigh8(*x)}, cycles: 12, advance: 2, flags: None}),

        // 3.3.2. 16-Bit Loads
        // sec 3.3.2.1 Put value nn into n.
        [0x01, l, h, ..] => Ok(DI{ins: LD16 {dest: R16(BC), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None}),
        [0x11, l, h, ..] => Ok(DI{ins: LD16 {dest: R16(DE), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None}),
        [0x21, l, h, ..] => Ok(DI{ins: LD16 {dest: R16(HL), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None}),
        [0x31, l, h, ..] => Ok(DI{ins: LD16 {dest: R16(SP), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None}),

        // sec 3.3.2.2 Put HL into Stack Pointer (SP).
        [0xF9, ..] => Ok(DI{ins: LD16 {dest: R16(SP), src: R16(HL)}, cycles: 8, advance: 1, flags: None}),

        // sec 3.3.2.3
        // N/A

        // sec 3.3.2.4 Put SP + n effective address into HL.
        [0xF8, _x, ..] => unimplemented!(), //DI{ins: LDHL {dest: R16(HL), lhs: R16(SP), rhs: *x as i8}, cycles: 12, advance: 2, flags: None},

        // sec 3.3.2.5 Put Stack Pointer (SP) at address n.
        [0x08, l, h, ..] => Ok(DI{ins: LD16 { src: R16(SP), dest: IndirectImmediate16(im16(h, l))}, cycles: 20, advance: 3, flags: None}),

        // sec 3.3.2.6 Push register pair nn onto stack. Decrement Stack Pointer (SP) twice.
        [0xF5, ..] => Ok(DI{ins: PUSH {src: AF}, cycles: 16, advance: 1, flags: None}),
        [0xC5, ..] => Ok(DI{ins: PUSH {src: BC}, cycles: 16, advance: 1, flags: None}),
        [0xD5, ..] => Ok(DI{ins: PUSH {src: DE}, cycles: 16, advance: 1, flags: None}),
        [0xE5, ..] => Ok(DI{ins: PUSH {src: HL}, cycles: 16, advance: 1, flags: None}),

        // sec 3.3.2.7 Pop two bytes off stack into register pair nn. Increment Stack Pointer (SP) twice.
        [0xF1, ..] => Ok(DI{ins: POP {dest: AF}, cycles: 12, advance: 1, flags: None}),
        [0xC1, ..] => Ok(DI{ins: POP {dest: BC}, cycles: 12, advance: 1, flags: None}),
        [0xD1, ..] => Ok(DI{ins: POP {dest: DE}, cycles: 12, advance: 1, flags: None}),
        [0xE1, ..] => Ok(DI{ins: POP {dest: HL}, cycles: 12, advance: 1, flags: None}),

        // 3.3.2. 16-Bit Loads
        // sec 3.3.3.1 Add n to A.
        [0x87, ..] => Ok(DI{ins: ADD8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0x80, ..] => Ok(DI{ins: ADD8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0x81, ..] => Ok(DI{ins: ADD8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0x82, ..] => Ok(DI{ins: ADD8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0x83, ..] => Ok(DI{ins: ADD8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0x84, ..] => Ok(DI{ins: ADD8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0x85, ..] => Ok(DI{ins: ADD8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0x86, ..] => Ok(DI{ins: ADD8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xC6, x, ..] => Ok(DI{ins: ADD8 { rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.2 
        [0x8F, ..] => Ok(DI{ins: ADC8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0x88, ..] => Ok(DI{ins: ADC8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0x89, ..] => Ok(DI{ins: ADC8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0x8A, ..] => Ok(DI{ins: ADC8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0x8B, ..] => Ok(DI{ins: ADC8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0x8C, ..] => Ok(DI{ins: ADC8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0x8D, ..] => Ok(DI{ins: ADC8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0x8E, ..] => Ok(DI{ins: ADC8{rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xCE, x, ..] => Ok(DI{ins: ADC8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.3 SUB n
        [0x97, ..] => Ok(DI{ins: SUB8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0x90, ..] => Ok(DI{ins: SUB8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0x91, ..] => Ok(DI{ins: SUB8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0x92, ..] => Ok(DI{ins: SUB8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0x93, ..] => Ok(DI{ins: SUB8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0x94, ..] => Ok(DI{ins: SUB8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0x95, ..] => Ok(DI{ins: SUB8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0x96, ..] => Ok(DI{ins: SUB8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xD6, x, ..] => Ok(DI{ins: SUB8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.4 SBC A,n
        [0x9F, ..] => Ok(DI{ins: SBC8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0x98, ..] => Ok(DI{ins: SBC8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0x99, ..] => Ok(DI{ins: SBC8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0x9A, ..] => Ok(DI{ins: SBC8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0x9B, ..] => Ok(DI{ins: SBC8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0x9C, ..] => Ok(DI{ins: SBC8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0x9D, ..] => Ok(DI{ins: SBC8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0x9E, ..] => Ok(DI{ins: SBC8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xDE, x, ..] => Ok(DI{ins: SBC8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.5 AND n
        [0xA7, ..] => Ok(DI{ins: AND8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0xA0, ..] => Ok(DI{ins: AND8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0xA1, ..] => Ok(DI{ins: AND8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0xA2, ..] => Ok(DI{ins: AND8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0xA3, ..] => Ok(DI{ins: AND8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0xA4, ..] => Ok(DI{ins: AND8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0xA5, ..] => Ok(DI{ins: AND8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0xA6, ..] => Ok(DI{ins: AND8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xE6, x, ..] => Ok(DI{ins: AND8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.6 OR n
        [0xB7, ..] => Ok(DI{ins: OR8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0xB0, ..] => Ok(DI{ins: OR8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0xB1, ..] => Ok(DI{ins: OR8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0xB2, ..] => Ok(DI{ins: OR8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0xB3, ..] => Ok(DI{ins: OR8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0xB4, ..] => Ok(DI{ins: OR8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0xB5, ..] => Ok(DI{ins: OR8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0xB6, ..] => Ok(DI{ins: OR8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xF6, x, ..] => Ok(DI{ins: OR8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.7 XOR n
        [0xAF, ..] => Ok(DI{ins: XOR8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0xA8, ..] => Ok(DI{ins: XOR8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0xA9, ..] => Ok(DI{ins: XOR8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0xAA, ..] => Ok(DI{ins: XOR8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0xAB, ..] => Ok(DI{ins: XOR8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0xAC, ..] => Ok(DI{ins: XOR8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0xAD, ..] => Ok(DI{ins: XOR8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0xAE, ..] => Ok(DI{ins: XOR8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xEE, x, ..] => Ok(DI{ins: XOR8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.8 CP n
        [0xBF, ..] => Ok(DI{ins: CP8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0xB8, ..] => Ok(DI{ins: CP8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0xB9, ..] => Ok(DI{ins: CP8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0xBA, ..] => Ok(DI{ins: CP8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0xBB, ..] => Ok(DI{ins: CP8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0xBC, ..] => Ok(DI{ins: CP8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0xBD, ..] => Ok(DI{ins: CP8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0xBE, ..] => Ok(DI{ins: CP8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }),
        [0xFE, x, ..] => Ok(DI{ins: CP8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.3.9 INC n
        [0x3C, ..] => Ok(DI{ins: INC8 {target: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0x04, ..] => Ok(DI{ins: INC8 {target: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0x0C, ..] => Ok(DI{ins: INC8 {target: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0x14, ..] => Ok(DI{ins: INC8 {target: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0x1C, ..] => Ok(DI{ins: INC8 {target: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0x24, ..] => Ok(DI{ins: INC8 {target: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0x2C, ..] => Ok(DI{ins: INC8 {target: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0x34, ..] => Ok(DI{ins: INC8 {target: Indirect8(HL)}, cycles: 12, advance: 1, flags: None }),

        // sec 3.3.3.10 DEC n
        [0x3D, ..] => Ok(DI{ins: DEC8 {target: R8(A)}, cycles: 4, advance: 1, flags: None }),
        [0x05, ..] => Ok(DI{ins: DEC8 {target: R8(B)}, cycles: 4, advance: 1, flags: None }),
        [0x0D, ..] => Ok(DI{ins: DEC8 {target: R8(C)}, cycles: 4, advance: 1, flags: None }),
        [0x15, ..] => Ok(DI{ins: DEC8 {target: R8(D)}, cycles: 4, advance: 1, flags: None }),
        [0x1D, ..] => Ok(DI{ins: DEC8 {target: R8(E)}, cycles: 4, advance: 1, flags: None }),
        [0x25, ..] => Ok(DI{ins: DEC8 {target: R8(H)}, cycles: 4, advance: 1, flags: None }),
        [0x2D, ..] => Ok(DI{ins: DEC8 {target: R8(L)}, cycles: 4, advance: 1, flags: None }),
        [0x35, ..] => Ok(DI{ins: DEC8 {target: Indirect8(HL)}, cycles: 12, advance: 1, flags: None }),

        // sec 3.3.4.1 ADD HL,n
        [0x09, ..] => Ok(DI{ins: ADD16 {dest: R16(HL), rhs: R16(BC)}, cycles: 8, advance: 1, flags: None }),
        [0x19, ..] => Ok(DI{ins: ADD16 {dest: R16(HL), rhs: R16(DE)}, cycles: 8, advance: 1, flags: None }),
        [0x29, ..] => Ok(DI{ins: ADD16 {dest: R16(HL), rhs: R16(HL)}, cycles: 8, advance: 1, flags: None }),
        [0x39, ..] => Ok(DI{ins: ADD16 {dest: R16(HL), rhs: R16(SP)}, cycles: 8, advance: 1, flags: None }),

        // sec 3.3.4.2 ADD SP, n
        [0xE8, x, ..] => Ok(DI{ins: ADD16 {dest: R16(SP), rhs: Immediate16(*x as i8 as i16 as u16)}, cycles: 16, advance: 2, flags: None }),
        
        // sec 3.3.4.3 INC nn
        [0x03, ..] => Ok(DI{ins: INC16 {target: BC}, cycles: 8, advance: 1, flags: None }),
        [0x13, ..] => Ok(DI{ins: INC16 {target: DE}, cycles: 8, advance: 1, flags: None }),
        [0x23, ..] => Ok(DI{ins: INC16 {target: HL}, cycles: 8, advance: 1, flags: None }),
        [0x33, ..] => Ok(DI{ins: INC16 {target: SP}, cycles: 8, advance: 1, flags: None }),

        // sec 3.3.4.4 DEC nn
        [0x0B, ..] => Ok(DI{ins: DEC16 {target: BC}, cycles: 8, advance: 1, flags: None }),
        [0x1B, ..] => Ok(DI{ins: DEC16 {target: DE}, cycles: 8, advance: 1, flags: None }),
        [0x2B, ..] => Ok(DI{ins: DEC16 {target: HL}, cycles: 8, advance: 1, flags: None }),
        [0x3B, ..] => Ok(DI{ins: DEC16 {target: SP}, cycles: 8, advance: 1, flags: None }),

        // sec 3.3.5.1 SWAP n
        [0xCB, 0x37, ..] => Ok(DI{ins: SWAP8 {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        [0xCB, 0x30, ..] => Ok(DI{ins: SWAP8 {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        [0xCB, 0x31, ..] => Ok(DI{ins: SWAP8 {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        [0xCB, 0x32, ..] => Ok(DI{ins: SWAP8 {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        [0xCB, 0x33, ..] => Ok(DI{ins: SWAP8 {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        [0xCB, 0x34, ..] => Ok(DI{ins: SWAP8 {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        [0xCB, 0x35, ..] => Ok(DI{ins: SWAP8 {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        [0xCB, 0x36, ..] => Ok(DI{ins: SWAP8 {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.5.2 DDA
        [0x27, ..] => Ok(DI{ins: DAA, cycles: 4, advance: 1, flags: None }),
        
        // sec 3.3.5.3 CPL
        [0x2F, ..] => Ok(DI{ins: CPL, cycles: 4, advance: 1, flags: None }),
        
        // sec 3.3.5.4 CCF
        [0x3F, ..] => Ok(DI{ins: CCF, cycles: 4, advance: 1, flags: None }),
        
        // sec 3.3.5.5 SCF
        [0x37, ..] => Ok(DI{ins: SCF, cycles: 4, advance: 1, flags: None }),

        // 3.3.5.6 NOP
        [0x00, ..] => Ok(DI{ins: NOP, cycles: 4, advance: 1, flags: None }),

        // 3.3.5.7 HALT
        [0x76, ..] => Ok(DI{ins: HALT, cycles: 4, advance: 1, flags: None }),

        // 3.3.5.8 STOP
        [0x10, 0x00, ..] => Ok(DI{ins: STOP, cycles: 4, advance: 2, flags: None }),

        // 3.3.5.9 DI
        [0xF3, ..] => Ok(DI{ins: DI, cycles: 4, advance: 1, flags: None }),

        // 3.3.5.10 EI
        [0xFB, ..] => Ok(DI{ins: EI, cycles: 4, advance: 1, flags: None }),

        // sec 3.3.6.1 RLCA
        [0x07, ..] => Ok(DI{ins: RLCA, cycles: 4, advance: 1, flags: None }),

        // sec 3.3.6.2 RLA
        [0x17, ..] => Ok(DI{ins: RLA, cycles: 4, advance: 1, flags: None }),

        // sec 3.3.6.3 RRCA
        [0x0F, ..] => Ok(DI{ins: RRCA, cycles: 4, advance: 1, flags: None }),

        // sec 3.3.6.4 RRA
        [0x1F, ..] => Ok(DI{ins: RRA, cycles: 4, advance: 1, flags: None }),

        // handle CB instructions
        [0xCB, x, ..] => decode_cb_instruction(*x).map_err(|_| DecodeError::DecodeError),

        // sec 3.3.8.1 JP nn
        [0xC3, l, h, ..] => Ok(DI{ins: JP {addr: Immediate16(im16(h, l)), cond: Flags::Always}, cycles: 16, advance: 3, flags: None }),

        // sec 3.3.8.2 JP cc,nn
        [0xC2, l, h, ..] => Ok(DI{ins: JP {addr: Immediate16(im16(h, l)), cond: Flags::NZ}, cycles: 16, advance: 3, flags: None }),
        [0xCA, l, h, ..] => Ok(DI{ins: JP {addr: Immediate16(im16(h, l)), cond: Flags::Z}, cycles: 16, advance: 3, flags: None }),
        [0xD2, l, h, ..] => Ok(DI{ins: JP {addr: Immediate16(im16(h, l)), cond: Flags::NC}, cycles: 16, advance: 3, flags: None }),
        [0xDA, l, h, ..] => Ok(DI{ins: JP {addr: Immediate16(im16(h, l)), cond: Flags::C}, cycles: 16, advance: 3, flags: None }),

        // sec 3.3.8.3 JP (HL)
        [0xE9, ..] => Ok(DI{ins: JP {addr: Indirect16(HL), cond: Flags::Always}, cycles: 4, advance: 1, flags: None }),

        // sec 3.3.8.4 JR n
        [0x18, x, ..] => Ok(DI{ins: JR {offset: *x as i8, cond: Flags::Always}, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.8.5 JR cc,n
        [0x20, x, ..] => Ok(DI{ins: JR {offset: *x as i8, cond: Flags::NZ}, cycles: 8, advance: 2, flags: None }),
        [0x28, x, ..] => Ok(DI{ins: JR {offset: *x as i8, cond: Flags::Z}, cycles: 8, advance: 2, flags: None }),
        [0x30, x, ..] => Ok(DI{ins: JR {offset: *x as i8, cond: Flags::NC}, cycles: 8, advance: 2, flags: None }),
        [0x38, x, ..] => Ok(DI{ins: JR {offset: *x as i8, cond: Flags::C}, cycles: 8, advance: 2, flags: None }),

        // sec 3.3.9.1 CALL nn
        [0xCD, l, h, ..] => Ok(DI{ins: CALL {addr: Immediate16(im16(h, l)), cond: Flags::Always}, cycles: 12, advance: 3, flags: None }),

        // sec 3.3.9.2 CALL cc,nn
        [0xC4, l, h, ..] => Ok(DI{ins: CALL {addr: Immediate16(im16(h, l)), cond: Flags::NZ}, cycles: 12, advance: 3, flags: None }),
        [0xCC, l, h, ..] => Ok(DI{ins: CALL {addr: Immediate16(im16(h, l)), cond: Flags::Z}, cycles: 12, advance: 3, flags: None }),
        [0xD4, l, h, ..] => Ok(DI{ins: CALL {addr: Immediate16(im16(h, l)), cond: Flags::NC}, cycles: 12, advance: 3, flags: None }),
        [0xDC, l, h, ..] => Ok(DI{ins: CALL {addr: Immediate16(im16(h, l)), cond: Flags::C}, cycles: 12, advance: 3, flags: None }),

        // sec 3.3.10.1 RST n
        [0xC7, ..] => Ok(DI{ins: RST {addr: 0x00}, cycles: 32, advance: 1, flags: None }),
        [0xCF, ..] => Ok(DI{ins: RST {addr: 0x08}, cycles: 32, advance: 1, flags: None }),
        [0xD7, ..] => Ok(DI{ins: RST {addr: 0x10}, cycles: 32, advance: 1, flags: None }),
        [0xDF, ..] => Ok(DI{ins: RST {addr: 0x18}, cycles: 32, advance: 1, flags: None }),
        [0xE7, ..] => Ok(DI{ins: RST {addr: 0x20}, cycles: 32, advance: 1, flags: None }),
        [0xEF, ..] => Ok(DI{ins: RST {addr: 0x28}, cycles: 32, advance: 1, flags: None }),
        [0xF7, ..] => Ok(DI{ins: RST {addr: 0x30}, cycles: 32, advance: 1, flags: None }),
        [0xFF, ..] => Ok(DI{ins: RST {addr: 0x38}, cycles: 32, advance: 1, flags: None }),

        // sec 3.3.11.1 RET
        [0xC9, ..] => Ok(DI{ins: RET {cond: Flags::Always}, cycles: 8, advance: 1, flags: None }),

        // sec RET cc
        [0xC0, ..] => Ok(DI{ins: RET {cond: Flags::NZ}, cycles: 8, advance: 1, flags: None }),
        [0xC8, ..] => Ok(DI{ins: RET {cond: Flags::Z}, cycles: 8, advance: 1, flags: None }),
        [0xD0, ..] => Ok(DI{ins: RET {cond: Flags::NC}, cycles: 8, advance: 1, flags: None }),
        [0xD8, ..] => Ok(DI{ins: RET {cond: Flags::C}, cycles: 8, advance: 1, flags: None }),

        // sec RETI
        [0xD9, ..] => Ok(DI{ins: RETI, cycles: 8, advance: 1, flags: None }),

        _ => Err(DecodeError::DecodeError),
    }
}

fn decode_cb_instruction(parameter: u8) -> Result<DecodedInstruction, ()> {
    use DecodedInstruction as DI;
    use Reg8::*;
    use Reg16::HL;

    match parameter {
        // sec 3.3.6.5 RLC n
        0x07 => Ok(DI{ins: RLC {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        0x00 => Ok(DI{ins: RLC {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        0x01 => Ok(DI{ins: RLC {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        0x02 => Ok(DI{ins: RLC {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        0x03 => Ok(DI{ins: RLC {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        0x04 => Ok(DI{ins: RLC {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        0x05 => Ok(DI{ins: RLC {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        0x06 => Ok(DI{ins: RLC {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.6.6 RL n
        0x17 => Ok(DI{ins: RL {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        0x10 => Ok(DI{ins: RL {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        0x11 => Ok(DI{ins: RL {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        0x12 => Ok(DI{ins: RL {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        0x13 => Ok(DI{ins: RL {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        0x14 => Ok(DI{ins: RL {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        0x15 => Ok(DI{ins: RL {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        0x16 => Ok(DI{ins: RL {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.6.7 RRC n
        0x0F => Ok(DI{ins: RRC {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        0x08 => Ok(DI{ins: RRC {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        0x09 => Ok(DI{ins: RRC {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        0x0A => Ok(DI{ins: RRC {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        0x0B => Ok(DI{ins: RRC {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        0x0C => Ok(DI{ins: RRC {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        0x0D => Ok(DI{ins: RRC {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        0x0E => Ok(DI{ins: RRC {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.6.8 RR n
        0x1F => Ok(DI{ins: RR {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        0x18 => Ok(DI{ins: RR {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        0x19 => Ok(DI{ins: RR {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        0x1A => Ok(DI{ins: RR {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        0x1B => Ok(DI{ins: RR {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        0x1C => Ok(DI{ins: RR {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        0x1D => Ok(DI{ins: RR {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        0x1E => Ok(DI{ins: RR {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.6.9 SLA n
        0x27 => Ok(DI{ins: SLA {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        0x20 => Ok(DI{ins: SLA {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        0x21 => Ok(DI{ins: SLA {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        0x22 => Ok(DI{ins: SLA {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        0x23 => Ok(DI{ins: SLA {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        0x24 => Ok(DI{ins: SLA {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        0x25 => Ok(DI{ins: SLA {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        0x26 => Ok(DI{ins: SLA {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.6.10 SRA n
        0x2F => Ok(DI{ins: SRA {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        0x28 => Ok(DI{ins: SRA {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        0x29 => Ok(DI{ins: SRA {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        0x2A => Ok(DI{ins: SRA {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        0x2B => Ok(DI{ins: SRA {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        0x2C => Ok(DI{ins: SRA {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        0x2D => Ok(DI{ins: SRA {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        0x2E => Ok(DI{ins: SRA {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.6.11 SRL n
        0x3F => Ok(DI{ins: SRL {target: R8(A)}, cycles: 8, advance: 2, flags: None }),
        0x38 => Ok(DI{ins: SRL {target: R8(B)}, cycles: 8, advance: 2, flags: None }),
        0x39 => Ok(DI{ins: SRL {target: R8(C)}, cycles: 8, advance: 2, flags: None }),
        0x3A => Ok(DI{ins: SRL {target: R8(D)}, cycles: 8, advance: 2, flags: None }),
        0x3B => Ok(DI{ins: SRL {target: R8(E)}, cycles: 8, advance: 2, flags: None }),
        0x3C => Ok(DI{ins: SRL {target: R8(H)}, cycles: 8, advance: 2, flags: None }),
        0x3D => Ok(DI{ins: SRL {target: R8(L)}, cycles: 8, advance: 2, flags: None }),
        0x3E => Ok(DI{ins: SRL {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }),

        // sec 3.3.7.1 BIT b,r
        x if x & 0b1100_0000 == 0x40 => 
        {
            let bit = (x & 0b0011_1000) >> 3;
            let register = x & 0b0000_0111;
            match register {
                0 => Ok(DI{ins: BIT {src: R8(B), bit}, cycles: 8, advance: 2, flags: None }),
                1 => Ok(DI{ins: BIT {src: R8(C), bit}, cycles: 8, advance: 2, flags: None }),
                2 => Ok(DI{ins: BIT {src: R8(D), bit}, cycles: 8, advance: 2, flags: None }),
                3 => Ok(DI{ins: BIT {src: R8(E), bit}, cycles: 8, advance: 2, flags: None }),
                4 => Ok(DI{ins: BIT {src: R8(H), bit}, cycles: 8, advance: 2, flags: None }),
                5 => Ok(DI{ins: BIT {src: R8(L), bit}, cycles: 8, advance: 2, flags: None }),
                6 => Ok(DI{ins: BIT {src: Indirect8(HL), bit}, cycles: 16, advance: 2, flags: None }),
                7 => Ok(DI{ins: BIT {src: R8(A), bit}, cycles: 8, advance: 2, flags: None }),
                _ => unreachable!(),
            }
        },

        // sec 3.3.7.2 SET b,r
        x if x & 0b1100_0000 == 0xC0 => 
        {
            let bit = (x & 0b0011_1000) >> 3;
            let register = x & 0b0000_0111;
            match register {
                0 => Ok(DI{ins: SET {dest: R8(B), bit}, cycles: 8, advance: 2, flags: None }),
                1 => Ok(DI{ins: SET {dest: R8(C), bit}, cycles: 8, advance: 2, flags: None }),
                2 => Ok(DI{ins: SET {dest: R8(D), bit}, cycles: 8, advance: 2, flags: None }),
                3 => Ok(DI{ins: SET {dest: R8(E), bit}, cycles: 8, advance: 2, flags: None }),
                4 => Ok(DI{ins: SET {dest: R8(H), bit}, cycles: 8, advance: 2, flags: None }),
                5 => Ok(DI{ins: SET {dest: R8(L), bit}, cycles: 8, advance: 2, flags: None }),
                6 => Ok(DI{ins: SET {dest: Indirect8(HL), bit}, cycles: 16, advance: 2, flags: None }),
                7 => Ok(DI{ins: SET {dest: R8(A), bit}, cycles: 8, advance: 2, flags: None }),
                _ => unreachable!(),
            }
        },

        // sec 3.3.7.3 RES b,r
        x if x & 0b1100_0000 == 0x80 => 
        {
            let bit = (x & 0b0011_1000) >> 3;
            let register = x & 0b0000_0111;
            match register {
                0 => Ok(DI{ins: RES {dest: R8(B), bit}, cycles: 8, advance: 2, flags: None }),
                1 => Ok(DI{ins: RES {dest: R8(C), bit}, cycles: 8, advance: 2, flags: None }),
                2 => Ok(DI{ins: RES {dest: R8(D), bit}, cycles: 8, advance: 2, flags: None }),
                3 => Ok(DI{ins: RES {dest: R8(E), bit}, cycles: 8, advance: 2, flags: None }),
                4 => Ok(DI{ins: RES {dest: R8(H), bit}, cycles: 8, advance: 2, flags: None }),
                5 => Ok(DI{ins: RES {dest: R8(L), bit}, cycles: 8, advance: 2, flags: None }),
                6 => Ok(DI{ins: RES {dest: Indirect8(HL), bit}, cycles: 16, advance: 2, flags: None }),
                7 => Ok(DI{ins: RES {dest: R8(A), bit}, cycles: 8, advance: 2, flags: None }),
                _ => unreachable!(),
            }
        },

        _ => Err(()),
    }
}

/// Constructs a u16 immediate from a high and low byte
fn im16(h: &u8, l: &u8) -> u16 {
    (*h as u16) << 8 | (*l as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    // test im16 function for correct endianess
    #[test]
    fn test_im16_endianess()
    {
        assert_eq!(im16(&0x12, &0x34), 0x1234);
    }

    // #[test]
    // fn test_decode_ld8imitate() {
    //     assert_eq!(
    //         decode_next_instruction(&[0x06u8, 42]),
    //         (
    //             LD8Immediate {
    //                 dest: Reg8::B,
    //                 value: 42
    //             },
    //             Cycles(8),
    //             PCAdvance(1)
    //         )
    //     );
    //     assert_eq!(
    //         decode_next_instruction(&[0x0Eu8, 42]),
    //         (
    //             LD8Immediate {
    //                 dest: Reg8::C,
    //                 value: 42
    //             },
    //             Cycles(8),
    //             PCAdvance(1)
    //         )
    //     );
    //     assert_eq!(
    //         decode_next_instruction(&[0x16u8, 42]),
    //         (
    //             LD8Immediate {
    //                 dest: Reg8::D,
    //                 value: 42
    //             },
    //             Cycles(8),
    //             PCAdvance(1)
    //         )
    //     );
    //     assert_eq!(
    //         decode_next_instruction(&[0x1Eu8, 42]),
    //         (
    //             LD8Immediate {
    //                 dest: Reg8::E,
    //                 value: 42
    //             },
    //             Cycles(8),
    //             PCAdvance(1)
    //         )
    //     );
    //     assert_eq!(
    //         decode_next_instruction(&[0x26u8, 42]),
    //         (
    //             LD8Immediate {
    //                 dest: Reg8::H,
    //                 value: 42
    //             },
    //             Cycles(8),
    //             PCAdvance(1)
    //         )
    //     );
    //     assert_eq!(
    //         decode_next_instruction(&[0x2Eu8, 42]),
    //         (
    //             LD8Immediate {
    //                 dest: Reg8::L,
    //                 value: 42
    //             },
    //             Cycles(8),
    //             PCAdvance(1)
    //         )
    //     );
    // }
}
