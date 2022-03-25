#[macro_export]
macro_rules! parentheses {
    ($expr: stmt; $self: ident) => {
        use crate::lexer::token::TokenKind;
        $self.cursor.consume(TokenKind::LeftParenthesis)?;
        $expr
        $self.cursor.consume(TokenKind::RightParenthesis)?;
    };
}

#[macro_export]
macro_rules! statement {
    ($expr: expr; $self: ident) => {{
        $self.cursor.next_token()?;
        $expr
    }};
}
