use crate::address::Address;
use crate::character_class::{self, CharacterClass};
use crate::opcode::{self, Opcode};

use std::array;
use std::iter::Iterator;
use std::convert::{From, TryFrom, TryInto};

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
    Class(CharacterClass),
    Literal(&'a str),
    Set(&'a str),
    Range(char, char),
    Action,
    Halt,
}

pub enum ParseError {
    EmptyChunk,
    InvalidOpcode,
    InvalidCharacterClass,
    MissingArgument,
}

impl From<opcode::TryFromByteError> for ParseError {
    fn from(_: opcode::TryFromByteError) -> ParseError {
        ParseError::InvalidOpcode
    }
}
impl From<array::TryFromSliceError> for ParseError {
    fn from(_: array::TryFromSliceError) -> ParseError {
        ParseError::MissingArgument
    }
}
impl From<character_class::TryFromU8> for ParseError {
    fn from(_: character_class::TryFromU8) -> ParseError {
        ParseError::InvalidCharacterClass
    }
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
            Halt => panic!("Halt instruction has no opcode")
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

    fn parse_byte_argument(bytes: &[u8]) -> Result<u8, ParseError> {
        match bytes.get(1) {
            Some(b) => Ok(*b),
            None => Err(ParseError::MissingArgument),
        }
    }

    fn parse_address_argument(bytes: &[u8]) -> Result<Address, ParseError> {
        let first_bytes: [u8; 2] = bytes[1..].try_into()?;
        Ok(Address::from(first_bytes))
    }

    fn parse(bytes: &[u8]) -> Result<(Instruction, usize), ParseError> {
        use Instruction::*;
        let opcode = match bytes.get(0) {
            Some(byte) => Opcode::try_from(*byte)?,
            None => return Err(ParseError::EmptyChunk),
        };
        match opcode {
            Opcode::Nop => Ok((Nop, 1)),
            Opcode::Succeed => Ok((Succeed, 1)),
            Opcode::Fail => Ok((Fail, 1)),
            Opcode::FailIfLessThan => {
                let n = InstructionIterator::parse_byte_argument(bytes)?;
                Ok((FailIfLessThan(n), 2))
            },
            Opcode::ToggleSuccess => Ok((ToggleSuccess, 1)),
            Opcode::QcZero => Ok((QcZero, 1)),
            Opcode::QcIncrement => Ok((QcIncrement, 1)),
            Opcode::Jump => {
                let address = InstructionIterator::parse_address_argument(bytes)?;
                Ok((Jump(address), 3))
            },
            Opcode::JumpIfFail => {
                let address = InstructionIterator::parse_address_argument(bytes)?;
                Ok((JumpIfFail(address), 3))
            },
            Opcode::JumpIfSuccess => {
                let address = InstructionIterator::parse_address_argument(bytes)?;
                Ok((JumpIfSuccess(address), 3))
            },
            Opcode::Call => {
                let address = InstructionIterator::parse_address_argument(bytes)?;
                Ok((Call(address), 3))
            },
            Opcode::Return => Ok((Return, 1)),
            Opcode::Push => Ok((Push, 1)),
            Opcode::Peek => Ok((Peek, 1)),
            Opcode::Pop => Ok((Pop, 1)),
            Opcode::Byte => {
                let b = InstructionIterator::parse_byte_argument(bytes)?;
                Ok((Byte(b), 2))
            },
            Opcode::NotByte => {
                let b = InstructionIterator::parse_byte_argument(bytes)?;
                Ok((NotByte(b), 2))
            },
            Opcode::Class => {
                let n = InstructionIterator::parse_byte_argument(bytes)?;
                let char_class: CharacterClass = n.try_into()?;
                Ok((Class(char_class), 2))
            }
            //Opcode::Literal => Ok((Literal, 1)),
            //Opcode::Set => Ok((Set, 1)),
            //Opcode::Range => Ok((Range, 1)),
            //Opcode::Action => Ok((Action, 1)),
            _ => Ok((Nop, 1))
        }
    }
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.bytes.len() {
            None
        }
        else {
            let parse_result = InstructionIterator::parse(self.bytes);
            match parse_result {
                Ok((instruction, increment)) => {
                    self.current += increment;
                    Some(instruction)
                },
                Err(_error) => {
                    self.current = self.bytes.len();
                    Some(Instruction::Halt)
                }
            }
        }
    }
}

