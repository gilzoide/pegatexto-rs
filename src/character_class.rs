#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum CharacterClass {
    Alphabetic = b'a',
    Alphanumeric = b'w',
    Control = b'c',
    Digit = b'd',
    Graphic = b'g',
    Lowercase = b'l',
    Punctuation = b'p',
    Whitespace = b's',
    Uppercase = b'u',
    Hexadigit = b'x',
}

use std::convert::TryFrom;

pub struct TryFromU8;

impl TryFrom<u8> for CharacterClass {
    type Error = TryFromU8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use CharacterClass::*;
        match value {
            b'w' => Ok(Alphanumeric),
            b'a' => Ok(Alphabetic),
            b'c' => Ok(Control),
            b'd' => Ok(Digit),
            b'g' => Ok(Graphic),
            b'l' => Ok(Lowercase),
            b'p' => Ok(Punctuation),
            b's' => Ok(Whitespace),
            b'u' => Ok(Uppercase),
            b'x' => Ok(Hexadigit),
            _ => Err(TryFromU8),
        }
    }
}

