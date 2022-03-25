#[macro_export]
macro_rules! choose {
    ($char: literal => $one: ident || $two: ident; $self: ident) => {{
        if $self.cursor.lookup(1) == $char {
            $self.cursor.next();
            $self.cursor.next();
            Ok(crate::lexer::token::Token::new(
                $self.cursor.chunk(),
                crate::lexer::token::TokenKind::$one,
            ))
        } else {
            $self.cursor.next();
            Ok(crate::lexer::token::Token::new(
                $self.cursor.chunk(),
                crate::lexer::token::TokenKind::$two,
            ))
        }
    }};
}
macro_rules! char {
    ($ident: ident; $self: ident) => {{
        let token = Token::new($self.cursor.chunk(), crate::lexer::token::TokenKind::$ident);
        $self.cursor.next();
        Ok(token)
    }};
}
