use crate::common::span::Span;

use super::{calculate_span::CalculateSpan, node::Node, spanned::Spanned};

#[derive(Clone, Debug)]
pub enum Statement<'a> {
    While(Spanned<WhileStatement<'a>>),
    If(Spanned<IfStatement<'a>>),
    Let(Spanned<LetStatement<'a>>),
    Return(Spanned<ReturnStatement<'a>>),
}
#[derive(Clone, Debug)]
pub struct WhileStatement<'a> {
    pub test: Box<Node<'a>>,
    pub body: Box<Node<'a>>,
}
#[derive(Clone, Debug)]
pub struct IfStatement<'a> {
    pub test: Box<Node<'a>>,
    pub consequent: Box<Node<'a>>,
    pub alternative: Option<Box<Node<'a>>>,
}
#[derive(Clone, Debug)]
pub struct LetStatement<'a> {
    pub mutable: bool,
    pub name: &'a str,
    pub value_type: Option<&'a str>,
    pub init: Option<Box<Node<'a>>>,
}
#[derive(Clone, Debug)]
pub struct ReturnStatement<'a> {
    pub value: Option<Box<Node<'a>>>,
}
impl<'a> CalculateSpan for Statement<'a> {
    fn calculate_span(&self) -> Span {
        match self {
            Statement::While(while_statement) => while_statement.span,
            Statement::If(if_statement) => if_statement.span,
            Statement::Let(let_statement) => let_statement.span,
            Statement::Return(return_statement) => return_statement.span,
        }
    }
}
