use crate::instruction::*;

pub struct Bytecode(Vec<u8>);

pub const VERSION: i32 = 1;

impl Bytecode {
    pub fn new() -> Bytecode {
        Bytecode(Vec::new())
    }

    pub fn push_instruction(&mut self, instruction: &Instruction) -> usize {
        let len = self.0.len();
        let opcode = instruction.opcode();
        self.push_byte(opcode as u8);
        use Instruction::*;
        match instruction {
            FailIfLessThan(n) => self.push_byte(*n),
            Jump(addr) | JumpIfFail(addr) | JumpIfSuccess(addr) | Call(addr) => {
                self.push_address(*addr)
            },
            Byte(b) | NotByte(b) => self.push_byte(*b),
            Class(c) => self.push_byte(*c as u8),
            Literal(s) | Set(s) => self.push_bytes(s.as_bytes()),
            Range(c_min, c_max) => {
                self.push_char(*c_min);
                self.push_char(*c_max)
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
        self.0.push(byte)
    }

    fn push_bytes(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes)
    }

    fn push_char(&mut self, c: char) {
        let mut buffer = [0u8; 4];
        let bytes = c.encode_utf8(&mut buffer).as_bytes();
        self.push_bytes(bytes);
    }

    fn push_address(&mut self, address: Address) {
        let bytes: [u8; 2] = address.into();
        self.push_bytes(&bytes)
    }
}

