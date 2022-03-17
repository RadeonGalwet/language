use crate::common::span::Span;

#[derive(Clone, Copy, Debug)]
pub struct Chunk<'a> {
    pub data: &'a str,
    pub span: Span,
}

impl<'a> Chunk<'a> {
    pub fn new(data: &'a str, span: Span) -> Self {
        Self { data, span }
    }
}
