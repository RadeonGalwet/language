use crate::{
    common::error::{Error, ErrorKind, Result},
    lexer::token::TokenKind,
};

use super::Parser;

impl<'a> Parser<'a> {
    pub(super) fn arguments<F, T>(
        &mut self,
        function: F,
        possible_tokens_after_comma: Vec<TokenKind>,
    ) -> Result<'a, Vec<T>>
    where
        F: Fn(&mut Self) -> Result<'a, T>,
    {
        let mut args = vec![];
        loop {
            args.push(function(self)?);
            if !self.cursor.optional(TokenKind::Comma)? {
                let token = self.cursor.peek()?;
                if possible_tokens_after_comma.contains(&token.kind) {
                    return Err(Box::new(Error::new(
                        ErrorKind::UnexpectedToken {
                            expected: vec![TokenKind::Comma],
                            received: token.kind,
                        },
                        token.chunk.span,
                        self.source,
                    )));
                }
                break;
            }
        }
        Ok(args)
    }
}
