use super::{Bytecode, OwnedBytecode};
use super::address::Address;
use super::instruction::Instruction;

#[derive(Clone)]
pub struct Builder(Vec<u8>);

impl Builder {
    pub fn new() -> Builder {
        Builder(Vec::new())
    }

    pub fn with_instructions(instructions: &[Instruction]) -> Builder {
        let mut builder = Builder::new();
        for i in instructions.iter() {
            builder.push_instruction(i);
        }
        builder
    }

    pub fn build(&self) -> Bytecode {
        Bytecode::from_bytes_unchecked(&self.0)
    }

    pub fn build_owned(self) -> OwnedBytecode {
        OwnedBytecode::from_bytes_unchecked(self.0)
    }

    pub fn current_address(&self) -> Address {
        let len = self.0.len();
        Address::new(len as u16)
    }

    pub fn push_instruction(&mut self, instruction: &Instruction) -> &mut Self {
        let opcode = instruction.opcode();
        self.push_byte(opcode as u8);
        use Instruction::*;
        match instruction {
            FailIfLessThan(n) | Capture(n) => self.push_byte(*n),
            Jump(addr) | JumpIfFail(addr) | JumpIfSuccess(addr) | Call(addr) => {
                self.push_address(*addr);
            },
            Byte(b) | NotByte(b) => self.push_byte(*b),
            Class(c) => self.push_byte(*c as u8),
            Literal(s) | Set(s) | NotSet(s) => {
                self.push_bytes(s.as_bytes());
                self.push_byte(0);
            },
            Range(b_min, b_max) => {
                self.push_byte(*b_min);
                self.push_byte(*b_max);
            }
            _ => ()
        };
        self
    }

    pub fn patch_jump(&mut self, jump_op: Address, address: Address) -> &mut Self {
        let bytes: [u8; 2] = address.into();
        let jump_op: usize = jump_op.into();
        self.0[jump_op + 1] = bytes[0];
        self.0[jump_op + 2] = bytes[1];
        self
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    fn push_byte(&mut self, byte: u8) {
        self.0.push(byte);
    }

    fn push_bytes(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }

    fn push_address(&mut self, address: Address) {
        let bytes: [u8; 2] = address.into();
        self.push_bytes(&bytes);
    }
}

