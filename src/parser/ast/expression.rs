use crate::{common::span::Span, lexer::token::TokenKind};

use super::{calculate_span::CalculateSpan, node::Node, spanned::Spanned};

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
        operator: Spanned<Operator>,
        value: Box<Node<'a>>,
    },
    Call {
        name: Spanned<&'a str>,
        arguments: Spanned<Vec<Node<'a>>>,
    },
}
impl<'a> CalculateSpan for Expression<'a> {
    fn calculate_span(&self) -> Span {
        match self {
            Expression::Infix { lhs, rhs, .. } => {
                Span::new(lhs.calculate_span().start, rhs.calculate_span().end)
            }
            Expression::Prefix { operator, value } => {
                Span::new(operator.span.start, value.calculate_span().end)
            }
            Expression::Call { name, arguments } => Span::new(name.span.start, arguments.span.end),
        }
    }
}
