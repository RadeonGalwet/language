use super::cursor::chunk::Chunk;

#[derive(Clone, Copy, Debug)]
pub enum TokenKind {
    Identifier,
    Integer,
    Float,

    Plus,
    Minus,
    Multiply,
    Divide,
    Assignment,
    Equal,
    LessThen,
    GreaterThen,
    LessThenEqual,
    GreaterThenEqual,

    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    Colon,
    Semicolon,
    Arrow,
    Comma,

    If,
    While,
    Let,
    Mut,
    Function,
    Return,
}

#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub chunk: Chunk<'a>,
    pub kind: TokenKind,
}

impl<'a> Token<'a> {
    pub fn new(chunk: Chunk<'a>, kind: TokenKind) -> Self {
        Self { chunk, kind }
    }
}
