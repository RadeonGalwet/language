use crate::common::error::Result;
use super::{Lexer, token::Token};

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<'a, Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip();
        if self.cursor.eof() {
            None
        } else {
            Some(self.next_token())
        }
    }
}