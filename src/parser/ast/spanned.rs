use std::fmt::Debug;

use crate::common::span::Span;

#[derive(Clone, Debug)]
pub struct Spanned<T: Clone + Debug> {
    pub value: T,
    pub span: Span,
}

impl<T: Clone + Debug> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
}
