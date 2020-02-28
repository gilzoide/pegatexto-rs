use super::address::Address;
use super::opcode::Opcode;
use super::parser::{self, ParseError};
use crate::character_class::CharacterClass;

use std::iter::Iterator;

#[derive(Debug, PartialEq)]
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
    Range(u8, u8),
    Action,
    Halt(ParseError),
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
            Halt(_) => panic!("Halt instruction has no opcode")
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
        if self.current >= self.bytes.len() {
            None
        }
        else {
            let slice = &self.bytes[self.current..];
            match parser::parse_instruction(slice) {
                Ok((instruction, increment)) => {
                    self.current += increment;
                    Some(instruction)
                },
                Err(error) => {
                    self.current = usize::max_value();
                    Some(Instruction::Halt(error))
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
        assert_eq!(iter.next().unwrap(), Instruction::Halt(ParseError::MissingArgument));
        assert_eq!(iter.next(), None);
    }
}
