use crate::lexer::token::TokenKind;

use super::node::Node;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Assignment,
    Equal,
    LessThen,
    GreaterThen,
    LessThenEqual,
    GreaterThenEqual,
}

impl From<TokenKind> for Operator {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::Plus => Operator::Plus,
            TokenKind::Minus => Operator::Minus,
            TokenKind::Multiply => Operator::Multiply,
            TokenKind::Divide => Operator::Divide,
            TokenKind::Assignment => Operator::Assignment,
            TokenKind::Equal => Operator::Equal,
            TokenKind::LessThen => Operator::LessThen,
            TokenKind::GreaterThen => Operator::GreaterThen,
            TokenKind::LessThenEqual => Operator::LessThenEqual,
            TokenKind::GreaterThenEqual => Operator::GreaterThenEqual,
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Infix {
        operator: Operator,
        lhs: Box<Node<'a>>,
        rhs: Box<Node<'a>>,
    },
    Prefix {
        operator: Operator,
        value: Box<Node<'a>>,
    },
    Call {
        name: &'a str,
        arguments: Vec<Node<'a>>,
    },
}
