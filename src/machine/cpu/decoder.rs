use super::Reg16;
use super::Reg8;
use super::Operand;
use super::Flags;
use super::Flags::*;



// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub struct Cycles(u8);

// #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
// pub struct PCAdvance(u8);

use super::Instruction;
use super::Instruction::*;

use super::Operand::*;

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
        // sec 3.3.1.1
        [0x06, x, ..] => DI{ins: LD{dest: R8(B), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x0E, x, ..] => DI{ins: LD{dest: R8(C), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x16, x, ..] => DI{ins: LD{dest: R8(D), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x1E, x, ..] => DI{ins: LD{dest: R8(E), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x26, x, ..] => DI{ins: LD{dest: R8(H), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},
        [0x2E, x, ..] => DI{ins: LD{dest: R8(L), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},

        // sec 3.3.1.2
        [0x7F, ..] => DI{ins: LD{dest: R8(A), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x78, ..] => DI{ins: LD{dest: R8(A), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x79, ..] => DI{ins: LD{dest: R8(A), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x7A, ..] => DI{ins: LD{dest: R8(A), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x7B, ..] => DI{ins: LD{dest: R8(A), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x7C, ..] => DI{ins: LD{dest: R8(A), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x7D, ..] => DI{ins: LD{dest: R8(A), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x7E, ..] => DI{ins: LD{dest: R8(A), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x40, ..] => DI{ins: LD{dest: R8(B), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x41, ..] => DI{ins: LD{dest: R8(B), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x42, ..] => DI{ins: LD{dest: R8(B), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x43, ..] => DI{ins: LD{dest: R8(B), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x44, ..] => DI{ins: LD{dest: R8(B), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x45, ..] => DI{ins: LD{dest: R8(B), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x46, ..] => DI{ins: LD{dest: R8(B), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x48, ..] => DI{ins: LD{dest: R8(C), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x49, ..] => DI{ins: LD{dest: R8(C), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x4A, ..] => DI{ins: LD{dest: R8(C), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x4B, ..] => DI{ins: LD{dest: R8(C), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x4C, ..] => DI{ins: LD{dest: R8(C), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x4D, ..] => DI{ins: LD{dest: R8(C), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x4E, ..] => DI{ins: LD{dest: R8(C), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x50, ..] => DI{ins: LD{dest: R8(D), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x51, ..] => DI{ins: LD{dest: R8(D), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x52, ..] => DI{ins: LD{dest: R8(D), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x53, ..] => DI{ins: LD{dest: R8(D), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x54, ..] => DI{ins: LD{dest: R8(D), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x55, ..] => DI{ins: LD{dest: R8(D), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x56, ..] => DI{ins: LD{dest: R8(D), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x58, ..] => DI{ins: LD{dest: R8(E), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x59, ..] => DI{ins: LD{dest: R8(E), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x5A, ..] => DI{ins: LD{dest: R8(E), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x5B, ..] => DI{ins: LD{dest: R8(E), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x5C, ..] => DI{ins: LD{dest: R8(E), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x5D, ..] => DI{ins: LD{dest: R8(E), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x5E, ..] => DI{ins: LD{dest: R8(E), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x60, ..] => DI{ins: LD{dest: R8(H), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x61, ..] => DI{ins: LD{dest: R8(H), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x62, ..] => DI{ins: LD{dest: R8(H), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x63, ..] => DI{ins: LD{dest: R8(H), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x64, ..] => DI{ins: LD{dest: R8(H), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x65, ..] => DI{ins: LD{dest: R8(H), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x66, ..] => DI{ins: LD{dest: R8(H), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x68, ..] => DI{ins: LD{dest: R8(L), src: R8(B)}, cycles: 4, advance: 1, flags: None},
        [0x69, ..] => DI{ins: LD{dest: R8(L), src: R8(C)}, cycles: 4, advance: 1, flags: None},
        [0x6A, ..] => DI{ins: LD{dest: R8(L), src: R8(D)}, cycles: 4, advance: 1, flags: None},
        [0x6B, ..] => DI{ins: LD{dest: R8(L), src: R8(E)}, cycles: 4, advance: 1, flags: None},
        [0x6C, ..] => DI{ins: LD{dest: R8(L), src: R8(H)}, cycles: 4, advance: 1, flags: None},
        [0x6D, ..] => DI{ins: LD{dest: R8(L), src: R8(L)}, cycles: 4, advance: 1, flags: None},
        [0x6E, ..] => DI{ins: LD{dest: R8(L), src: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},

        [0x70, ..] => DI{ins: LD{src: R8(B), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x71, ..] => DI{ins: LD{src: R8(C), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x72, ..] => DI{ins: LD{src: R8(D), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x73, ..] => DI{ins: LD{src: R8(E), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x74, ..] => DI{ins: LD{src: R8(H), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x75, ..] => DI{ins: LD{src: R8(L), dest: Indirect8(HL)}, cycles: 8, advance: 1, flags: None},
        [0x36, x, ..] => DI{ins: LD{dest: R16(HL), src: Immediate8(*x)}, cycles: 12, advance: 2, flags: None},

        // sec 3.3.1.3
        // ignoring commands repeated from sec 3.3.1.2
        [0x0A, ..] => DI{ins: LD{dest: R8(A), src: Indirect8(BC)}, cycles: 8, advance: 1, flags: None},
        [0x1A, ..] => DI{ins: LD{dest: R8(A), src: Indirect8(DE)}, cycles: 8, advance: 1, flags: None},
        [0xFA, l, h, ..] => DI{dest: R8(A), src: IndirectImmediate8(im16(h, l)) }, cycles: 16, advance: 3, flags: None},
        [0x3E, x, ..] => DI{ins: LD{dest: R8(A), src: Immediate8(*x)}, cycles: 8, advance: 2, flags: None},

        // sec 3.3.1.4
        [0x47, ..] => DI{ins: LD{dest: R8(B), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x4F, ..] => DI{ins: LD{dest: R8(C), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x57, ..] => DI{ins: LD{dest: R8(D), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x5F, ..] => DI{ins: LD{dest: R8(E), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x67, ..] => DI{ins: LD{dest: R8(H), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x6F, ..] => DI{ins: LD{dest: R8(L), src: R8(A)}, cycles: 4, advance: 1, flags: None},
        [0x02, ..] => DI{ins: LD{src: R8(A), dest: Indirect8(BC)}, cycles: 8, advance: 1, flags: None},
        [0x12, ..] => DI{ins: LD{src: R8(A), dest: Indirect8(DE)}, cycles: 8, advance: 1, flags: None},
        [0xEA, l, h, ..] => DI{ins: LD{dest: Indirect8(im16(h, l)),  src: R8(A) , src: A}, cycles: 16, advance: 3, flags: None},

        // sec 3.3.1.5
        [0xF2, ..] => DI{ins: LDLoadHigh{dest: A, addr: C}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.6
        [0xE2, ..] => DI{ins: LDStoreHigh{addr: C, src: A}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.7 through 9
        [0x3A, ..] => DI{ins: LDDLoadDec {dest: A, addr: HL}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.10 through 12
        [0x32, ..] => DI{ins: LDDStoreDec {addr: HL, src: A, }, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.13 through 15
        [0x2A, ..] => DI{ins: LDILoadInc {dest: A, addr: HL}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.16 through 18
        [0x22, ..] => DI{ins: LDIStoreInc {addr: HL, src: A, }, cycles: 8, advance: 1, flags: None},

        // sec 3.3.1.19
        [0xE0, x, ..] => DI{ins: LDHStoreHighImmediate {addr: *x, src: A, }, cycles: 12, advance: 2, flags: None},

        // sec 3.3.1.20
        [0xF0, x, ..] => DI{ins: LDHLoadHighImmediate {dest: A, addr: *x}, cycles: 12, advance: 2, flags: None},

        // sec 3.3.2.1
        [0x01, l, h, ..] => DI{ins: LD16Immediate {dest: BC, value: im16(h, l)}, cycles: 12, advance: 3, flags: None},
        [0x11, l, h, ..] => DI{ins: LD16Immediate {dest: DE, value: im16(h, l)}, cycles: 12, advance: 3, flags: None},
        [0x21, l, h, ..] => DI{ins: LD16Immediate {dest: HL, value: im16(h, l)}, cycles: 12, advance: 3, flags: None},
        [0x31, l, h, ..] => DI{ins: LD16Immediate {dest: SP, value: im16(h, l)}, cycles: 12, advance: 3, flags: None},

        // sec 3.3.2.2
        [0xF9, ..] => DI{ins: LDMove16 {dest: SP, src: HL}, cycles: 8, advance: 1, flags: None},

        // sec 3.3.2.3
        // N/A

        // sec 3.3.2.4
        [0xF8, x, ..] => DI{ins: LDHLSP {offset: *x as i8}, cycles: 12, advance: 2, flags: None},

        // sec 3.3.2.5
        [0x08, l, h, ..] => DI{ins: LDStore16ImmediateAddr { src: SP, addr: im16(h, l)}, cycles: 20, advance: 3, flags: None},

        // sec 3.3.2.6
        [0xF5, ..] => DI{ins: PUSH {src: AF}, cycles: 16, advance: 1, flags: None},
        [0xC5, ..] => DI{ins: PUSH {src: BC}, cycles: 16, advance: 1, flags: None},
        [0xD5, ..] => DI{ins: PUSH {src: DE}, cycles: 16, advance: 1, flags: None},
        [0xE5, ..] => DI{ins: PUSH {src: HL}, cycles: 16, advance: 1, flags: None},

        // sec 3.3.2.7
        [0xF1, ..] => DI{ins: POP {dest: AF}, cycles: 12, advance: 1, flags: None},
        [0xC1, ..] => DI{ins: POP {dest: BC}, cycles: 12, advance: 1, flags: None},
        [0xD1, ..] => DI{ins: POP {dest: DE}, cycles: 12, advance: 1, flags: None},
        [0xE1, ..] => DI{ins: POP {dest: HL}, cycles: 12, advance: 1, flags: None},

        // sec 3.3.3.1
        [0x87, ..] => DI{ins: ADD8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x80, ..] => DI{ins: ADD8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x81, ..] => DI{ins: ADD8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x82, ..] => DI{ins: ADD8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x83, ..] => DI{ins: ADD8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x84, ..] => DI{ins: ADD8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x85, ..] => DI{ins: ADD8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x86, ..] => DI{ins: ADD8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xC6, x, ..] => DI{ins: ADD8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC)},

        // sec 3.3.3.2
        [0x8F, ..] => DI{ins: ADC8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x88, ..] => DI{ins: ADC8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x89, ..] => DI{ins: ADC8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8A, ..] => DI{ins: ADC8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8B, ..] => DI{ins: ADC8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8C, ..] => DI{ins: ADC8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8D, ..] => DI{ins: ADC8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0x8E, ..] => DI{ins: ADC8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xCE, x, ..] => DI{ins: ADC8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC)},

        // sec 3.3.3.3
        [0x97, ..] => DI{ins: SUB8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x90, ..] => DI{ins: SUB8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x91, ..] => DI{ins: SUB8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x92, ..] => DI{ins: SUB8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x93, ..] => DI{ins: SUB8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x94, ..] => DI{ins: SUB8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x95, ..] => DI{ins: SUB8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x96, ..] => DI{ins: SUB8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0xD6, x, ..] => DI{ins: SUB8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},

        // sec 3.3.3.4
        [0x9F, ..] => DI{ins: SBC8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x98, ..] => DI{ins: SBC8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x99, ..] => DI{ins: SBC8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9A, ..] => DI{ins: SBC8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9B, ..] => DI{ins: SBC8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9C, ..] => DI{ins: SBC8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9D, ..] => DI{ins: SBC8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0x9E, ..] => DI{ins: SBC8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},
        [0xDE, x, ..] => DI{ins: SBC8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC | Flags::N)},

        // sec 3.3.3.5
        [0xA7, ..] => DI{ins: AND8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA0, ..] => DI{ins: AND8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA1, ..] => DI{ins: AND8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA2, ..] => DI{ins: AND8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA3, ..] => DI{ins: AND8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA4, ..] => DI{ins: AND8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA5, ..] => DI{ins: AND8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xA6, ..] => DI{ins: AND8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC)},
        [0xE6, x, ..] => DI{ins: AND8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC)},

        // sec 3.3.3.6
        [0xB7, ..] => DI{ins: OR8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB0, ..] => DI{ins: OR8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB1, ..] => DI{ins: OR8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB2, ..] => DI{ins: OR8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB3, ..] => DI{ins: OR8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB4, ..] => DI{ins: OR8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB5, ..] => DI{ins: OR8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xB6, ..] => DI{ins: OR8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xF6, x, ..] => DI{ins: OR8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.3.7
        [0xAF, ..] => DI{ins: XOR8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xA8, ..] => DI{ins: XOR8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xA9, ..] => DI{ins: XOR8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAA, ..] => DI{ins: XOR8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAB, ..] => DI{ins: XOR8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAC, ..] => DI{ins: XOR8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAD, ..] => DI{ins: XOR8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xAE, ..] => DI{ins: XOR8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z)},
        [0xEE, x, ..] => DI{ins: XOR8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z)},

        // sec 3.3.3.8
        [0xBF, ..] => DI{ins: CP8ToAccumulator {src: A}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xB8, ..] => DI{ins: CP8ToAccumulator {src: B}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xB9, ..] => DI{ins: CP8ToAccumulator {src: C}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBA, ..] => DI{ins: CP8ToAccumulator {src: D}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBB, ..] => DI{ins: CP8ToAccumulator {src: E}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBC, ..] => DI{ins: CP8ToAccumulator {src: H}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBD, ..] => DI{ins: CP8ToAccumulator {src: L}, cycles: 4, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xBE, ..] => DI{ins: CP8AtHLToAccumulator {}, cycles: 8, advance: 1, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},
        [0xFE, x, ..] => DI{ins: CP8Immediate { value: *x }, cycles: 8, advance: 2, flags: None }, //Some(Flags::Z | Flags::HC | Flags::C)},


        

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

    #[test]
    fn test_decode_ld8imitate() {
        assert_eq!(
            decode_next_instruction(&[0x06u8, 42]),
            (
                LD8Immediate {
                    dest: Reg8::B,
                    value: 42
                },
                Cycles(8),
                PCAdvance(1)
            )
        );
        assert_eq!(
            decode_next_instruction(&[0x0Eu8, 42]),
            (
                LD8Immediate {
                    dest: Reg8::C,
                    value: 42
                },
                Cycles(8),
                PCAdvance(1)
            )
        );
        assert_eq!(
            decode_next_instruction(&[0x16u8, 42]),
            (
                LD8Immediate {
                    dest: Reg8::D,
                    value: 42
                },
                Cycles(8),
                PCAdvance(1)
            )
        );
        assert_eq!(
            decode_next_instruction(&[0x1Eu8, 42]),
            (
                LD8Immediate {
                    dest: Reg8::E,
                    value: 42
                },
                Cycles(8),
                PCAdvance(1)
            )
        );
        assert_eq!(
            decode_next_instruction(&[0x26u8, 42]),
            (
                LD8Immediate {
                    dest: Reg8::H,
                    value: 42
                },
                Cycles(8),
                PCAdvance(1)
            )
        );
        assert_eq!(
            decode_next_instruction(&[0x2Eu8, 42]),
            (
                LD8Immediate {
                    dest: Reg8::L,
                    value: 42
                },
                Cycles(8),
                PCAdvance(1)
            )
        );
    }
}
