

use super::Reg8;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Instruction {
    LD8Immediate{dest: Reg8, value: u8},
    LDMove8{dest: Reg8, src: Reg8},
    LDReadHL{dest: Reg8},
    LDStoreHL{src: Reg8},
    LDStoreImmediateHL{value: u8},
}
