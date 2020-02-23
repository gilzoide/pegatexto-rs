use crate::expression::CharClass;

use std::convert::From;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Ord)]
pub struct Address(u16);

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<[u8; 2]> for Address {
    fn from(bytes: [u8; 2]) -> Address {
        Address(u16::from_le_bytes(bytes))
    }
}

impl From<Address> for [u8; 2] {
    fn from(address: Address) -> [u8; 2] {
        address.0.to_le_bytes()
    }
}

#[derive(Debug)]
#[repr(u8)]
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

#[derive(Debug)]
pub enum Instruction<'a> {
    Nop,
    Succeed,
    Fail,
    FailIfLessThan(u8),
    ToggleSuccess,
    QcZero,
    QcIncrement,
    Jump(Address),
    JumpIfFail(Address),
    JumpIfSuccess(Address),
    Call(Address),
    Return,
    Push,
    Peek,
    Pop,
    Byte(u8),
    NotByte(u8),
    Class(CharClass),
    Literal(&'a str),
    Set(&'a str),
    Range(char, char),
    Action,
}

impl Instruction<'_> {
    pub fn opcode(&self) -> Opcode {
        use Instruction::*;
        match *self {
            Nop => Opcode::Nop,
            Succeed => Opcode::Succeed,
            Fail => Opcode::Fail,
            FailIfLessThan(_) => Opcode::FailIfLessThan,
            ToggleSuccess => Opcode::ToggleSuccess,
            QcZero => Opcode::QcZero,
            QcIncrement => Opcode::QcIncrement,
            Jump(_) => Opcode::Jump,
            JumpIfFail(_) => Opcode::JumpIfFail,
            JumpIfSuccess(_) => Opcode::JumpIfSuccess,
            Call(_) => Opcode::Call,
            Return => Opcode::Return,
            Push => Opcode::Push,
            Peek => Opcode::Peek,
            Pop => Opcode::Pop,
            Byte(_) => Opcode::Byte,
            NotByte(_) => Opcode::NotByte,
            Class(_) => Opcode::Class,
            Literal(_) => Opcode::Literal,
            Set(_) => Opcode::Set,
            Range(_, _) => Opcode::Range,
            Action => Opcode::Action,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Opcode::*;
        let assembly_code = match *self {
            Nop => "nop",
            Succeed => "succ",
            Fail => "fail",
            FailIfLessThan => "flt",
            ToggleSuccess => "togl",
            QcZero => "qcz",
            QcIncrement => "qci",
            Jump => "jmp",
            JumpIfFail => "jmpf",
            JumpIfSuccess => "jmps",
            Call => "call",
            Return => "ret",
            Push => "push",
            Peek => "peek",
            Pop => "pop",
            Byte => "byte",
            NotByte => "nbyte",
            Class => "cls",
            Literal => "str",
            Set => "set",
            Range => "rng",
            Action => "act",
        };
        write!(f, "{}", assembly_code)
    }
}
