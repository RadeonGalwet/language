use std::iter::Peekable;

use crate::{
    common::{
        error::{Error, ErrorKind, Result},
        source::Source,
        span::Span,
    },
    lexer::{
        token::{Token, TokenKind},
        Lexer,
    },
};

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    pub lexer: Peekable<Lexer<'a>>,
    pub source: Source<'a>,
    pub current_span: Option<Span>,
}

impl<'a> Cursor<'a> {
    pub fn new(lexer: Peekable<Lexer<'a>>, source: Source<'a>) -> Self {
        Self {
            lexer,
            source,
            current_span: None,
        }
    }
    pub fn next_token(&mut self) -> Result<'a, Token<'a>> {
        let token = self.lexer.next();
        match token {
            Some(token) => match token {
                Ok(token) => {
                    self.update_span(token.chunk.span);
                    Ok(token)
                }
                Err(err) => Err(err),
            },
            None => Err(Box::new(Error::new(
                ErrorKind::UnexpectedEndOfInput,
                self.current_span.expect("Empty input"),
                self.source,
            ))),
        }
    }
    #[inline]
    pub fn test(&mut self, kind: TokenKind) -> bool {
        match self.peek() {
            Ok(token) => token.kind == kind,
            Err(_) => false,
        }
    }
    pub fn test_and_next(&mut self, kind: TokenKind) -> Result<'a, bool> {
        if self.test(kind) {
            self.next_token()?;
            Ok(true)
        } else {
            self.next_token()?;
            Ok(false)
        }
    }
    pub fn optional(&mut self, kind: TokenKind) -> Result<'a, bool> {
        if self.test(kind) {
            self.next_token()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    pub fn peek(&mut self) -> Result<'a, &Token<'a>> {
        let token = self.lexer.peek();
        match token {
            Some(token) => match token {
                Ok(token) => Ok(token),
                Err(err) => Err(err.to_owned()),
            },
            None => Err(Box::new(Error::new(
                ErrorKind::UnexpectedEndOfInput,
                self.current_span.expect("Empty input"),
                self.source,
            ))),
        }
    }
    #[inline]
    fn update_span(&mut self, new_span: Span) {
        match self.current_span {
            Some(span) => self.current_span = Some(Span::new(span.start, new_span.end)),
            None => self.current_span = Some(new_span),
        }
    }
    pub fn lookup(&mut self, lookup_amount: usize) -> Option<Result<'a, Token<'a>>> {
        let mut iterator = self.lexer.to_owned();
        for _ in 0..lookup_amount {
            iterator.next();
        }
        iterator.next()
    }
    #[inline]
    pub fn clear(&mut self) {
        match self.current_span {
            Some(span) => self.current_span = Some(Span::new(span.end, span.end)),
            None => self.current_span = Some(Span::new(0, 0)),
        }
    }
    pub fn consume(&mut self, kind: TokenKind) -> Result<'a, Token<'a>> {
        let current_token = self.peek()?;
        if current_token.kind == kind {
            self.next_token()
        } else {
            Err(Box::new(Error::new(
                ErrorKind::UnexpectedToken {
                    expected: vec![kind],
                    received: current_token.kind,
                },
                current_token.chunk.span,
                self.source,
            )))
        }
    }
}
