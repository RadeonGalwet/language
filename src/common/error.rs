use std::result;

use super::{source::Source, span::Span};

pub type Result<'a, T> = result::Result<T, Error<'a>>;
#[derive(Clone, Copy, Debug)]
pub struct Error<'a> {
    pub kind: ErrorKind,
    pub span: Span,
    pub source: Source<'a>,
}

impl<'a> Error<'a> {
    pub fn new(kind: ErrorKind, span: Span, source: Source<'a>) -> Self {
        Self { kind, span, source }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
    UnexpectedToken,
    UnexpectedEndOfInput
}
