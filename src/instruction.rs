use crate::expression::CharClass;
use crate::opcode::Opcode;

use std::convert::{From, TryFrom};
use std::cmp::Ordering;
use std::iter::Iterator;
use std::ops::{Add, Sub};

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

impl Add<u16> for Address {
    type Output = Self;

    fn add(self, other: u16) -> Self::Output {
        Address(self.0 + other)
    }
}

impl Sub<u16> for Address {
    type Output = Self;

    fn sub(self, other: u16) -> Self::Output {
        Address(self.0 - other)
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

impl From<Address> for usize {
    fn from(address: Address) -> usize {
        address.0 as usize
    }
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

pub struct InstructionIterator<'a> {
    bytes: &'a [u8],
    current: usize,
}

impl<'a> InstructionIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> InstructionIterator {
        InstructionIterator { bytes: bytes, current: 0 }
    }

    pub fn jump(&mut self, address: Address) {
        self.current = usize::from(address);
    }
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        use Instruction::*;
        if self.current >= self.bytes.len() {
            None
        }
        else {
            let opcode = Opcode::try_from(self.bytes[self.current]).unwrap();
            self.current += 1;
            Some(Nop)
        }
    }
}

