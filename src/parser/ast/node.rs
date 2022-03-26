use crate::common::span::Span;

use super::{
    calculate_span::CalculateSpan, expression::Expression, spanned::Spanned, statement::Statement,
    Block,
};

#[derive(Clone, Debug)]
pub enum Node<'a> {
    Integer(Spanned<&'a str>),
    Float(Spanned<&'a str>),
    Identifier(Spanned<&'a str>),
    Block(Spanned<Block<'a>>),
    Expression(Expression<'a>),
    Statement(Statement<'a>),
}
impl<'a> CalculateSpan for Node<'a> {
    fn calculate_span(&self) -> Span {
        match self {
            Node::Integer(integer) => integer.span,
            Node::Float(float) => float.span,
            Node::Identifier(id) => id.span,
            Node::Block(block) => block.span,
            Node::Expression(expression) => expression.calculate_span(),
            Node::Statement(statement) => statement.calculate_span(),
        }
    }
}
