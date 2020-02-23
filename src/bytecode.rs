use crate::instruction::*;

pub struct Bytecode(Vec<u8>);

pub const VERSION: i32 = 1;

impl Bytecode {
    pub fn new() -> Bytecode {
        Bytecode(Vec::new())
    }

    pub fn push_byte(&mut self, byte: u8) -> usize {
        let len = self.0.len();
        self.0.push(byte);
        len
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> usize {
        let len = self.0.len();
        self.0.extend_from_slice(bytes);
        len
    }

    pub fn push_jump(&mut self, opcode: Opcode, address: Address) -> usize {
        let len = self.0.len();
        self.0.push(opcode as u8);
        self.0.extend_from_slice(&address.to_le_bytes());
        len
    }
}
