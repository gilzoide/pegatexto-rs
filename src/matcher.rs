use crate::bytecode::Bytecode;
use crate::bytecode::instruction::{Instruction, InstructionIterator};

pub enum MatchError {
    NoMatch,
}

struct MatchState {
    sp: usize,
    qc: i32,
    ac: i32,
}

pub fn try_match(bytecode: &Bytecode, text: &str) -> Result<usize, MatchError> {
    let mut success_flag = true;
    let mut sp = 0_usize;
    let mut qc = 0;
    let mut ac = 0;

    let mut state_stack = vec![MatchState { sp, qc, ac }];

    fn get_next_byte(text_slice: &str) -> Result<u8, MatchError> {
        match text_slice.as_bytes().get(0) {
            Some(b) => Ok(*b),
            None => Err(MatchError::NoMatch),
        }
    }
    fn get_next_char(text_slice: &str) -> Result<char, MatchError> {
        match text_slice.chars().next() {
            Some(c) => Ok(c),
            None => Err(MatchError::NoMatch),
        }
    }

    let mut iter = InstructionIterator::new(&bytecode);
    while let Some(instruction) = iter.next() {
        let text_slice = &text[sp..];
        match instruction {
            Instruction::Nop => (),
            Instruction::Succeed => {
                success_flag = true;
            },
            Instruction::Fail => {
                success_flag = false;
            },
            Instruction::FailIfLessThan(n) => {
                success_flag = qc >= n as i32;
            },
            Instruction::ToggleSuccess => {
                success_flag = !success_flag;
            },
            Instruction::QcZero => {
                qc = 0;
            },
            Instruction::QcIncrement => {
                qc += 1;
            },
            Instruction::Jump(addr) => {
                iter.jump(addr);
            },
            Instruction::JumpIfFail(addr) => {
                if !success_flag {
                    iter.jump(addr);
                }
            },
            Instruction::JumpIfSuccess(addr) => {
                if success_flag {
                    iter.jump(addr);
                }
            },
            Instruction::Call(_) => {
                // TODO
            },
            Instruction::Return => {
                // TODO
            },
            Instruction::Push => {
                state_stack.push(MatchState { sp, qc, ac });
            },
            Instruction::Peek => {
                match state_stack.last() {
                    Some(state) => {
                        sp = state.sp;
                        qc = state.qc;
                        ac = state.ac;
                    },
                    None => (),
                }
            },
            Instruction::Pop => {
                state_stack.pop();
            },
            Instruction::Byte(b) => {
                let next_byte = get_next_byte(text_slice)?;
                success_flag = next_byte == b;
                if success_flag {
                    sp += 1;
                }
            },
            Instruction::NotByte(b) => {
                let next_byte = get_next_byte(text_slice)?;
                success_flag = next_byte != b;
                if success_flag {
                    sp += 1;
                }
            },
            Instruction::Class(c) => {
                let next_char = get_next_char(text_slice)?;
                success_flag = c.is_member(next_char);
                if success_flag {
                    sp += next_char.len_utf8();
                }
            },
            Instruction::Literal(s) => {
                success_flag = text_slice.starts_with(s);
                if success_flag {
                    sp += s.len();
                }
            },
            Instruction::Set(s) => {
                let next_char = get_next_char(text_slice)?;
                success_flag = s.contains(next_char);
                if success_flag {
                    sp += 1;
                }
            },
            Instruction::Range(b_min, b_max) => {
                let next_byte = get_next_byte(text_slice)?;
                success_flag = next_byte >= b_min && next_byte <= b_max;
                if success_flag {
                    sp += 1;
                }
            },
            Instruction::Action => {
                // TODO
            },
            Instruction::Halt(_opt_err) => break,
        }
    }
    if success_flag {
        Ok(sp)
    }
    else {
        Err(MatchError::NoMatch)
    }
}
