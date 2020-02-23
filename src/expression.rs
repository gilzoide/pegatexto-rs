#[derive(Debug, Copy, Clone)]
pub enum CharClass {
    Alphanumeric,
    Alphabetic,
    Control,
    Digit,
    Graphic,
    Lowercase,
    Punctuation,
    Whitespace,
    Uppercase,
    Hexadigit,
}

use std::convert::{From,TryFrom};

impl TryFrom<char> for CharClass {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use CharClass::*;
        match value {
            'w' => Ok(Alphanumeric),
            'a' => Ok(Alphabetic),
            'c' => Ok(Control),
            'd' => Ok(Digit),
            'g' => Ok(Graphic),
            'l' => Ok(Lowercase),
            'p' => Ok(Punctuation),
            's' => Ok(Whitespace),
            'u' => Ok(Uppercase),
            'h' => Ok(Hexadigit),
            _ => Err("Invalid character class"),
        }
    }
}

impl From<CharClass> for char {
    fn from(value: CharClass) -> Self {
        use CharClass::*;
        match value {
            Alphanumeric => 'w',
            Alphabetic => 'a',
            Control => 'c',
            Digit => 'd',
            Graphic => 'g',
            Lowercase => 'l',
            Punctuation => 'p',
            Whitespace => 's',
            Uppercase => 'u',
            Hexadigit => 'h',
        }
    }
}

pub enum Expression<'a> {
    Byte(char),
    Literal(&'a str),
    Class(CharClass),
    Set(&'a str),
    Range(char, char),
    Any,
    NonTerminal(&'a str),
    Quantifier(&'a Expression<'a>, i32),
    And(&'a Expression<'a>),
    Not(&'a Expression<'a>),
    Sequence(Vec<Expression<'a>>),
    Choice(Vec<Expression<'a>>),
    //Error(i32, Expression),
}

use std::fmt;

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;
        match *self {
            Byte(c) => write!(f, "'{}'", c),
            Literal(s) => write!(f, "\"{}\"", s),
            Class(c) => write!(f, "\\{}", char::from(c)),
            Set(s) => write!(f, "[{}]", s),
            Range(c_min, c_max) => write!(f, "[{}-{}]", c_min, c_max),
            Any => write!(f, "."),
            NonTerminal(s) => write!(f, "{}", s),
            Quantifier(e, n) => {
                let suffix = match n {
                    -1 => "?".to_owned(),
                    0 => "*".to_owned(),
                    1 => "+".to_owned(),
                    _ => format!("^{}", n),
                };
                write!(f, "{}{}", e, suffix)
            },
            And(e) => write!(f, "&{}", e),
            Not(e) => write!(f, "!{}", e),
            Sequence(ref _es) => {
                write!(f, "")
            },
            Choice(ref _es) => {
                write!(f, "")
            },
        }
    }
}

