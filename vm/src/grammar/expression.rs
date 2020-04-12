use super::character_class::CharacterClass;

use std::ops::{Add, BitXor, Div, Neg, Not, Shr};

pub enum Expression {
    Char(char),
    Literal(String),
    Class(CharacterClass),
    Set(String),
    InverseSet(String),
    Range(char, char),
    Any,
    NonTerminal(String),
    Quantifier(Box<Expression>, i32),
    And(Box<Expression>),
    Not(Box<Expression>),
    Sequence(Vec<Expression>),
    Choice(Vec<Expression>),
    Capture(Box<Expression>, String),
    //Error(i32, Expression),
}

impl Add for Expression {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Expression::Sequence(mut exprs) => {
                exprs.push(other);
                Expression::Sequence(exprs)
            },
            _ => Expression::Sequence(vec![self, other])
        }
    }
}

impl BitXor<i32> for Expression {
    type Output = Self;

    fn bitxor(self, other: i32) -> Self::Output {
        Expression::Quantifier(Box::new(self), other)
    }
}

impl Div for Expression {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match self {
            Expression::Choice(mut exprs) => {
                exprs.push(other);
                Expression::Choice(exprs)
            },
            _ => Expression::Choice(vec![self, other])
        }
    }
}

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Expression::And(_) => self,
            _ => Expression::And(Box::new(self)),
        }
    }
}

impl Not for Expression {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Expression::Not(e) => *e,
            _ => Expression::Not(Box::new(self)),
        }
    }
}

impl Shr<&str> for Expression {
    type Output = Self;

    fn shr(self, name: &str) -> Self::Output {
        Expression::Capture(Box::new(self), name.to_string())
    }
}

