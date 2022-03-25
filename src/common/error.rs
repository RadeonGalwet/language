use std::result;

use crate::lexer::token::TokenKind;

use super::{source::Source, span::Span};

pub type Result<'a, T> = result::Result<T, Box<Error<'a>>>;
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum ErrorKind {
    UnexpectedCharacter,
    UnexpectedToken {
        expected: Vec<TokenKind>,
        received: TokenKind,
    },
    UnexpectedEndOfInput,
}
