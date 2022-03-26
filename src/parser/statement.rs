use crate::{
    common::{error::Result, span::Span},
    lexer::token::TokenKind,
    parentheses,
    parser::ast::{
        calculate_span::CalculateSpan,
        statement::{IfStatement, WhileStatement},
    },
    statement,
};

use super::{
    ast::{
        node::Node,
        spanned::Spanned,
        statement::{LetStatement, ReturnStatement, Statement},
    },
    Parser,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_statement(&mut self) -> Result<'a, Node<'a>> {
        let token = self.cursor.peek()?;
        match token.kind {
            TokenKind::While => statement!(parse_while; self),
            TokenKind::LeftCurlyBrace => self.parse_block(),
            TokenKind::If => statement!(parse_if; self),
            TokenKind::Return => statement!(parse_return; self),
            TokenKind::Let => statement!(parse_let; self),
            _ => {
                let expression = self.parse_expression(0)?;
                self.cursor.consume(TokenKind::Semicolon)?;
                Ok(expression)
            }
        }
    }
    pub(self) fn parse_let(&mut self, let_kw_span: Span) -> Result<'a, Node<'a>> {
        let mutable = self.cursor.optional(TokenKind::Mut)?;
        let name = self.cursor.consume(TokenKind::Identifier)?.chunk.data;
        let value_type = if self.cursor.optional(TokenKind::Colon)? {
            Some(self.cursor.consume(TokenKind::Identifier)?.chunk.data)
        } else {
            None
        };
        let init = if self.cursor.optional(TokenKind::Assignment)? {
            let expression = self.parse_expression(0)?;
            Some(Box::new(expression))
        } else {
            None
        };
        let semicolon_token = self.cursor.consume(TokenKind::Semicolon)?;
        Ok(Node::Statement(Statement::Let(Spanned::new(
            LetStatement {
                mutable,
                name,
                value_type,
                init,
            },
            Span::new(let_kw_span.start, semicolon_token.chunk.span.end),
        ))))
    }
    pub(self) fn parse_while(&mut self, while_kw_span: Span) -> Result<'a, Node<'a>> {
        parentheses!(let test = self.parse_expression(0)?; self);
        let block = self.parse_statement()?;
        let block_span = block.calculate_span();
        Ok(Node::Statement(Statement::While(Spanned::new(
            WhileStatement {
                test: Box::new(test),
                body: Box::new(block),
            },
            Span::new(while_kw_span.start, block_span.end),
        ))))
    }
    pub(self) fn parse_return(&mut self, return_kw_span: Span) -> Result<'a, Node<'a>> {
        if self.cursor.test(TokenKind::Semicolon) {
            let semicolon_token = self.cursor.next_token()?;
            Ok(Node::Statement(Statement::Return(Spanned::new(
                ReturnStatement { value: None },
                Span::new(return_kw_span.start, semicolon_token.chunk.span.end),
            ))))
        } else {
            let expression = self.parse_expression(0)?;
            let semicolon_token = self.cursor.consume(TokenKind::Semicolon)?;
            Ok(Node::Statement(Statement::Return(Spanned::new(
                ReturnStatement {
                    value: Some(Box::new(expression)),
                },
                Span::new(return_kw_span.start, semicolon_token.chunk.span.end),
            ))))
        }
    }
    pub(self) fn parse_if(&mut self, if_kw_span: Span) -> Result<'a, Node<'a>> {
        parentheses!(let test = self.parse_expression(0)?; self);
        let consequent = self.parse_statement()?;
        let consequent_span = consequent.calculate_span();
        let alternative = if self.cursor.test_and_next(TokenKind::Else)? {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        Ok(Node::Statement(Statement::If(Spanned::new(
            IfStatement {
                test: Box::new(test),
                consequent: Box::new(consequent),
                alternative,
            },
            Span::new(if_kw_span.start, consequent_span.end),
        ))))
    }
    pub(super) fn parse_block(&mut self) -> Result<'a, Node<'a>> {
        let lcb = self.cursor.consume(TokenKind::LeftCurlyBrace)?;
        let mut statements = vec![];
        while !self.cursor.test(TokenKind::RightCurlyBrace) && self.cursor.peek().is_ok() {
            statements.push(self.parse_statement()?);
        }
        let rcb = self.cursor.next_token()?;
        Ok(Node::Block(Spanned::new(
            statements,
            Span::new(lcb.chunk.span.start, rcb.chunk.span.end),
        )))
    }
}
