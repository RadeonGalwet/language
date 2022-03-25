use super::node::Node;

#[derive(Clone, Debug)]
pub struct Argument<'a> {
    pub name: &'a str,
    pub argument_type: &'a str,
}
#[derive(Clone, Debug)]
pub struct Function<'a> {
    pub name: &'a str,
    pub arguments: Vec<Argument<'a>>,
    pub body: Node<'a>,
    pub return_type: Option<&'a str>,
}
