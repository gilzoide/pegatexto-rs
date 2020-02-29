pub mod address;
pub mod builder;
pub mod instruction;
pub mod opcode;
pub mod parser;

#[derive(Clone)]
pub struct Bytecode(Vec<u8>);

impl Bytecode {
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Bytecode, parser::ParseError> {
        let mut iter = instruction::InstructionIterator::new(&bytes);
        let instruction_is_halt_with_error = |i| match i {
            instruction::Instruction::Halt(Some(err)) => Some(err),
            _ => None,
        };
        match iter.find_map(instruction_is_halt_with_error) {
            Some(err) => Err(err),
            None => Ok(Bytecode(bytes)),
        }
    }

    pub fn from_bytes_unchecked(bytes: Vec<u8>) -> Bytecode {
        Bytecode(bytes)
    }
}

