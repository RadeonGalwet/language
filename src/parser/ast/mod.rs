use self::{function::Function, node::Node};

pub mod expression;
pub mod function;
pub mod node;
pub mod spanned;
pub mod statement;
pub type Block<'a> = Vec<Node<'a>>;
#[derive(Clone, Debug)]
pub struct Program<'a> {
    pub path: &'a str,
    pub functions: Vec<Function<'a>>,
}
