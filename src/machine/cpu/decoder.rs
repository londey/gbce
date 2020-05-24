use super::Reg8;

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

pub fn decode_next_instruction(instruction_stream: &[u8]) -> DecodedInstruction {
    use DecodedInstruction as DI;
    use Reg8::*;

    // #[rustfmt::skip]
    match instruction_stream {
        // sec 3.3.1.1
        [0x06, x, ..] => DI{ins: LD8Immediate{dest: B, value: *x}, cycles: 8, advance: 2, flags: None},
        [0x0E, x, ..] => DI{ins: LD8Immediate{dest: C, value: *x}, cycles: 8, advance: 2, flags: None},
        [0x16, x, ..] => DI{ins: LD8Immediate{dest: D, value: *x}, cycles: 8, advance: 2, flags: None},
        [0x1E, x, ..] => DI{ins: LD8Immediate{dest: E, value: *x}, cycles: 8, advance: 2, flags: None},
        [0x26, x, ..] => DI{ins: LD8Immediate{dest: H, value: *x}, cycles: 8, advance: 2, flags: None},
        [0x2E, x, ..] => DI{ins: LD8Immediate{dest: L, value: *x}, cycles: 8, advance: 2, flags: None},

        // sec 3.3.1.2
        [0x7F, ..] => DI{ins: LDMove8{dest: A, src: A}, cycles: 4, advance: 1, flags: None},
        [0x78, ..] => DI{ins: LDMove8{dest: A, src: B}, cycles: 4, advance: 1, flags: None},
        [0x79, ..] => DI{ins: LDMove8{dest: A, src: C}, cycles: 4, advance: 1, flags: None},
        [0x7A, ..] => DI{ins: LDMove8{dest: A, src: D}, cycles: 4, advance: 1, flags: None},
        [0x7B, ..] => DI{ins: LDMove8{dest: A, src: E}, cycles: 4, advance: 1, flags: None},
        [0x7C, ..] => DI{ins: LDMove8{dest: A, src: H}, cycles: 4, advance: 1, flags: None},
        [0x7D, ..] => DI{ins: LDMove8{dest: A, src: L}, cycles: 4, advance: 1, flags: None},
        [0x7E, ..] => DI{ins: LDReadHL{dest: A}, cycles: 8, advance: 1, flags: None},

        [0x40, ..] => DI{ins: LDMove8{dest: B, src: B}, cycles: 4, advance: 1, flags: None},
        [0x41, ..] => DI{ins: LDMove8{dest: B, src: C}, cycles: 4, advance: 1, flags: None},
        [0x42, ..] => DI{ins: LDMove8{dest: B, src: D}, cycles: 4, advance: 1, flags: None},
        [0x43, ..] => DI{ins: LDMove8{dest: B, src: E}, cycles: 4, advance: 1, flags: None},
        [0x44, ..] => DI{ins: LDMove8{dest: B, src: H}, cycles: 4, advance: 1, flags: None},
        [0x45, ..] => DI{ins: LDMove8{dest: B, src: L}, cycles: 4, advance: 1, flags: None},
        [0x46, ..] => DI{ins: LDReadHL{dest: B}, cycles: 8, advance: 1, flags: None},

        [0x48, ..] => DI{ins: LDMove8{dest: C, src: B}, cycles: 4, advance: 1, flags: None},
        [0x49, ..] => DI{ins: LDMove8{dest: C, src: C}, cycles: 4, advance: 1, flags: None},
        [0x4A, ..] => DI{ins: LDMove8{dest: C, src: D}, cycles: 4, advance: 1, flags: None},
        [0x4B, ..] => DI{ins: LDMove8{dest: C, src: E}, cycles: 4, advance: 1, flags: None},
        [0x4C, ..] => DI{ins: LDMove8{dest: C, src: H}, cycles: 4, advance: 1, flags: None},
        [0x4D, ..] => DI{ins: LDMove8{dest: C, src: L}, cycles: 4, advance: 1, flags: None},
        [0x4E, ..] => DI{ins: LDReadHL{dest: C}, cycles: 8, advance: 1, flags: None},

        [0x50, ..] => DI{ins: LDMove8{dest: D, src: B}, cycles: 4, advance: 1, flags: None},
        [0x51, ..] => DI{ins: LDMove8{dest: D, src: C}, cycles: 4, advance: 1, flags: None},
        [0x52, ..] => DI{ins: LDMove8{dest: D, src: D}, cycles: 4, advance: 1, flags: None},
        [0x53, ..] => DI{ins: LDMove8{dest: D, src: E}, cycles: 4, advance: 1, flags: None},
        [0x54, ..] => DI{ins: LDMove8{dest: D, src: H}, cycles: 4, advance: 1, flags: None},
        [0x55, ..] => DI{ins: LDMove8{dest: D, src: L}, cycles: 4, advance: 1, flags: None},
        [0x56, ..] => DI{ins: LDReadHL{dest: D}, cycles: 8, advance: 1, flags: None},

        [0x58, ..] => DI{ins: LDMove8{dest: E, src: B}, cycles: 4, advance: 1, flags: None},
        [0x59, ..] => DI{ins: LDMove8{dest: E, src: C}, cycles: 4, advance: 1, flags: None},
        [0x5A, ..] => DI{ins: LDMove8{dest: E, src: D}, cycles: 4, advance: 1, flags: None},
        [0x5B, ..] => DI{ins: LDMove8{dest: E, src: E}, cycles: 4, advance: 1, flags: None},
        [0x5C, ..] => DI{ins: LDMove8{dest: E, src: H}, cycles: 4, advance: 1, flags: None},
        [0x5D, ..] => DI{ins: LDMove8{dest: E, src: L}, cycles: 4, advance: 1, flags: None},
        [0x5E, ..] => DI{ins: LDReadHL{dest: E}, cycles: 8, advance: 1, flags: None},

        [0x60, ..] => DI{ins: LDMove8{dest: H, src: B}, cycles: 4, advance: 1, flags: None},
        [0x61, ..] => DI{ins: LDMove8{dest: H, src: C}, cycles: 4, advance: 1, flags: None},
        [0x62, ..] => DI{ins: LDMove8{dest: H, src: D}, cycles: 4, advance: 1, flags: None},
        [0x63, ..] => DI{ins: LDMove8{dest: H, src: E}, cycles: 4, advance: 1, flags: None},
        [0x64, ..] => DI{ins: LDMove8{dest: H, src: H}, cycles: 4, advance: 1, flags: None},
        [0x65, ..] => DI{ins: LDMove8{dest: H, src: L}, cycles: 4, advance: 1, flags: None},
        [0x66, ..] => DI{ins: LDReadHL{dest: H}, cycles: 8, advance: 1, flags: None},

        [0x68, ..] => DI{ins: LDMove8{dest: L, src: B}, cycles: 4, advance: 1, flags: None},
        [0x69, ..] => DI{ins: LDMove8{dest: L, src: C}, cycles: 4, advance: 1, flags: None},
        [0x6A, ..] => DI{ins: LDMove8{dest: L, src: D}, cycles: 4, advance: 1, flags: None},
        [0x6B, ..] => DI{ins: LDMove8{dest: L, src: E}, cycles: 4, advance: 1, flags: None},
        [0x6C, ..] => DI{ins: LDMove8{dest: L, src: H}, cycles: 4, advance: 1, flags: None},
        [0x6D, ..] => DI{ins: LDMove8{dest: L, src: L}, cycles: 4, advance: 1, flags: None},
        [0x6E, ..] => DI{ins: LDReadHL{dest: L}, cycles: 8, advance: 1, flags: None},

        [0x70, ..] => DI{ins: LDStoreHL{src: B}, cycles: 8, advance: 1, flags: None},
        [0x71, ..] => DI{ins: LDStoreHL{src: C}, cycles: 8, advance: 1, flags: None},
        [0x72, ..] => DI{ins: LDStoreHL{src: D}, cycles: 8, advance: 1, flags: None},
        [0x73, ..] => DI{ins: LDStoreHL{src: E}, cycles: 8, advance: 1, flags: None},
        [0x74, ..] => DI{ins: LDStoreHL{src: H}, cycles: 8, advance: 1, flags: None},
        [0x75, ..] => DI{ins: LDStoreHL{src: L}, cycles: 8, advance: 1, flags: None},
        [0x76, x, ..] => DI{ins: LDStoreImmediateHL{value: *x}, cycles: 12, advance: 2, flags: None},

        


        _ => panic!("unimplemented instruction")
    }
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
