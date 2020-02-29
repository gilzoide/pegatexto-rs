use super::Bytecode;
use super::address::Address;
use super::instruction::*;

#[derive(Clone)]
pub struct Builder(Vec<u8>);

impl Builder {
    pub fn new() -> Builder {
        Builder(Vec::new())
    }

    pub fn build(self) -> Bytecode {
        Bytecode::from_bytes_unchecked(self.0)
    }

    pub fn push_instruction(&mut self, instruction: &Instruction) -> usize {
        let len = self.0.len();
        let opcode = instruction.opcode();
        self.push_byte(opcode as u8);
        use Instruction::*;
        match instruction {
            FailIfLessThan(n) => self.push_byte(*n),
            Jump(addr) | JumpIfFail(addr) | JumpIfSuccess(addr) | Call(addr) => {
                self.push_address(*addr);
            },
            Byte(b) | NotByte(b) => self.push_byte(*b),
            Class(c) => self.push_byte(*c as u8),
            Literal(s) | Set(s) => {
                self.push_bytes(s.as_bytes());
                self.push_byte(0);
            },
            Range(b_min, b_max) => {
                self.push_byte(*b_min);
                self.push_byte(*b_max);
            }
            _ => ()
        };
        len
    }

    pub fn patch_jump(&mut self, jump_op: usize, address: Address) {
        let bytes: [u8; 2] = address.into();
        self.0[jump_op + 1] = bytes[0];
        self.0[jump_op + 2] = bytes[1];
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

