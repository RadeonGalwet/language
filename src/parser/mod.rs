pub mod ast;
pub mod cursor;
pub mod expression;
pub mod statement;
pub mod utils;
#[macro_use]
pub mod macros;
use crate::{
    common::{
        error::{Error, ErrorKind, Result},
        source::Source,
    },
    lexer::token::TokenKind,
    parser::ast::function::{Argument, Function},
};

use self::{ast::Program, cursor::Cursor};
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    source: Source<'a>,
    cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: Source<'a>, cursor: Cursor<'a>) -> Self {
        Self { source, cursor }
    }
    pub fn parse_program(&mut self) -> Result<'a, Program<'a>> {
        let mut functions = vec![];
        while self.cursor.peek().is_ok() {
            let token = self.cursor.peek()?;
            match token.kind {
                TokenKind::Function => {
                    self.cursor.next_token()?;
                    functions.push(self.parse_function()?);
                }
                _ => {
                    return Err(Box::new(Error::new(
                        ErrorKind::UnexpectedToken {
                            expected: vec![TokenKind::Function],
                            received: token.kind,
                        },
                        token.chunk.span,
                        self.source,
                    )))
                }
            }
        }
        Ok(Program {
            path: self.source.path,
            functions,
        })
    }
    pub(self) fn parse_function(&mut self) -> Result<'a, Function<'a>> {
        let name = self.cursor.consume(TokenKind::Identifier)?.chunk.data;
        self.cursor.consume(TokenKind::LeftParenthesis)?;

        let arguments = self.arguments(
            |parser| {
                let argument_name = parser.cursor.consume(TokenKind::Identifier)?;
                parser.cursor.consume(TokenKind::Colon)?;
                let argument_type = parser.cursor.consume(TokenKind::Identifier)?;
                Ok(Argument {
                    name: argument_name.chunk.data,
                    argument_type: argument_type.chunk.data,
                })
            },
            vec![TokenKind::Identifier],
        )?;
        self.cursor.consume(TokenKind::RightParenthesis)?;
        let return_type = if self.cursor.optional(TokenKind::Arrow)? {
            Some(self.cursor.consume(TokenKind::Identifier)?.chunk.data)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(Function {
            name,
            arguments,
            body,
            return_type,
        })
    }
}
