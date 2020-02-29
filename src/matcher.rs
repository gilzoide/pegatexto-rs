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
                let str_byte = text_slice.as_bytes()[0];
                success_flag = str_byte == b;
                sp += success_flag as usize;
            },
            Instruction::NotByte(b) => {
                let str_byte = text_slice.as_bytes()[0];
                success_flag = str_byte != b;
                sp += success_flag as usize;
            },
            Instruction::Class(c) => {
                let str_char = text_slice.chars().next().unwrap();
                success_flag = c.is_member(str_char);
                if success_flag {
                    sp += str_char.len_utf8();
                }
            },
            Instruction::Literal(s) => {
                success_flag = text_slice.starts_with(s);
                if success_flag {
                    sp += s.len();
                }
            },
            Instruction::Set(s) => {
                let str_char = text_slice.chars().next().unwrap();
                success_flag = s.contains(str_char);
                sp += success_flag as usize;
            },
            Instruction::Range(b_min, b_max) => {
                let str_byte = text_slice.as_bytes()[0];
                success_flag = str_byte >= b_min && str_byte <= b_max;
                sp += success_flag as usize;
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
