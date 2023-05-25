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

#[rustfmt::skip]
pub fn decode_next_instruction(instruction_stream: &[u8]) -> DecodedInstruction {
    use DecodedInstruction as DI;
    use Reg8::*;
    use Reg16::*;

    match instruction_stream {

        // 3.3.1 8-Bit Loads
        // sec 3.3.1.1 Put value nn into n.
        [0x06, x, ..] => DI{ins: LD8{dest: R8(B), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x0E, x, ..] => DI{ins: LD8{dest: R8(C), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x16, x, ..] => DI{ins: LD8{dest: R8(D), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x1E, x, ..] => DI{ins: LD8{dest: R8(E), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x26, x, ..] => DI{ins: LD8{dest: R8(H), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x2E, x, ..] => DI{ins: LD8{dest: R8(L), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},

        // sec 3.3.1.2 Put value r2 into r1.
        [0x7F, ..] => DI{ins: LD8{dest: R8(A), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x78, ..] => DI{ins: LD8{dest: R8(A), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x79, ..] => DI{ins: LD8{dest: R8(A), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x7A, ..] => DI{ins: LD8{dest: R8(A), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x7B, ..] => DI{ins: LD8{dest: R8(A), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x7C, ..] => DI{ins: LD8{dest: R8(A), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x7D, ..] => DI{ins: LD8{dest: R8(A), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x7E, ..] => DI{ins: LD8{dest: R8(A), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x40, ..] => DI{ins: LD8{dest: R8(B), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x41, ..] => DI{ins: LD8{dest: R8(B), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x42, ..] => DI{ins: LD8{dest: R8(B), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x43, ..] => DI{ins: LD8{dest: R8(B), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x44, ..] => DI{ins: LD8{dest: R8(B), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x45, ..] => DI{ins: LD8{dest: R8(B), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x46, ..] => DI{ins: LD8{dest: R8(B), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x48, ..] => DI{ins: LD8{dest: R8(C), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x49, ..] => DI{ins: LD8{dest: R8(C), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x4A, ..] => DI{ins: LD8{dest: R8(C), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x4B, ..] => DI{ins: LD8{dest: R8(C), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x4C, ..] => DI{ins: LD8{dest: R8(C), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x4D, ..] => DI{ins: LD8{dest: R8(C), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x4E, ..] => DI{ins: LD8{dest: R8(C), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x50, ..] => DI{ins: LD8{dest: R8(D), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x51, ..] => DI{ins: LD8{dest: R8(D), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x52, ..] => DI{ins: LD8{dest: R8(D), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x53, ..] => DI{ins: LD8{dest: R8(D), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x54, ..] => DI{ins: LD8{dest: R8(D), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x55, ..] => DI{ins: LD8{dest: R8(D), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x56, ..] => DI{ins: LD8{dest: R8(D), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x58, ..] => DI{ins: LD8{dest: R8(E), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x59, ..] => DI{ins: LD8{dest: R8(E), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x5A, ..] => DI{ins: LD8{dest: R8(E), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x5B, ..] => DI{ins: LD8{dest: R8(E), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x5C, ..] => DI{ins: LD8{dest: R8(E), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x5D, ..] => DI{ins: LD8{dest: R8(E), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x5E, ..] => DI{ins: LD8{dest: R8(E), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x60, ..] => DI{ins: LD8{dest: R8(H), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x61, ..] => DI{ins: LD8{dest: R8(H), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x62, ..] => DI{ins: LD8{dest: R8(H), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x63, ..] => DI{ins: LD8{dest: R8(H), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x64, ..] => DI{ins: LD8{dest: R8(H), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x65, ..] => DI{ins: LD8{dest: R8(H), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x66, ..] => DI{ins: LD8{dest: R8(H), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x68, ..] => DI{ins: LD8{dest: R8(L), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x69, ..] => DI{ins: LD8{dest: R8(L), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x6A, ..] => DI{ins: LD8{dest: R8(L), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x6B, ..] => DI{ins: LD8{dest: R8(L), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x6C, ..] => DI{ins: LD8{dest: R8(L), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x6D, ..] => DI{ins: LD8{dest: R8(L), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x6E, ..] => DI{ins: LD8{dest: R8(L), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x70, ..] => DI{ins: LD8{src: R8(B), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x71, ..] => DI{ins: LD8{src: R8(C), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x72, ..] => DI{ins: LD8{src: R8(D), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x73, ..] => DI{ins: LD8{src: R8(E), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x74, ..] => DI{ins: LD8{src: R8(H), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x75, ..] => DI{ins: LD8{src: R8(L), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x36, x, ..] => DI{ins: LD8{dest: Indirect8(HL), src: Immediate8(*x)}, cycles: 12, advance: 2, flags: None},

        // sec 3.3.1.3 Put value n into A.
        // ignoring commands repeated from sec 3.3.1.2
        [0x0A, ..] => DI{ins: LD8{dest: R8(A), src: Indirect8(BC)}, cycles: 8, advance: 1, flags: None},
        [0x1A, ..] => DI{ins: LD8{dest: R8(A), src: Indirect8(DE)}, cycles: 8, advance: 1, flags: None},
        [0xFA, l, h, ..] => DI{ ins: LD8{ dest: R8(A), src: IndirectImmediate8(im16(h, l)) }, cycles: 16, advance: 3, flags: None},
        [0x3E, x, ..] => DI{ins: LD8{dest: R8(A), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},

        // sec 3.3.1.4 Put value A into n.
        [0x47, ..] => DI{ins: LD8{dest: R8(B), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x4F, ..] => DI{ins: LD8{dest: R8(C), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x57, ..] => DI{ins: LD8{dest: R8(D), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x5F, ..] => DI{ins: LD8{dest: R8(E), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x67, ..] => DI{ins: LD8{dest: R8(H), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x6F, ..] => DI{ins: LD8{dest: R8(L), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x02, ..] => DI{ins: LD8{src: R8(A), dest: Indirect8(BC)}, cycles: 8, advance: 1, flags: None},
        [0x12, ..] => DI{ins: LD8{src: R8(A), dest: Indirect8(DE)}, cycles: 8, advance: 1, flags: None},
        [0xEA, l, h, ..] => DI{ins: LD8{dest: IndirectImmediate8(im16(h, l)),  src: R8(A) }, cycles: 16, advance: 3, flags: None},

        // sec 3.3.1.5 Put value at address $FF00 + register C into A.
        [0xF2, ..] => DI{ins: LD8{dest: R8(A), src: IndirectHigh8(C)}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.6 Put A into address $FF00 + register C.
        [0xE2, ..] => DI{ins: LD8{src: R8(A), dest: IndirectHigh8(C)}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.7
        // N/A

        // sec 3.3.1.8
        // N/A

        // sec 3.3.1.9 Put value at address HL into A. Decrement HL.
        [0x3A, ..] => DI{ins: LD8 {dest: R8(A), src: Indirect8PostDec(HL)}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.10
        // N/A

        // sec 3.3.1.11
        // N/A

        // sec 3.3.1.12 Put A into memory address HL. Decrement HL.
        [0x32, ..] => DI{ins: LD8 {dest: Indirect8PostDec(HL), src: R8(A)}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.13
        // N/A

        // sec 3.3.1.14
        // N/A

        // sec 3.3.1.15 Put value at address HL into A. Increment HL.
        [0x2A, ..] => DI{ins: LD8 {dest: R8(A), src: Indirect8PostInc(HL)}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.16
        // N/A

        // sec 3.3.1.17
        // N/A

        // sec 3.3.1.18 Put A into memory address HL. Increment HL.
        [0x22, ..] => DI{ins: LD8 {dest: Indirect8PostInc(HL), src: R8(A)}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.19 Put A into memory address $FF00+n.
        [0xE0, x, ..] => DI{ins: LD8 {dest: IndirectImmediateHigh8(*x), src: R8(A), }, cycles: 12, advance: 2, flags: None},

        // sec 3.3.1.20 Put memory address $FF00+n into A.
        [0xF0, x, ..] => DI{ins: LD8 {dest: R8(A), src: IndirectImmediateHigh8(*x)}, cycles: 12, advance: 2, flags: None},

        // 3.3.2. 16-Bit Loads
        // sec 3.3.2.1 Put value nn into n.
        [0x01, l, h, ..] => DI{ins: LD16 {dest: R16(BC), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None},
        [0x11, l, h, ..] => DI{ins: LD16 {dest: R16(DE), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None},
        [0x21, l, h, ..] => DI{ins: LD16 {dest: R16(HL), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None},
        [0x31, l, h, ..] => DI{ins: LD16 {dest: R16(SP), src: Immediate16(im16(h, l))}, cycles: 12, advance: 3, flags: None},

        // sec 3.3.2.2 Put HL into Stack Pointer (SP).
        [0xF9, ..] => DI{ins: LD16 {dest: R16(SP), src: R16(HL)}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.2.3
        // N/A

        // sec 3.3.2.4 Put SP + n effective address into HL.
        [0xF8, _x, ..] => unimplemented!(), //DI{ins: LDHL {dest: R16(HL), lhs: R16(SP), rhs: *x as i8}, cycles: 12, advance: 2, flags: None},

        // sec 3.3.2.5 Put Stack Pointer (SP) at address n.
        [0x08, l, h, ..] => DI{ins: LD16 { src: R16(SP), dest: IndirectImmediate16(im16(h, l))}, cycles: 20, advance: 3, flags: None},

        // sec 3.3.2.6 Push register pair nn onto stack. Decrement Stack Pointer (SP) twice.
        [0xF5, ..] => DI{ins: PUSH {src: AF}, cycles: 16, advance: 1, flags: None},
        [0xC5, ..] => DI{ins: PUSH {src: BC}, cycles: 16, advance: 1, flags: None},
        [0xD5, ..] => DI{ins: PUSH {src: DE}, cycles: 16, advance: 1, flags: None},
        [0xE5, ..] => DI{ins: PUSH {src: HL}, cycles: 16, advance: 1, flags: None},

        // sec 3.3.2.7 Pop two bytes off stack into register pair nn. Increment Stack Pointer (SP) twice.
        [0xF1, ..] => DI{ins: POP {dest: AF}, cycles: 12, advance: 1, flags: None},
        [0xC1, ..] => DI{ins: POP {dest: BC}, cycles: 12, advance: 1, flags: None},
        [0xD1, ..] => DI{ins: POP {dest: DE}, cycles: 12, advance: 1, flags: None},
        [0xE1, ..] => DI{ins: POP {dest: HL}, cycles: 12, advance: 1, flags: None},

        // 3.3.2. 16-Bit Loads
        // sec 3.3.3.1 Add n to A.
        [0x87, ..] => DI{ins: ADD8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x80, ..] => DI{ins: ADD8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x81, ..] => DI{ins: ADD8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x82, ..] => DI{ins: ADD8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x83, ..] => DI{ins: ADD8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x84, ..] => DI{ins: ADD8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x85, ..] => DI{ins: ADD8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x86, ..] => DI{ins: ADD8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xC6, x, ..] => DI{ins: ADD8 { rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC)},

        // sec 3.3.3.2 
        [0x8F, ..] => DI{ins: ADC8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x88, ..] => DI{ins: ADC8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x89, ..] => DI{ins: ADC8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8A, ..] => DI{ins: ADC8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8B, ..] => DI{ins: ADC8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8C, ..] => DI{ins: ADC8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8D, ..] => DI{ins: ADC8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8E, ..] => DI{ins: ADC8{rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xCE, x, ..] => DI{ins: ADC8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC)},

        // sec 3.3.3.3 SUB n
        [0x97, ..] => DI{ins: SUB8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x90, ..] => DI{ins: SUB8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x91, ..] => DI{ins: SUB8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x92, ..] => DI{ins: SUB8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x93, ..] => DI{ins: SUB8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x94, ..] => DI{ins: SUB8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x95, ..] => DI{ins: SUB8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x96, ..] => DI{ins: SUB8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0xD6, x, ..] => DI{ins: SUB8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},

        // sec 3.3.3.4 SBC A,n
        [0x9F, ..] => DI{ins: SBC8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x98, ..] => DI{ins: SBC8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x99, ..] => DI{ins: SBC8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9A, ..] => DI{ins: SBC8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9B, ..] => DI{ins: SBC8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9C, ..] => DI{ins: SBC8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9D, ..] => DI{ins: SBC8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9E, ..] => DI{ins: SBC8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0xDE, x, ..] => DI{ins: SBC8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},

        // sec 3.3.3.5 AND n
        [0xA7, ..] => DI{ins: AND8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA0, ..] => DI{ins: AND8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA1, ..] => DI{ins: AND8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA2, ..] => DI{ins: AND8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA3, ..] => DI{ins: AND8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA4, ..] => DI{ins: AND8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA5, ..] => DI{ins: AND8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA6, ..] => DI{ins: AND8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xE6, x, ..] => DI{ins: AND8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC)},

        // sec 3.3.3.6 OR n
        [0xB7, ..] => DI{ins: OR8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB0, ..] => DI{ins: OR8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB1, ..] => DI{ins: OR8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB2, ..] => DI{ins: OR8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB3, ..] => DI{ins: OR8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB4, ..] => DI{ins: OR8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB5, ..] => DI{ins: OR8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB6, ..] => DI{ins: OR8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xF6, x, ..] => DI{ins: OR8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.3.7 XOR n
        [0xAF, ..] => DI{ins: XOR8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xA8, ..] => DI{ins: XOR8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xA9, ..] => DI{ins: XOR8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAA, ..] => DI{ins: XOR8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAB, ..] => DI{ins: XOR8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAC, ..] => DI{ins: XOR8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAD, ..] => DI{ins: XOR8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAE, ..] => DI{ins: XOR8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xEE, x, ..] => DI{ins: XOR8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.3.8 CP n
        [0xBF, ..] => DI{ins: CP8 {rhs: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xB8, ..] => DI{ins: CP8 {rhs: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xB9, ..] => DI{ins: CP8 {rhs: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBA, ..] => DI{ins: CP8 {rhs: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBB, ..] => DI{ins: CP8 {rhs: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBC, ..] => DI{ins: CP8 {rhs: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBD, ..] => DI{ins: CP8 {rhs: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBE, ..] => DI{ins: CP8 {rhs: Indirect8(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xFE, x, ..] => DI{ins: CP8{ rhs: Immediate8(*x) }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},

        // sec 3.3.3.9 INC n
        [0x3C, ..] => DI{ins: INC8 {target: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x04, ..] => DI{ins: INC8 {target: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x0C, ..] => DI{ins: INC8 {target: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x14, ..] => DI{ins: INC8 {target: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x1C, ..] => DI{ins: INC8 {target: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x24, ..] => DI{ins: INC8 {target: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x2C, ..] => DI{ins: INC8 {target: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x34, ..] => DI{ins: INC8 {target: Indirect8(HL)}, cycles: 12, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},

        // sec 3.3.3.10 DEC n
        [0x3D, ..] => DI{ins: DEC8 {target: R8(A)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},
        [0x05, ..] => DI{ins: DEC8 {target: R8(B)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},
        [0x0D, ..] => DI{ins: DEC8 {target: R8(C)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},
        [0x15, ..] => DI{ins: DEC8 {target: R8(D)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},
        [0x1D, ..] => DI{ins: DEC8 {target: R8(E)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},
        [0x25, ..] => DI{ins: DEC8 {target: R8(H)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},
        [0x2D, ..] => DI{ins: DEC8 {target: R8(L)}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},
        [0x35, ..] => DI{ins: DEC8 {target: Indirect8(HL)}, cycles: 12, advance: 1, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC)},

        // sec 3.3.4.1 ADD HL,n
        [0x09, ..] => DI{ins: ADD16 {dest: R16(HL), rhs: R16(BC)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::HC | Flags::C)},
        [0x19, ..] => DI{ins: ADD16 {dest: R16(HL), rhs: R16(DE)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::HC | Flags::C)},
        [0x29, ..] => DI{ins: ADD16 {dest: R16(HL), rhs: R16(HL)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::HC | Flags::C)},
        [0x39, ..] => DI{ins: ADD16 {dest: R16(HL), rhs: R16(SP)}, cycles: 8, advance: 1, flags: None }, //Some(Flags::HC | Flags::C)},

        // sec 3.3.4.2 ADD SP, n
        [0xE8, x, ..] => unimplemented!(), //DI{ins: ADD16 {dest: R16(SP), rhs: Immediate16(*x as i8 as u16)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z | Flags::N | Flags::HC | Flags::C)},
        
        // sec 3.3.4.3 INC nn
        [0x03, ..] => DI{ins: INC16 {target: BC}, cycles: 8, advance: 1, flags: None },
        [0x13, ..] => DI{ins: INC16 {target: DE}, cycles: 8, advance: 1, flags: None },
        [0x23, ..] => DI{ins: INC16 {target: HL}, cycles: 8, advance: 1, flags: None },
        [0x33, ..] => DI{ins: INC16 {target: SP}, cycles: 8, advance: 1, flags: None },

        // sec 3.3.4.4 DEC nn
        [0x0B, ..] => DI{ins: DEC16 {target: BC}, cycles: 8, advance: 1, flags: None },
        [0x1B, ..] => DI{ins: DEC16 {target: DE}, cycles: 8, advance: 1, flags: None },
        [0x2B, ..] => DI{ins: DEC16 {target: HL}, cycles: 8, advance: 1, flags: None },
        [0x3B, ..] => DI{ins: DEC16 {target: SP}, cycles: 8, advance: 1, flags: None },

        // sec 3.3.5.1 SWAP n
        [0xCB, 0x37, ..] => DI{ins: SWAP8 {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x30, ..] => DI{ins: SWAP8 {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x31, ..] => DI{ins: SWAP8 {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x32, ..] => DI{ins: SWAP8 {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x33, ..] => DI{ins: SWAP8 {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x34, ..] => DI{ins: SWAP8 {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x35, ..] => DI{ins: SWAP8 {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x36, ..] => DI{ins: SWAP8 {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.5.2 DDA
        [0x27, ..] => DI{ins: DAA, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        
        // sec 3.3.5.3 CPL
        [0x2F, ..] => DI{ins: CPL, cycles: 4, advance: 1, flags: None }, //Some(Flags::N | Flags::HC)},
        
        // sec 3.3.5.4 CCF
        [0x3F, ..] => DI{ins: CCF, cycles: 4, advance: 1, flags: None }, //Some(Flags::C)},
        
        // sec 3.3.5.5 SCF
        [0x37, ..] => DI{ins: SCF, cycles: 4, advance: 1, flags: None }, //Some(Flags::C)},

        // 3.3.5.6 NOP
        [0x00, ..] => DI{ins: NOP, cycles: 4, advance: 1, flags: None },

        // 3.3.5.7 HALT
        [0x76, ..] => DI{ins: HALT, cycles: 4, advance: 1, flags: None },

        // 3.3.5.8 STOP
        [0x10, 0x00, ..] => DI{ins: STOP, cycles: 4, advance: 2, flags: None },

        // 3.3.5.9 DI
        [0xF3, ..] => DI{ins: DI, cycles: 4, advance: 1, flags: None },

        // 3.3.5.10 EI
        [0xFB, ..] => DI{ins: EI, cycles: 4, advance: 1, flags: None },

        // sec 3.3.6.1 RLCA
        [0x07, ..] => DI{ins: RLCA, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.2 RLA
        [0x17, ..] => DI{ins: RLA, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.3 RRCA
        [0x0F, ..] => DI{ins: RRCA, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.4 RRA
        [0x1F, ..] => DI{ins: RRA, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.5 RLC n
        [0xCB, 0x07, ..] => DI{ins: RLC {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x00, ..] => DI{ins: RLC {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x01, ..] => DI{ins: RLC {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x02, ..] => DI{ins: RLC {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x03, ..] => DI{ins: RLC {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x04, ..] => DI{ins: RLC {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x05, ..] => DI{ins: RLC {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x06, ..] => DI{ins: RLC {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.6 RL n
        [0xCB, 0x17, ..] => DI{ins: RL {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x10, ..] => DI{ins: RL {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x11, ..] => DI{ins: RL {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x12, ..] => DI{ins: RL {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x13, ..] => DI{ins: RL {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x14, ..] => DI{ins: RL {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x15, ..] => DI{ins: RL {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x16, ..] => DI{ins: RL {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.7 RRC n
        [0xCB, 0x0F, ..] => DI{ins: RRC {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x08, ..] => DI{ins: RRC {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x09, ..] => DI{ins: RRC {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x0A, ..] => DI{ins: RRC {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x0B, ..] => DI{ins: RRC {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x0C, ..] => DI{ins: RRC {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x0D, ..] => DI{ins: RRC {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x0E, ..] => DI{ins: RRC {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.8 RR n
        [0xCB, 0x1F, ..] => DI{ins: RR {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x18, ..] => DI{ins: RR {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x19, ..] => DI{ins: RR {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x1A, ..] => DI{ins: RR {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x1B, ..] => DI{ins: RR {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x1C, ..] => DI{ins: RR {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x1D, ..] => DI{ins: RR {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x1E, ..] => DI{ins: RR {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.9 SLA n
        [0xCB, 0x27, ..] => DI{ins: SLA {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x20, ..] => DI{ins: SLA {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x21, ..] => DI{ins: SLA {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x22, ..] => DI{ins: SLA {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x23, ..] => DI{ins: SLA {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x24, ..] => DI{ins: SLA {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x25, ..] => DI{ins: SLA {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x26, ..] => DI{ins: SLA {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.10 SRA n
        [0xCB, 0x2F, ..] => DI{ins: SRA {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x28, ..] => DI{ins: SRA {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x29, ..] => DI{ins: SRA {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x2A, ..] => DI{ins: SRA {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x2B, ..] => DI{ins: SRA {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x2C, ..] => DI{ins: SRA {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x2D, ..] => DI{ins: SRA {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x2E, ..] => DI{ins: SRA {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.6.11 SRL n
        [0xCB, 0x3F, ..] => DI{ins: SRL {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x38, ..] => DI{ins: SRL {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x39, ..] => DI{ins: SRL {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x3A, ..] => DI{ins: SRL {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x3B, ..] => DI{ins: SRL {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x3C, ..] => DI{ins: SRL {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x3D, ..] => DI{ins: SRL {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        [0xCB, 0x3E, ..] => DI{ins: SRL {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},

        // src 3.3.7.1 BIT b,r
        // [0xCB, 0x3F, ..] => DI{ins: BIT {target: R8(A)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        // [0xCB, 0x38, ..] => DI{ins: BIT {target: R8(B)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        // [0xCB, 0x39, ..] => DI{ins: BIT {target: R8(C)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        // [0xCB, 0x3A, ..] => DI{ins: BIT {target: R8(D)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        // [0xCB, 0x3B, ..] => DI{ins: BIT {target: R8(E)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        // [0xCB, 0x3C, ..] => DI{ins: BIT {target: R8(H)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        // [0xCB, 0x3D, ..] => DI{ins: BIT {target: R8(L)}, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},
        // [0xCB, 0x3E, ..] => DI{ins: BIT {target: Indirect8(HL)}, cycles: 16, advance: 2, flags: None }, //Some(Flags::Z)},



        _ => panic!("unimplemented instruction")
    }
}

/// Constructs a u16 immediate from a high and low byte
fn im16(h: &u8, l: &u8) -> u16 {
    (*h as u16) << 8 | (*l as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

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
