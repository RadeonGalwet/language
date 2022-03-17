use crate::common::{source::Source, span::Span};

use self::{chunk::Chunk, slice::utf8_slice};

pub mod chunk;
pub mod slice;

#[derive(Clone, Debug)]
pub struct Cursor<'a> {
    start: usize,
    current: usize,
    pub input: Source<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: Source<'a>) -> Self {
        Self {
            start: 0,
            current: 0,
            input,
        }
    }
    #[inline]
    pub fn next(&mut self) {
        self.current += 1;
    }
    #[inline]
    pub fn peek(&self) -> char {
        utf8_slice(self.input.code, self.current, self.current + 1)
            .chars()
            .next()
            .expect("Empty slice in `peek`")
    }
    pub fn span(&self) -> Span {
        Span {
            start: self.start,
            end: self.current,
        }
    }
    pub fn chunk(&mut self) -> Chunk<'a> {
        let span = self.span();
        let data = utf8_slice(self.input.code, span.start, span.end);
        self.clear();
        Chunk::new(data, span)
    }
    pub fn lookup(&mut self, lookup_amount: usize) -> char {
        utf8_slice(
            self.input.code,
            self.current + lookup_amount,
            self.current + (lookup_amount + 1),
        )
        .chars()
        .next()
        .unwrap()
    }
    #[inline]
    pub fn clear(&mut self) {
        self.start = self.current;
    }
    #[inline]
    pub fn eof(&self) -> bool {
        self.current > (self.input.code.chars().count() - 1)
    }
}
