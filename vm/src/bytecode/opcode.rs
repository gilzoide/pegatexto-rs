use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Nop,
    Succeed,
    Fail,
    ToggleSuccess,
    QuantifierInit,
    QuantifierLeast,
    QuantifierExact,
    Jump,
    JumpIfFail,
    JumpIfSuccess,
    Call,
    Return,
    Push,
    Peek,
    Pop,
    Byte,
    NotByte,
    Class,
    Literal,
    Set,
    Range,
    Action,
    Halt,
}

const OPCODE_TABLE: [Opcode; 23] = [
    Opcode::Nop,
    Opcode::Succeed,
    Opcode::Fail,
    Opcode::ToggleSuccess,
    Opcode::QuantifierInit,
    Opcode::QuantifierLeast,
    Opcode::QuantifierExact,
    Opcode::Jump,
    Opcode::JumpIfFail,
    Opcode::JumpIfSuccess,
    Opcode::Call,
    Opcode::Return,
    Opcode::Push,
    Opcode::Peek,
    Opcode::Pop,
    Opcode::Byte,
    Opcode::NotByte,
    Opcode::Class,
    Opcode::Literal,
    Opcode::Set,
    Opcode::Range,
    Opcode::Action,
    Opcode::Halt,
];

const OPCODE_ASSEMBLY_TABLE: [&str; 23] = [
    "nop",
    "succ",
    "fail",
    "togl",
    "qinit",
    "qleast",
    "qexact",
    "jmp",
    "jmpf",
    "jmps",
    "call",
    "ret",
    "push",
    "peek",
    "pop",
    "byte",
    "nbyte",
    "cls",
    "str",
    "set",
    "rng",
    "act",
    "halt",
];

pub struct TryFromByteError;

impl TryFrom<u8> for Opcode {
    type Error = TryFromByteError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let index = value as usize;
        if index >= OPCODE_TABLE.len() {
            Err(TryFromByteError)
        }
        else {
            Ok(OPCODE_TABLE[index])
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index = *self as usize;
        write!(f, "{}", OPCODE_ASSEMBLY_TABLE[index])
    }
}

