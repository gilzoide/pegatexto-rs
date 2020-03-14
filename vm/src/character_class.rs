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

impl CharacterClass {
    pub fn is_member(&self, c: char) -> bool {
        use CharacterClass::*;
        match *self {
            Alphabetic => c.is_alphabetic(),
            Alphanumeric => c.is_alphanumeric(),
            Control => c.is_control(),
            Digit => c.is_digit(10),
            Graphic => c.is_ascii_graphic(),
            Lowercase => c.is_lowercase(),
            Punctuation => c.is_ascii_punctuation(),
            Whitespace => c.is_whitespace(),
            Uppercase => c.is_uppercase(),
            Hexadigit => c.is_digit(16),
        }
    }
}

use std::convert::TryFrom;

pub struct TryFromU8;

impl TryFrom<u8> for CharacterClass {
    type Error = TryFromU8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use CharacterClass::*;
        match value {
            b'a' => Ok(Alphabetic),
            b'w' => Ok(Alphanumeric),
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

