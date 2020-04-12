use crate::bytecode::Bytecode;
use crate::bytecode::address::Address;
use crate::bytecode::instruction::{Instruction, InstructionIterator};

#[derive(Debug, PartialEq)]
pub enum MatchError {
    NoMatch,
    UnmatchedPop,
}

#[derive(Clone, Copy, Debug)]
struct MatchState {
    sp: usize,
    qc: i32,
    ac: i32,
    ip: Address,
}

pub fn try_match(bytecode: &Bytecode, text: &str) -> Result<usize, MatchError> {
    let mut success_flag = true;

    let mut state = MatchState { sp: 0, qc: 0, ac: 0, ip: Address::new(0) };
    let mut state_stack = Vec::new();

    fn get_next_byte(text_slice: &str) -> Option<u8> {
        text_slice.as_bytes().get(0).copied()
    }
    fn get_next_char(text_slice: &str) -> Option<char> {
        text_slice.chars().next()
    }
    fn push(state_stack: &mut Vec<MatchState>, mut state: MatchState, ip: Address) -> MatchState {
        state.ip = ip;
        println!(">> Push {:?}", state);
        state_stack.push(state);
        state
    }
    fn peek(state_stack: &Vec<MatchState>) -> Result<MatchState, MatchError> {
        match state_stack.last() {
            Some(state) => Ok(*state),
            None => Err(MatchError::UnmatchedPop),
        }
    }
    fn pop(state_stack: &mut Vec<MatchState>) -> Result<MatchState, MatchError> {
        match state_stack.pop() {
            Some(s) => {
                println!("<< Pop {:?}", s); 
                Ok(s)
            },
            None => {
                println!("<< Pop empty");
                Err(MatchError::UnmatchedPop)
            },
        }
    }
    fn jump(iter: &mut InstructionIterator, addr: Address) {
        println!("== Jump {:?}", addr);
        iter.jump(addr);
    }

    let mut iter = InstructionIterator::new(&bytecode);
    while let Some(instruction) = iter.next() {
        println!("{}", instruction);
        let text_slice = &text[state.sp..];
        match instruction {
            Instruction::Any => {
                success_flag = get_next_byte(text_slice).is_some();
                if success_flag {
                    state.sp += 1;
                }
            },
            Instruction::Succeed => {
                success_flag = true;
            },
            Instruction::Fail => {
                success_flag = false;
            },
            Instruction::FailIfLessThan(n) => {
                success_flag = state.qc >= n as i32;
            },
            Instruction::ToggleSuccess => {
                success_flag = !success_flag;
            },
            Instruction::QuantifierInit => {
                state = push(&mut state_stack, state, iter.current());
                state.qc = 0;
            },
            Instruction::QuantifierNext => {
                if success_flag {
                    state.qc += 1;
                    jump(&mut iter, state.ip);
                }
            },
            Instruction::Jump(addr) => {
                jump(&mut iter, addr);
            },
            Instruction::JumpIfFail(addr) => {
                if !success_flag {
                    jump(&mut iter, addr);
                }
            },
            Instruction::JumpIfSuccess(addr) => {
                if success_flag {
                    jump(&mut iter, addr);
                }
            },
            Instruction::Call(addr) => {
                state = push(&mut state_stack, state, iter.current());
                jump(&mut iter, addr);
            },
            Instruction::Return => {
                match pop(&mut state_stack) {
                    Ok(s) => jump(&mut iter, s.ip),
                    Err(_) => break,
                }
            },
            Instruction::Push => {
                state = push(&mut state_stack, state, iter.current());
            },
            Instruction::Peek => {
                state = peek(&state_stack)?;
            },
            Instruction::Pop => {
                pop(&mut state_stack)?;
            },
            Instruction::Byte(b) => {
                success_flag = match get_next_byte(text_slice) {
                    Some(next_byte) => next_byte == b,
                    None => false,
                };
                if success_flag {
                    state.sp += 1;
                }
            },
            Instruction::NotByte(b) => {
                success_flag = match get_next_byte(text_slice) {
                    Some(next_byte) => next_byte != b,
                    None => false,
                };
                if success_flag {
                    state.sp += 1;
                }
            },
            Instruction::Class(c) => {
                success_flag = match get_next_char(text_slice) {
                    Some(next_char) => {
                        if c.is_member(next_char) {
                            state.sp += next_char.len_utf8();
                            true
                        }
                        else {
                            false
                        }
                    },
                    None => false,
                };
            },
            Instruction::Literal(s) => {
                success_flag = text_slice.starts_with(s);
                if success_flag {
                    state.sp += s.len();
                }
            },
            Instruction::Set(s) => {
                success_flag = match get_next_char(text_slice) {
                    Some(next_char) => s.contains(next_char),
                    None => false,
                };
                if success_flag {
                    state.sp += 1;
                }
            },
            Instruction::Range(b_min, b_max) => {
                success_flag = match get_next_byte(text_slice) {
                    Some(next_byte) => next_byte >= b_min && next_byte <= b_max,
                    None => false,
                };
                if success_flag {
                    state.sp += 1;
                }
            },
            Instruction::Action => {
                // TODO
            },
            Instruction::Halt(_opt_err) => break,
        }
    }

    if success_flag {
        Ok(state.sp)
    }
    else {
        Err(MatchError::NoMatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;
    use crate::bytecode::OwnedBytecode;
    use crate::bytecode::builder::Builder;
    use crate::matcher::MatchError::*;

    macro_rules! test_match {
        ($bytecode:expr, $str:expr, $result:expr) => {
            assert_eq!(try_match($bytecode, $str), $result)
        }
    }

    #[test]
    fn test_match_any() {
        let any = OwnedBytecode::from_instructions(&[Any]);
        let any = any.as_bytecode();
        test_match!(&any, ".", Ok(1));
        test_match!(&any, "\u{0}", Ok(1));
        test_match!(&any, "", Err(NoMatch));
    }
}
