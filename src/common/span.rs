use crate::lexer::cursor::slice::utf8_slice;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn slice(self, string: &str) -> &str {
        utf8_slice(string, self.start, self.end)
    }
}
