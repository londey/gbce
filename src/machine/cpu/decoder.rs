
use super::Reg8;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Cycles(u8);

use super::Instruction;
use super::Instruction::*;


pub fn decode_next_instruction(instruction_stream: &[u8]) -> (Instruction, Cycles) {
    match instruction_stream {
        // sec 3.3.1.1
        [0x06, x, ..] => (LD8Immediate{dest: Reg8::B, value: *x}, Cycles(8)),
        [0x0E, x, ..] => (LD8Immediate{dest: Reg8::C, value: *x}, Cycles(8)),
        [0x16, x, ..] => (LD8Immediate{dest: Reg8::D, value: *x}, Cycles(8)),
        [0x1E, x, ..] => (LD8Immediate{dest: Reg8::E, value: *x}, Cycles(8)),
        [0x26, x, ..] => (LD8Immediate{dest: Reg8::H, value: *x}, Cycles(8)),
        [0x2E, x, ..] => (LD8Immediate{dest: Reg8::L, value: *x}, Cycles(8)),

        // sec 3.3.1.2
        [0x7F, ..] => (LDMove8{dest: Reg8::A, src: Reg8::A}, Cycles(4)),
        [0x78, ..] => (LDMove8{dest: Reg8::A, src: Reg8::B}, Cycles(4)),
        [0x79, ..] => (LDMove8{dest: Reg8::A, src: Reg8::C}, Cycles(4)),
        [0x7A, ..] => (LDMove8{dest: Reg8::A, src: Reg8::D}, Cycles(4)),
        [0x7B, ..] => (LDMove8{dest: Reg8::A, src: Reg8::E}, Cycles(4)),
        [0x7C, ..] => (LDMove8{dest: Reg8::A, src: Reg8::H}, Cycles(4)),
        [0x7D, ..] => (LDMove8{dest: Reg8::A, src: Reg8::L}, Cycles(4)),
        [0x7E, ..] => (LDReadHL{dest: Reg8::A}, Cycles(8)),

        [0x40, ..] => (LDMove8{dest: Reg8::B, src: Reg8::B}, Cycles(4)),
        [0x41, ..] => (LDMove8{dest: Reg8::B, src: Reg8::C}, Cycles(4)),
        [0x42, ..] => (LDMove8{dest: Reg8::B, src: Reg8::D}, Cycles(4)),
        [0x43, ..] => (LDMove8{dest: Reg8::B, src: Reg8::E}, Cycles(4)),
        [0x44, ..] => (LDMove8{dest: Reg8::B, src: Reg8::H}, Cycles(4)),
        [0x45, ..] => (LDMove8{dest: Reg8::B, src: Reg8::L}, Cycles(4)),
        [0x46, ..] => (LDReadHL{dest: Reg8::B}, Cycles(8)),

        [0x48, ..] => (LDMove8{dest: Reg8::C, src: Reg8::B}, Cycles(4)),
        [0x49, ..] => (LDMove8{dest: Reg8::C, src: Reg8::C}, Cycles(4)),
        [0x4A, ..] => (LDMove8{dest: Reg8::C, src: Reg8::D}, Cycles(4)),
        [0x4B, ..] => (LDMove8{dest: Reg8::C, src: Reg8::E}, Cycles(4)),
        [0x4C, ..] => (LDMove8{dest: Reg8::C, src: Reg8::H}, Cycles(4)),
        [0x4D, ..] => (LDMove8{dest: Reg8::C, src: Reg8::L}, Cycles(4)),
        [0x4E, ..] => (LDReadHL{dest: Reg8::C}, Cycles(8)),

        [0x50, ..] => (LDMove8{dest: Reg8::D, src: Reg8::B}, Cycles(4)),
        [0x51, ..] => (LDMove8{dest: Reg8::D, src: Reg8::C}, Cycles(4)),
        [0x52, ..] => (LDMove8{dest: Reg8::D, src: Reg8::D}, Cycles(4)),
        [0x53, ..] => (LDMove8{dest: Reg8::D, src: Reg8::E}, Cycles(4)),
        [0x54, ..] => (LDMove8{dest: Reg8::D, src: Reg8::H}, Cycles(4)),
        [0x55, ..] => (LDMove8{dest: Reg8::D, src: Reg8::L}, Cycles(4)),
        [0x56, ..] => (LDReadHL{dest: Reg8::D}, Cycles(8)),

        [0x58, ..] => (LDMove8{dest: Reg8::E, src: Reg8::B}, Cycles(4)),
        [0x59, ..] => (LDMove8{dest: Reg8::E, src: Reg8::C}, Cycles(4)),
        [0x5A, ..] => (LDMove8{dest: Reg8::E, src: Reg8::D}, Cycles(4)),
        [0x5B, ..] => (LDMove8{dest: Reg8::E, src: Reg8::E}, Cycles(4)),
        [0x5C, ..] => (LDMove8{dest: Reg8::E, src: Reg8::H}, Cycles(4)),
        [0x5D, ..] => (LDMove8{dest: Reg8::E, src: Reg8::L}, Cycles(4)),
        [0x5E, ..] => (LDReadHL{dest: Reg8::E}, Cycles(8)),

        [0x60, ..] => (LDMove8{dest: Reg8::H, src: Reg8::B}, Cycles(4)),
        [0x61, ..] => (LDMove8{dest: Reg8::H, src: Reg8::C}, Cycles(4)),
        [0x62, ..] => (LDMove8{dest: Reg8::H, src: Reg8::D}, Cycles(4)),
        [0x63, ..] => (LDMove8{dest: Reg8::H, src: Reg8::E}, Cycles(4)),
        [0x64, ..] => (LDMove8{dest: Reg8::H, src: Reg8::H}, Cycles(4)),
        [0x65, ..] => (LDMove8{dest: Reg8::H, src: Reg8::L}, Cycles(4)),
        [0x66, ..] => (LDReadHL{dest: Reg8::H}, Cycles(8)),

        [0x68, ..] => (LDMove8{dest: Reg8::L, src: Reg8::B}, Cycles(4)),
        [0x69, ..] => (LDMove8{dest: Reg8::L, src: Reg8::C}, Cycles(4)),
        [0x6A, ..] => (LDMove8{dest: Reg8::L, src: Reg8::D}, Cycles(4)),
        [0x6B, ..] => (LDMove8{dest: Reg8::L, src: Reg8::E}, Cycles(4)),
        [0x6C, ..] => (LDMove8{dest: Reg8::L, src: Reg8::H}, Cycles(4)),
        [0x6D, ..] => (LDMove8{dest: Reg8::L, src: Reg8::L}, Cycles(4)),
        [0x6E, ..] => (LDReadHL{dest: Reg8::L}, Cycles(8)),

        [0x70, ..] => (LDStoreHL{src: Reg8::B}, Cycles(8)),
        [0x71, ..] => (LDStoreHL{src: Reg8::C}, Cycles(8)),
        [0x72, ..] => (LDStoreHL{src: Reg8::D}, Cycles(8)),
        [0x73, ..] => (LDStoreHL{src: Reg8::E}, Cycles(8)),
        [0x74, ..] => (LDStoreHL{src: Reg8::H}, Cycles(8)),
        [0x75, ..] => (LDStoreHL{src: Reg8::L}, Cycles(8)),
        [0x76, x, ..] => (LDStoreImmediateHL{value: *x}, Cycles(12)),

        


        _ => panic!("unimplemented instruction")
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ld8imitate() {
        assert_eq!(decode_next_instruction(&[0x06u8, 42]), (LD8Immediate{dest: Reg8::B, value: 42}, Cycles(8)));
        assert_eq!(decode_next_instruction(&[0x0Eu8, 42]), (LD8Immediate{dest: Reg8::C, value: 42}, Cycles(8)));
        assert_eq!(decode_next_instruction(&[0x16u8, 42]), (LD8Immediate{dest: Reg8::D, value: 42}, Cycles(8)));
        assert_eq!(decode_next_instruction(&[0x1Eu8, 42]), (LD8Immediate{dest: Reg8::E, value: 42}, Cycles(8)));
        assert_eq!(decode_next_instruction(&[0x26u8, 42]), (LD8Immediate{dest: Reg8::H, value: 42}, Cycles(8)));
        assert_eq!(decode_next_instruction(&[0x2Eu8, 42]), (LD8Immediate{dest: Reg8::L, value: 42}, Cycles(8)));
    }
}
