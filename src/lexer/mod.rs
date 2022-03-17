use unicode_xid::UnicodeXID;

use crate::common::{
    error::{Error, ErrorKind, Result},
    source::Source,
};

use self::{
    cursor::{chunk::Chunk, Cursor},
    token::{Token, TokenKind},
};

pub mod cursor;
pub mod iter;
pub mod token;
#[derive(Clone, Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Source<'a>) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }
    #[inline]
    pub fn is_number_start(&mut self) -> bool {
        let char = self.cursor.peek();
        ('0'..='9').contains(&char)
    }
    #[inline]
    pub fn is_number_continue(&mut self) -> bool {
        let char = self.cursor.peek();
        self.is_number_start() || char == '.'
    }
    #[inline]
    pub fn is_identifier_start(&mut self) -> bool {
        let char = self.cursor.peek();
        UnicodeXID::is_xid_start(char)
    }
    #[inline]
    pub fn is_identifier_continue(&mut self) -> bool {
        let char = self.cursor.peek();
        UnicodeXID::is_xid_continue(char)
    }
    pub fn skip(&mut self) {
        while !self.cursor.eof() && (self.cursor.peek() == ' ' || self.cursor.peek() == '\t' || self.cursor.peek() == '\n') {
            self.cursor.next();
        }
        self.cursor.clear();
    }
    pub fn lex_identifier(&mut self) -> Result<'a, Chunk<'a>> {
        while !self.cursor.eof() && self.is_identifier_continue() {
            self.cursor.next()
        }
        Ok(self.cursor.chunk())
    }
    pub fn lex_keyword_or_identifier(&mut self) -> Result<'a, Token<'a>> {
        let identifier = self.lex_identifier()?;
        Ok(match identifier.data {
            "if" => Token::new(identifier, TokenKind::If),
            "while" => Token::new(identifier, TokenKind::While),
            "let" => Token::new(identifier, TokenKind::Let),
            "mut" => Token::new(identifier, TokenKind::Mut),
            "function" => Token::new(identifier, TokenKind::Function),
            "return" => Token::new(identifier, TokenKind::Return),
            _ => Token::new(identifier, TokenKind::Identifier),
        })
    }
    pub fn lex_integer(&mut self) -> Result<'a, Token<'a>> {
        let mut is_float = false;
        let mut has_error = false;
        while !self.cursor.eof() && self.is_number_continue() {
            if self.cursor.peek() == '.' {
                if is_float {
                    has_error = true;
                }
                is_float = true;
            }
            self.cursor.next();
        }
        if has_error {
            return Err(Error::new(
                ErrorKind::UnexpectedToken,
                self.cursor.span(),
                self.cursor.input,
            ));
        }
        let kind = if is_float {
            TokenKind::Float
        } else {
            TokenKind::Integer
        };
        Ok(Token::new(self.cursor.chunk(), kind))
    }
    pub fn lex_char(&mut self) -> Result<'a, Token<'a>> {
        let char = self.cursor.peek();
        self.cursor.next();
        let result = match char {
            '+' => Ok(Token::new(self.cursor.chunk(), TokenKind::Plus)),
            '-' => {
                if self.cursor.lookup(1) == '>' {
                    self.cursor.next();
                    Ok(Token::new(self.cursor.chunk(), TokenKind::Arrow))
                } else {
                    Ok(Token::new(self.cursor.chunk(), TokenKind::Minus))
                }
            }
            '*' => Ok(Token::new(self.cursor.chunk(), TokenKind::Multiply)),
            '/' => Ok(Token::new(self.cursor.chunk(), TokenKind::Divide)),
            ':' => Ok(Token::new(self.cursor.chunk(), TokenKind::Colon)),
            ';' => Ok(Token::new(self.cursor.chunk(), TokenKind::Semicolon)),
            '(' => Ok(Token::new(self.cursor.chunk(), TokenKind::LeftParenthesis)),
            ')' => Ok(Token::new(self.cursor.chunk(), TokenKind::RightParenthesis)),
            '{' => Ok(Token::new(self.cursor.chunk(), TokenKind::LeftCurlyBrace)),
            '}' => Ok(Token::new(self.cursor.chunk(), TokenKind::RightCurlyBrace)),
            ',' => Ok(Token::new(self.cursor.chunk(), TokenKind::Comma)),
            '=' => {
                if self.cursor.lookup(1) == '=' {
                    self.cursor.next();
                    Ok(Token::new(self.cursor.chunk(), TokenKind::Equal))
                } else {
                    Ok(Token::new(self.cursor.chunk(), TokenKind::Assignment))
                }
            }
            '>' => {
                if self.cursor.lookup(1) == '=' {
                    self.cursor.next();
                    Ok(Token::new(self.cursor.chunk(), TokenKind::GreaterThenEqual))
                } else {
                    Ok(Token::new(self.cursor.chunk(), TokenKind::GreaterThen))
                }
            }
            '<' => {
                if self.cursor.lookup(1) == '=' {
                    self.cursor.next();
                    Ok(Token::new(self.cursor.chunk(), TokenKind::LessThenEqual))
                } else {
                    Ok(Token::new(self.cursor.chunk(), TokenKind::LessThen))
                }
            }
            _ => Err(Error::new(
                ErrorKind::UnexpectedEndOfInput,
                self.cursor.span(),
                self.cursor.input,
            )),
        };
        result
    }
    pub fn next_token(&mut self) -> Result<'a, Token<'a>> {
        if self.is_number_start() {
            return self.lex_integer();
        }
        if self.is_identifier_start() {
            return self.lex_keyword_or_identifier();
        }
        self.lex_char()
    }
}
