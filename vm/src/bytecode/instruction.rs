use super::address::Address;
use super::opcode::Opcode;
use super::parser::{self, ParseError};
use crate::grammar::character_class::CharacterClass;

use std::fmt;
use std::iter::Iterator;

#[derive(Debug, PartialEq)]
pub enum Instruction<'a> {
    Nop,
    Succeed,
    Fail,
    ToggleSuccess,
    QuantifierInit,
    QuantifierLeast(u8),
    QuantifierExact(u8),
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
    Range(u8, u8),
    Action,
    Halt(Option<ParseError>),
}

impl Instruction<'_> {
    pub fn opcode(&self) -> Opcode {
        use Instruction::*;
        match *self {
            Nop => Opcode::Nop,
            Succeed => Opcode::Succeed,
            Fail => Opcode::Fail,
            ToggleSuccess => Opcode::ToggleSuccess,
            QuantifierInit => Opcode::QuantifierInit,
            QuantifierLeast(_) => Opcode::QuantifierLeast,
            QuantifierExact(_) => Opcode::QuantifierExact,
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
            Halt(_) => Opcode::Halt,
        }
    }
}

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        let res = write!(f, "{}", self.opcode());
        match *self {
            QuantifierLeast(n) | QuantifierExact(n) => write!(f, " {}", n),
            Jump(address) | JumpIfFail(address) | JumpIfSuccess(address) | Call(address) => {
                write!(f, " {}", address)
            },
            Byte(byte) | NotByte(byte) => write!(f, " '{}'", byte as char),
            Class(character_class) => write!(f, " \\{}", character_class as u8 as char),
            Literal(string) | Set(string) => write!(f, " {:?}", string),
            Range(min, max) => write!(f, " [{}-{}]", min as char, max as char),
            //Halt(_) => Opcode::Halt,
            _ => res
        }
    }
}

pub struct InstructionIterator<'a> {
    bytes: &'a [u8],
    current: Address,
}

impl<'a> InstructionIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> InstructionIterator {
        InstructionIterator { bytes: bytes, current: Address::new(0) }
    }

    pub fn jump(&mut self, address: Address) {
        self.current = address;
    }

    pub fn current(&self) -> Address {
        self.current
    }

    pub fn bytes_len(&self) -> usize {
        self.bytes.len()
    }
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let current: usize = self.current.into();
        if current >= self.bytes.len() {
            None
        }
        else {
            let slice = &self.bytes[current..];
            match parser::parse_instruction(slice) {
                Ok((instruction, increment)) => {
                    self.current += increment as u16;
                    Some(instruction)
                },
                Err(error) => {
                    self.current = Address::max_value();
                    Some(Instruction::Halt(Some(error)))
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        // \w*
        let bytecode = [
            Opcode::Class as u8, b'w',
            Opcode::JumpIfSuccess as u8, 0, 0,
            Opcode::Succeed as u8,
        ];
        let mut iter = InstructionIterator::new(&bytecode);
        assert_eq!(iter.next().unwrap(), Instruction::Class(CharacterClass::Alphanumeric));
        assert_eq!(iter.next().unwrap(), Instruction::JumpIfSuccess(Address::new(0)));
        assert_eq!(iter.next().unwrap(), Instruction::Succeed);
        assert_eq!(iter.next(), None);

        let empty_bytecode = [];
        let mut iter = InstructionIterator::new(&empty_bytecode);
        assert_eq!(iter.next(), None);

        let faulty_bytecode = [
            Opcode::Class as u8, b'w',
            Opcode::JumpIfSuccess as u8, 0,
        ];
        let mut iter = InstructionIterator::new(&faulty_bytecode);
        assert_eq!(iter.next().unwrap(), Instruction::Class(CharacterClass::Alphanumeric));
        assert_eq!(iter.next().unwrap(), Instruction::Halt(Some(ParseError::MissingArgument)));
        assert_eq!(iter.next(), None);
    }
}
