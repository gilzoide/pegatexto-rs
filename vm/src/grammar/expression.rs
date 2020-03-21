use super::character_class::CharacterClass;

use std::ops::{Add, BitXor, Div, Neg, Not};

pub enum Expression<'a> {
    Char(char),
    Literal(&'a str),
    Class(CharacterClass),
    Set(&'a str),
    Range(char, char),
    Any,
    NonTerminal(&'a str),
    Quantifier(Box<Expression<'a>>, i32),
    And(Box<Expression<'a>>),
    Not(Box<Expression<'a>>),
    Sequence(Vec<Expression<'a>>),
    Choice(Vec<Expression<'a>>),
    //Error(i32, Expression),
}

impl<'a> Add for Expression<'a> {
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

impl<'a> BitXor<i32> for Expression<'a> {
    type Output = Self;

    fn bitxor(self, other: i32) -> Self::Output {
        Expression::Quantifier(Box::new(self), other)
    }
}

impl<'a> Div for Expression<'a> {
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

impl<'a> Neg for Expression<'a> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Expression::And(_) => self,
            _ => Expression::And(Box::new(self)),
        }
    }
}

impl<'a> Not for Expression<'a> {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Expression::Not(e) => *e,
            _ => Expression::Not(Box::new(self)),
        }
    }
}
