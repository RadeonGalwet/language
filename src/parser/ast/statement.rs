use super::node::Node;

#[derive(Clone, Debug)]
pub enum Statement<'a> {
    While {
        test: Box<Node<'a>>,
        body: Box<Node<'a>>,
    },
    If {
        test: Box<Node<'a>>,
        consequent: Box<Node<'a>>,
        alternative: Option<Box<Node<'a>>>,
    },
    Let {
        mutable: bool,
        name: &'a str,
        value_type: Option<&'a str>,
        init: Option<Box<Node<'a>>>,
    },
    Return {
        value: Option<Box<Node<'a>>>,
    },
}
