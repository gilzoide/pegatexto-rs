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

#[derive(Clone, Copy, Debug, Default)]
struct MatchCapture {
    start: usize,
    end: usize,
    argc: i32,
    id: u8,
}

pub fn try_match(bytecode: &Bytecode, text: &str) -> Result<usize, MatchError> {
    try_match_then(bytecode, text, |_, _, _| ()).map(|p| p.0)
}

pub fn try_match_then<F, T>(bytecode: &Bytecode, text: &str, action: F) -> Result<(usize, Option<T>), MatchError> 
where
    F: Fn(&str, u8, &[T]) -> T
{
    let mut success_flag = true;

    let mut state = MatchState { sp: 0, qc: 0, ac: 0, ip: Address::new(0) };
    let mut state_stack = Vec::new();

    let mut capture_stack = Vec::new();

    let mut iter = InstructionIterator::new(&bytecode);

    fn get_next_byte(text_slice: &str) -> Option<u8> {
        text_slice.as_bytes().get(0).copied()
    }
    fn get_next_char(text_slice: &str) -> Option<char> {
        text_slice.chars().next()
    }
    macro_rules! push {
        () => {
            {
                let state = MatchState {
                    ip: iter.current(),
                    ac: capture_stack.len() as i32,
                    ..state
                };
                //println!(">> Push {:?}", state);
                state_stack.push(state);
                state
            }
        }
    }
    macro_rules! peek {
        () => {
            state_stack.last().copied().ok_or(MatchError::UnmatchedPop)
        }
    }
    macro_rules! pop {
        () => {
            state_stack.pop().ok_or(MatchError::UnmatchedPop)
        }
    }
    macro_rules! jump {
        ($addr:expr) => {
            iter.jump($addr);
        }
    }
    macro_rules! match_some {
        ($opt_len:expr) => {
            success_flag = match $opt_len {
                Some(len) => {
                    state.sp += len;
                    true
                },
                None => false,
            }
        }
    }

    while let Some(instruction) = iter.next() {
        //println!("  {}", instruction);
        let text_slice = &text[state.sp..];
        match instruction {
            Instruction::Any => {
                match_some!(get_next_char(text_slice)
                    .map(char::len_utf8));
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
                state = push!();
                state.qc = 0;
            },
            Instruction::QuantifierNext => {
                if success_flag {
                    state.qc += 1;
                    jump!(state.ip);
                }
            },
            Instruction::Jump(addr) => {
                jump!(addr);
            },
            Instruction::JumpIfFail(addr) => {
                if !success_flag {
                    jump!(addr);
                }
            },
            Instruction::JumpIfSuccess(addr) => {
                if success_flag {
                    jump!(addr);
                }
            },
            Instruction::Call(addr) => {
                state = push!();
                jump!(addr);
            },
            Instruction::Return => {
                match pop!() {
                    Ok(s) => jump!(s.ip),
                    Err(_) => break,
                }
            },
            Instruction::Push => {
                state = push!();
            },
            Instruction::Peek => {
                state = peek!()?;
                capture_stack.truncate(state.ac as usize);
            },
            Instruction::Pop => {
                pop!()?;
            },
            Instruction::Byte(b) => {
                match_some!(get_next_byte(text_slice)
                    .filter(|&next_byte| next_byte == b)
                    .and(Some(1)));
            },
            Instruction::Char(c) => {
                match_some!(get_next_char(text_slice)
                    .filter(|&next_char| next_char == c)
                    .map(char::len_utf8));
            },
            Instruction::Class(cls) => {
                match_some!(get_next_char(text_slice)
                    .filter(|&c| cls.is_member(c))
                    .map(char::len_utf8));
            },
            Instruction::Literal(s) => {
                match_some!(Some(text_slice)
                    .filter(|text_slice| text_slice.starts_with(s))
                    .map(str::len));
            },
            Instruction::Set(s) => {
                match_some!(get_next_char(text_slice)
                    .filter(|&c| s.contains(c))
                    .map(char::len_utf8));
            },
            Instruction::NotSet(s) => {
                match_some!(get_next_char(text_slice)
                    .filter(|&c| !s.contains(c))
                    .map(char::len_utf8));
            },
            Instruction::Range(b_min, b_max) => {
                match_some!(get_next_byte(text_slice)
                    .filter(|&next_byte| next_byte >= b_min && next_byte <= b_max)
                    .and(Some(1)));
            },
            Instruction::Capture(i) => {
                let previous_state = peek!()?;
                let capture = MatchCapture {
                    start: previous_state.sp,
                    end: state.sp,
                    argc: state.ac - previous_state.ac,
                    id: i,
                };
                capture_stack.push(capture);
                //println!("== Capture {:?} (ac: {})", &text[capture.start..capture.end], capture.argc);
            },
            Instruction::Halt(_opt_err) => break,
        }
    }

    if success_flag {
        //println!("MATCHED {:?}", capture_stack);
        let action_result = run_action_on(text, &capture_stack, action);
        Ok((state.sp, action_result))
    }
    else {
        Err(MatchError::NoMatch)
    }
}

fn run_action_on<F, T>(text: &str, captures: &[MatchCapture], action: F) -> Option<T>
where
    F: Fn(&str, u8, &[T]) -> T
{
    let num_captures = captures.len();
    if num_captures == 0 {
        return None;
    }

    let mut data_index: usize = 0;
    let mut data_stack = Vec::new();
    for capture in captures.iter() {
        let argc = capture.argc as usize;
        // "pop" arguments
        data_index -= argc;
        // run action with arguments (which are still stacked in `data_stack` in the right position)
        let value = action(
            &text[capture.start .. capture.end],
            capture.id,
            &data_stack[data_index .. data_index + argc]
        );
        data_stack.truncate(data_index);
        data_stack.push(value);
        // "push" result
        data_index += 1;
    }
    Some(data_stack.swap_remove(0))
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

    #[test]
    fn test_set() {
        let set = OwnedBytecode::from_instructions(&[Set("1234")]);
        let set = set.as_bytecode();
        test_match!(&set, "", Err(NoMatch));
        test_match!(&set, "0", Err(NoMatch));
        test_match!(&set, "1", Ok(1));
        test_match!(&set, "2", Ok(1));
        test_match!(&set, "3", Ok(1));
        test_match!(&set, "4", Ok(1));
        test_match!(&set, "5", Err(NoMatch));
    }

    #[test]
    fn test_inverse_set() {
        let set = OwnedBytecode::from_instructions(&[NotSet("1234")]);
        let set = set.as_bytecode();
        test_match!(&set, "", Err(NoMatch));
        test_match!(&set, "0", Ok(1));
        test_match!(&set, "1", Err(NoMatch));
        test_match!(&set, "2", Err(NoMatch));
        test_match!(&set, "3", Err(NoMatch));
        test_match!(&set, "4", Err(NoMatch));
        test_match!(&set, "5", Ok(1));
    }
}
