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
    ($function: ident; $self: ident) => {{
        let token = $self.cursor.next_token()?;
        $self.$function(token.chunk.span)
    }};
}
