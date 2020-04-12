use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Any,
    Succeed,
    Fail,
    FailIfLessThan,
    ToggleSuccess,
    QuantifierInit,
    QuantifierNext,
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
    NotSet,
    Range,
    Capture,
    Halt,
}

const OPCODE_TABLE: [Opcode; 24] = [
    Opcode::Any,
    Opcode::Succeed,
    Opcode::Fail,
    Opcode::FailIfLessThan,
    Opcode::ToggleSuccess,
    Opcode::QuantifierInit,
    Opcode::QuantifierNext,
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
    Opcode::NotSet,
    Opcode::Range,
    Opcode::Capture,
    Opcode::Halt,
];

const OPCODE_ASSEMBLY_TABLE: [&str; 24] = [
    "any",
    "succ",
    "fail",
    "flt",
    "togl",
    "qinit",
    "qnext",
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
    "nset",
    "rng",
    "cap",
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

