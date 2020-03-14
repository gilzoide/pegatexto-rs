use super::address::Address;
use super::instruction::Instruction;
use super::opcode::{self, Opcode};
use crate::slice_to_array;
use crate::character_class::{self, CharacterClass};

use std::array;
use std::convert::{From, TryFrom, TryInto};
use std::str;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyChunk,
    InvalidOpcode,
    InvalidCharacterClass,
    MissingStringTerminator,
    InvalidUtf8String,
    InvalidRange,
    MissingArgument,
}

impl From<opcode::TryFromByteError> for ParseError {
    fn from(_: opcode::TryFromByteError) -> ParseError {
        ParseError::InvalidOpcode
    }
}
impl From<array::TryFromSliceError> for ParseError {
    fn from(_: array::TryFromSliceError) -> ParseError {
        ParseError::MissingArgument
    }
}
impl From<character_class::TryFromU8> for ParseError {
    fn from(_: character_class::TryFromU8) -> ParseError {
        ParseError::InvalidCharacterClass
    }
}
impl From<str::Utf8Error> for ParseError {
    fn from(_: str::Utf8Error) -> ParseError {
        ParseError::InvalidUtf8String
    }
}

macro_rules! parse_instruction_byte {
    ($ctor:ident, $bytes:ident) => {
        {
            let b = parse_byte_argument($bytes)?;
            Ok(($ctor(b), 2))
        }
    }
}
macro_rules! parse_instruction_address {
    ($ctor:ident, $bytes:ident) => {
        {
            let address = parse_address_argument($bytes)?;
            Ok(($ctor(address), 3))
        }
    }
}
macro_rules! parse_instruction_character_class {
    ($ctor:ident, $bytes:ident) => {
        {
            let b = parse_byte_argument($bytes)?;
            let char_class: CharacterClass = b.try_into()?;
            Ok(($ctor(char_class), 2))
        }
    }
}
macro_rules! parse_instruction_string {
    ($ctor:ident, $bytes:ident) => {
        {
            let string = parse_string_argument($bytes)?;
            Ok(($ctor(string), 1 + string.len() + 1))
        }
    }
}
macro_rules! parse_instruction_range {
    ($ctor:ident, $bytes:ident) => {
        {
            let (b_min, b_max) = parse_range_argument($bytes)?;
            Ok(($ctor(b_min, b_max), 3))
        }
    }
}

fn parse_byte_argument(bytes: &[u8]) -> Result<u8, ParseError> {
    match bytes.get(0) {
        Some(b) => Ok(*b),
        None => Err(ParseError::MissingArgument),
    }
}

fn parse_address_argument(bytes: &[u8]) -> Result<Address, ParseError> {
    match slice_to_array!(bytes, u8, 2) {
        Some(two_byte_array) => Ok(Address::from(two_byte_array)),
        None => Err(ParseError::MissingArgument),
    }
}

fn parse_range_argument(bytes: &[u8]) -> Result<(u8, u8), ParseError> {
    match slice_to_array!(bytes, u8, 2) {
        Some([b_min, b_max]) => {
            if b_min < b_max {
                Ok((b_min, b_max))
            } else {
                Err(ParseError::InvalidRange)
            }
        },
        None => Err(ParseError::MissingArgument),
    }
}

fn parse_string_argument(bytes: &[u8]) -> Result<&str, ParseError> {
    if bytes.len() == 0 || bytes[0] == 0 {
        return Err(ParseError::MissingArgument)
    }
    let s = match bytes.iter().enumerate().skip_while(|(_, b)| **b != 0).next() {
        Some((size_until_null, _last_byte)) => {
            let slice = &bytes[0..size_until_null];
            str::from_utf8(slice)?
        },
        None => Err(ParseError::MissingStringTerminator)?,
    };
    Ok(s)
}

