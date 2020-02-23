pub type Address = u16;

#[repr(u8)]
pub enum Opcode {
    Nop,
    Succeed,
    Fail,
    FailIfLessThan,
    ToggleSuccess,
    QcZero,
    QcInc,
    Jump,
    JumpIfFail,
    JumpIfSuccess,
    Call,
    Return,
    Push,
    Peek,
    Pop,
    Byte,
    NotByte,
    Literal,
    Class,
    Set,
    Range,
    Action,
}

use std::fmt;

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Opcode::*;
        match *self {
            Nop => write!(f, "nop"),
            Succeed => write!(f, "succ"),
            Fail => write!(f, "fail"),
            FailIfLessThan => write!(f, "flt"),
            ToggleSuccess => write!(f, "togl"),
            QcZero => write!(f, "qcz"),
            QcInc => write!(f, "qci"),
            Jump => write!(f, "jmp"),
            JumpIfFail => write!(f, "jmpf"),
            JumpIfSuccess => write!(f, "jmps"),
            Call => write!(f, "call"),
            Return => write!(f, "ret"),
            Push => write!(f, "push"),
            Peek => write!(f, "peek"),
            Pop => write!(f, "pop"),
            Byte => write!(f, "b"),
            NotByte => write!(f, "nb"),
            Literal => write!(f, "str"),
            Class => write!(f, "cls"),
            Set => write!(f, "set"),
            Range => write!(f, "rng"),
            Action => write!(f, "act"),
        }
    }
}
