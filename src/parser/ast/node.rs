use super::{expression::Expression, statement::Statement, Block};

#[derive(Clone, Debug)]
pub enum Node<'a> {
    Integer(&'a str),
    Float(&'a str),
    Identifier(&'a str),
    Block(Block<'a>),
    Expression(Expression<'a>),
    Statement(Statement<'a>),
}