pub fn parse_instruction(bytes: &[u8]) -> Result<(Instruction, usize), ParseError> {
    use Instruction::*;
    let opcode = match bytes.get(0) {
        Some(byte) => Opcode::try_from(*byte)?,
        None => Err(ParseError::EmptyChunk)?,
    };
    let bytes = &bytes[1..];
    match opcode {
        Opcode::Nop => Ok((Nop, 1)),
        Opcode::Succeed => Ok((Succeed, 1)),
        Opcode::Fail => Ok((Fail, 1)),
        Opcode::FailIfLessThan => parse_instruction_byte!(FailIfLessThan, bytes),
        Opcode::ToggleSuccess => Ok((ToggleSuccess, 1)),
        Opcode::QcZero => Ok((QcZero, 1)),
        Opcode::QcIncrement => Ok((QcIncrement, 1)),
        Opcode::Jump => parse_instruction_address!(Jump, bytes),
        Opcode::JumpIfFail => parse_instruction_address!(JumpIfFail, bytes),
        Opcode::JumpIfSuccess => parse_instruction_address!(JumpIfSuccess, bytes),
        Opcode::Call => parse_instruction_address!(Call, bytes),
        Opcode::Return => Ok((Return, 1)),
        Opcode::Push => Ok((Push, 1)),
        Opcode::Peek => Ok((Peek, 1)),
        Opcode::Pop => Ok((Pop, 1)),
        Opcode::Byte => parse_instruction_byte!(Byte, bytes),
        Opcode::NotByte => parse_instruction_byte!(NotByte, bytes),
        Opcode::Class => parse_instruction_character_class!(Class, bytes),
        Opcode::Literal => parse_instruction_string!(Literal, bytes),
        Opcode::Set => parse_instruction_string!(Set, bytes),
        Opcode::Range => parse_instruction_range!(Range, bytes),
        Opcode::Action => Ok((Action, 1)),
        Opcode::Halt => Ok((Halt(None), 1)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse {
        ($array:expr, $result:expr) => {
            assert_eq!(parse_instruction(&$array), $result)
        }
    }

    #[test]
    fn test_instruction_parser() {
        test_parse!([Opcode::Nop as u8], Ok((Instruction::Nop, 1)));
        test_parse!([Opcode::Nop as u8, 1, 2, 3], Ok((Instruction::Nop, 1)));
        
        test_parse!([Opcode::Succeed as u8], Ok((Instruction::Succeed, 1)));
        
        test_parse!([Opcode::Fail as u8], Ok((Instruction::Fail, 1)));
        
        test_parse!([Opcode::FailIfLessThan as u8, 1], Ok((Instruction::FailIfLessThan(1), 2)));
        test_parse!([Opcode::FailIfLessThan as u8, 255], Ok((Instruction::FailIfLessThan(255), 2)));
        test_parse!([Opcode::FailIfLessThan as u8, 255, 0], Ok((Instruction::FailIfLessThan(255), 2)));
        test_parse!([Opcode::FailIfLessThan as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::ToggleSuccess as u8], Ok((Instruction::ToggleSuccess, 1)));
        
        test_parse!([Opcode::QcZero as u8], Ok((Instruction::QcZero, 1)));
        
        test_parse!([Opcode::QcIncrement as u8], Ok((Instruction::QcIncrement, 1)));

        test_parse!([Opcode::Jump as u8, 42, 0], Ok((Instruction::Jump(Address::new(42)), 3)));
        test_parse!([Opcode::Jump as u8, 42, 0, 255], Ok((Instruction::Jump(Address::new(42)), 3)));
        test_parse!([Opcode::Jump as u8, 0, 1], Ok((Instruction::Jump(Address::new(256)), 3)));
        test_parse!([Opcode::Jump as u8, 42], Err(ParseError::MissingArgument));
        test_parse!([Opcode::Jump as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::JumpIfFail as u8, 42, 0], Ok((Instruction::JumpIfFail(Address::new(42)), 3)));
        test_parse!([Opcode::JumpIfFail as u8, 42, 0, 255], Ok((Instruction::JumpIfFail(Address::new(42)), 3)));
        test_parse!([Opcode::JumpIfFail as u8, 0, 1], Ok((Instruction::JumpIfFail(Address::new(256)), 3)));
        test_parse!([Opcode::JumpIfFail as u8, 42], Err(ParseError::MissingArgument));
        test_parse!([Opcode::JumpIfFail as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::JumpIfSuccess as u8, 42, 0], Ok((Instruction::JumpIfSuccess(Address::new(42)), 3)));
        test_parse!([Opcode::JumpIfSuccess as u8, 42, 0, 255], Ok((Instruction::JumpIfSuccess(Address::new(42)), 3)));
        test_parse!([Opcode::JumpIfSuccess as u8, 0, 1], Ok((Instruction::JumpIfSuccess(Address::new(256)), 3)));
        test_parse!([Opcode::JumpIfSuccess as u8, 42], Err(ParseError::MissingArgument));
        test_parse!([Opcode::JumpIfSuccess as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::Call as u8, 42, 0], Ok((Instruction::Call(Address::new(42)), 3)));
        test_parse!([Opcode::Call as u8, 42, 0, 255], Ok((Instruction::Call(Address::new(42)), 3)));
        test_parse!([Opcode::Call as u8, 0, 1], Ok((Instruction::Call(Address::new(256)), 3)));
        test_parse!([Opcode::Call as u8, 42], Err(ParseError::MissingArgument));
        test_parse!([Opcode::Call as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::Return as u8], Ok((Instruction::Return, 1)));

        test_parse!([Opcode::Push as u8], Ok((Instruction::Push, 1)));
        
        test_parse!([Opcode::Peek as u8], Ok((Instruction::Peek, 1)));
        
        test_parse!([Opcode::Pop as u8], Ok((Instruction::Pop, 1)));

        test_parse!([Opcode::Byte as u8, 0], Ok((Instruction::Byte(0), 2)));
        test_parse!([Opcode::Byte as u8, 255], Ok((Instruction::Byte(255), 2)));
        test_parse!([Opcode::Byte as u8, 255, 0], Ok((Instruction::Byte(255), 2)));
        test_parse!([Opcode::Byte as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::NotByte as u8, 0], Ok((Instruction::NotByte(0), 2)));
        test_parse!([Opcode::NotByte as u8, 255], Ok((Instruction::NotByte(255), 2)));
        test_parse!([Opcode::NotByte as u8, 255, 0], Ok((Instruction::NotByte(255), 2)));
        test_parse!([Opcode::NotByte as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::Class as u8, b'a'], Ok((Instruction::Class(CharacterClass::Alphabetic), 2)));
        test_parse!([Opcode::Class as u8, b'w'], Ok((Instruction::Class(CharacterClass::Alphanumeric), 2)));
        test_parse!([Opcode::Class as u8, b'c'], Ok((Instruction::Class(CharacterClass::Control), 2)));
        test_parse!([Opcode::Class as u8, b'd'], Ok((Instruction::Class(CharacterClass::Digit), 2)));
        test_parse!([Opcode::Class as u8, b'g'], Ok((Instruction::Class(CharacterClass::Graphic), 2)));
        test_parse!([Opcode::Class as u8, b'l'], Ok((Instruction::Class(CharacterClass::Lowercase), 2)));
        test_parse!([Opcode::Class as u8, b'p'], Ok((Instruction::Class(CharacterClass::Punctuation), 2)));
        test_parse!([Opcode::Class as u8, b's'], Ok((Instruction::Class(CharacterClass::Whitespace), 2)));
        test_parse!([Opcode::Class as u8, b'u'], Ok((Instruction::Class(CharacterClass::Uppercase), 2)));
        test_parse!([Opcode::Class as u8, b'x'], Ok((Instruction::Class(CharacterClass::Hexadigit), 2)));
        test_parse!([Opcode::Class as u8, b'x', 0], Ok((Instruction::Class(CharacterClass::Hexadigit), 2)));
        test_parse!([Opcode::Class as u8, 0], Err(ParseError::InvalidCharacterClass));
        test_parse!([Opcode::Class as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::Literal as u8, b'h', b'e', b'l', b'l', b'o', 0], Ok((Instruction::Literal("hello"), 7)));
        test_parse!([Opcode::Literal as u8, b'n', b'o', b'n', b'u', b'l'], Err(ParseError::MissingStringTerminator));
        test_parse!([Opcode::Literal as u8, 159, 146, 150, 0], Err(ParseError::InvalidUtf8String));
        test_parse!([Opcode::Literal as u8, 0], Err(ParseError::MissingArgument));
        test_parse!([Opcode::Literal as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::Set as u8, b'h', b'e', b'l', b'o', 0], Ok((Instruction::Set("helo"), 6)));
        test_parse!([Opcode::Set as u8, b'!', b'n', b'u', b'l'], Err(ParseError::MissingStringTerminator));
        test_parse!([Opcode::Set as u8, 159, 146, 150, 0], Err(ParseError::InvalidUtf8String));
        test_parse!([Opcode::Set as u8, 0], Err(ParseError::MissingArgument));
        test_parse!([Opcode::Set as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::Range as u8, b'0', b'9'], Ok((Instruction::Range(b'0', b'9'), 3)));
        test_parse!([Opcode::Range as u8, b'0', b'9', 0], Ok((Instruction::Range(b'0', b'9'), 3)));
        test_parse!([Opcode::Range as u8, b'9', b'9'], Err(ParseError::InvalidRange));
        test_parse!([Opcode::Range as u8, b'9', b'0'], Err(ParseError::InvalidRange));
        test_parse!([Opcode::Range as u8, b'9'], Err(ParseError::MissingArgument));
        test_parse!([Opcode::Range as u8], Err(ParseError::MissingArgument));

        test_parse!([Opcode::Action as u8], Ok((Instruction::Action, 1)));

        test_parse!([Opcode::Halt as u8], Ok((Instruction::Halt(None), 1)));

        test_parse!([Opcode::Halt as u8 + 1], Err(ParseError::InvalidOpcode));
        test_parse!([255], Err(ParseError::InvalidOpcode));
    }
}