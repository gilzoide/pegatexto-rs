use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Nop,
    Succeed,
    Fail,
    FailIfLessThan,
    ToggleSuccess,
    QcZero,
    QcIncrement,
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
}

const OPCODE_TABLE: [Opcode; 22] = [
    Opcode::Nop,
    Opcode::Succeed,
    Opcode::Fail,
    Opcode::FailIfLessThan,
    Opcode::ToggleSuccess,
    Opcode::QcZero,
    Opcode::QcIncrement,
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
];

const OPCODE_ASSEMBLY_TABLE: [&'static str; 22] = [
    "nop",
    "succ",
    "fail",
    "flt",
    "togl",
    "qcz",
    "qci",
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
];

impl TryFrom<u8> for Opcode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let index = value as usize;
        if index >= OPCODE_TABLE.len() {
            Err("Invalid opcode byte found")
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

