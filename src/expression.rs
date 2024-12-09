use std::fmt::{Display, Formatter, Result};

use crate::token::Token;

pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Literal(Token),
    Grouping(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expression::Binary(left, operator, right) => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expression::Unary(operator, right) => {
                write!(f, "({} {})", operator, right)
            }
            Expression::Literal(token) => {
                write!(f, "{}", token)
            }
            Expression::Grouping(expression) => {
                write!(f, "(group {})", expression)
            }
        }
    }
}
