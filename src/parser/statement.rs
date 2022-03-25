use crate::{common::error::Result, lexer::token::TokenKind, parentheses, statement};

use super::{
    ast::{node::Node, statement::Statement},
    Parser,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_statement(&mut self) -> Result<'a, Node<'a>> {
        let token = self.cursor.peek()?;
        match token.kind {
            TokenKind::While => statement!(self.parse_while(); self),
            TokenKind::LeftCurlyBrace => self.parse_block(),
            TokenKind::If => statement!(self.parse_if(); self),
            TokenKind::Return => statement!(self.parse_return(); self),
            TokenKind::Let => statement!(self.parse_let(); self),
            _ => {
                let expression = self.parse_expression(0)?;
                self.cursor.consume(TokenKind::Semicolon)?;
                Ok(expression)
            }
        }
    }
    pub(self) fn parse_let(&mut self) -> Result<'a, Node<'a>> {
        let mutable = self.cursor.optional(TokenKind::Mut)?;
        let name = self.cursor.consume(TokenKind::Identifier)?.chunk.data;
        let value_type = if self.cursor.optional(TokenKind::Colon)? {
            Some(self.cursor.consume(TokenKind::Identifier)?.chunk.data)
        } else {
            None
        };

        let value = if self.cursor.optional(TokenKind::Assignment)? {
            let expression = self.parse_expression(0)?;
            self.cursor.consume(TokenKind::Semicolon)?;
            Some(Box::new(expression))
        } else {
            self.cursor.consume(TokenKind::Semicolon)?;

            None
        };
        Ok(Node::Statement(Statement::Let {
            mutable,
            name,
            value_type,
            init: value,
        }))
    }
    pub(self) fn parse_while(&mut self) -> Result<'a, Node<'a>> {
        parentheses!(let test = self.parse_expression(0)?; self);
        let block = self.parse_statement()?;
        Ok(Node::Statement(Statement::While {
            test: Box::new(test),
            body: Box::new(block),
        }))
    }
    pub(self) fn parse_return(&mut self) -> Result<'a, Node<'a>> {
        if self.cursor.optional(TokenKind::Semicolon)? {
            Ok(Node::Statement(Statement::Return { value: None }))
        } else {
            let expression = self.parse_expression(0)?;

            self.cursor.consume(TokenKind::Semicolon)?;
            Ok(Node::Statement(Statement::Return {
                value: Some(Box::new(expression)),
            }))
        }
    }
    pub(self) fn parse_if(&mut self) -> Result<'a, Node<'a>> {
        parentheses!(let test = self.parse_expression(0)?; self);
        let consequent = self.parse_statement()?;
        let alternative = if self.cursor.test_and_next(TokenKind::Else)? {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        Ok(Node::Statement(Statement::If {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternative,
        }))
    }
    pub(super) fn parse_block(&mut self) -> Result<'a, Node<'a>> {
        self.cursor.consume(TokenKind::LeftCurlyBrace)?;
        let mut statements = vec![];
        while !self.cursor.test(TokenKind::RightCurlyBrace) && self.cursor.peek().is_ok() {
            statements.push(self.parse_statement()?);
        }
        self.cursor.next_token()?;
        Ok(Node::Block(statements))
    }
}
