pub mod address;
pub mod builder;
pub mod instruction;
pub mod opcode;
pub mod parser;

use builder::Builder;
use instruction::Instruction;

use std::ops::Deref;

pub struct Bytecode<'a>(&'a [u8]);

impl<'a> Bytecode<'_> {
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Bytecode<'a>, parser::ParseError> {
        match Bytecode::check_error(bytes) {
            Some(err) => Err(err),
            None => Ok(Bytecode(bytes)),
        }
    }

    pub fn from_bytes_unchecked(bytes: &'a [u8]) -> Bytecode<'a> {
        Bytecode(bytes)
    }

    pub fn check_error(bytes: &[u8]) -> Option<parser::ParseError> {
        let mut iter = instruction::InstructionIterator::new(bytes);
        let instruction_is_halt_with_error = |i| match i {
            instruction::Instruction::Halt(Some(err)) => Some(err),
            _ => None,
        };
        iter.find_map(instruction_is_halt_with_error)
    }
}

impl<'a> Deref for Bytecode<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Clone)]
pub struct OwnedBytecode(Vec<u8>);

impl OwnedBytecode {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<OwnedBytecode, parser::ParseError> {
        match Bytecode::check_error(&bytes) {
            Some(err) => Err(err),
            None => Ok(OwnedBytecode(bytes)),
        }
    }

    pub fn from_bytecode(bytecode: &Bytecode) -> OwnedBytecode {
        OwnedBytecode::from_bytes_unchecked(bytecode.to_vec())
    }

    pub fn from_bytes_unchecked(bytes: Vec<u8>) -> OwnedBytecode {
        OwnedBytecode(bytes)
    }

    pub fn from_instructions(instructions: &[Instruction]) -> OwnedBytecode {
        Builder::with_instructions(instructions).build_owned()
    }

    pub fn as_bytecode(&self) -> Bytecode<'_> {
        Bytecode::from_bytes_unchecked(&self.0)
    }
}

