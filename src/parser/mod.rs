pub mod ast;
pub mod cursor;

use crate::{
    common::{
        error::{Error, ErrorKind, Result},
        source::Source,
    },
    lexer::token::TokenKind,
    parser::ast::{
        expression::Expression,
        function::{Argument, Function},
        statement::Statement,
    },
};

use self::{
    ast::{expression::Operator, node::Node, Block, Program},
    cursor::Cursor,
};
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
                    self.cursor.next_token()?;
                    let body = self.parse_block()?;
                    functions.push(Function {
                        name,
                        arguments,
                        body: Node::Block(body),
                        return_type,
                    });
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
    pub fn parse_statement(&mut self) -> Result<'a, Node<'a>> {
        let token = self.cursor.peek()?;
        match token.kind {
            TokenKind::While => {
                self.cursor.consume(TokenKind::While)?;
                self.cursor.consume(TokenKind::LeftParenthesis)?;
                let test = self.parse_expression(0)?;
                self.cursor.consume(TokenKind::RightParenthesis)?;
                let block = self.parse_statement()?;
                Ok(Node::Statement(Statement::While {
                    test: Box::new(test),
                    body: Box::new(block),
                }))
            }
            TokenKind::LeftCurlyBrace => {
                self.cursor.next_token()?;
                Ok(Node::Block(self.parse_block()?))
            }
            TokenKind::If => self.parse_if(),
            TokenKind::Return => {
                self.cursor.next_token()?;
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
            TokenKind::Let => {
                self.cursor.next_token()?;
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
            _ => {
                let expression = self.parse_expression(0)?;
                self.cursor.consume(TokenKind::Semicolon)?;
                Ok(expression)
            }
        }
    }
    pub fn parse_if(&mut self) -> Result<'a, Node<'a>> {
        self.cursor.consume(TokenKind::If)?;
        self.cursor.consume(TokenKind::LeftParenthesis)?;
        let test = self.parse_expression(0)?;
        self.cursor.consume(TokenKind::RightParenthesis)?;
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
    pub fn parse_block(&mut self) -> Result<'a, Block<'a>> {
        let mut statements = vec![];
        while !self.cursor.test(TokenKind::RightCurlyBrace) && self.cursor.peek().is_ok() {
            statements.push(self.parse_statement()?);
        }
        self.cursor.consume(TokenKind::RightCurlyBrace)?;
        Ok(statements)
    }
    pub fn parse_expression(&mut self, minimum_binding_power: u8) -> Result<'a, Node<'a>> {
        let lhs_token = self.cursor.next_token()?;
        let mut lhs = match lhs_token.kind {
            TokenKind::Integer => Node::Integer(lhs_token.chunk.data),
            TokenKind::Float => Node::Float(lhs_token.chunk.data),
            TokenKind::Identifier => Node::Identifier(lhs_token.chunk.data),
            TokenKind::LeftParenthesis => {
                let expression = self.parse_expression(0)?;
                self.cursor.consume(TokenKind::RightParenthesis)?;
                expression
            }
            TokenKind::Plus | TokenKind::Minus => {
                let operator = Operator::from(lhs_token.kind);
                let ((), right_binding_power) = Self::prefix_binding_power(operator).unwrap();
                let rhs = self.parse_expression(right_binding_power)?;
                Node::Expression(Expression::Prefix {
                    operator,
                    value: Box::new(rhs),
                })
            }
            _ => {
                return Err(Box::new(Error::new(
                    ErrorKind::UnexpectedToken {
                        expected: vec![
                            TokenKind::Identifier,
                            TokenKind::Integer,
                            TokenKind::Float,
                            TokenKind::LeftParenthesis,
                            TokenKind::Plus,
                            TokenKind::Minus,
                        ],
                        received: lhs_token.kind,
                    },
                    lhs_token.chunk.span,
                    self.source,
                )))
            }
        };
        while let Ok(operator_token) = self.cursor.peek() {
            if let Some((left_binding_power, ())) = Self::postfix_binding_power(operator_token.kind)
            {
                if operator_token.kind == TokenKind::LeftParenthesis {
                    if left_binding_power < minimum_binding_power {
                        break;
                    }
                    self.cursor.next_token()?;
                    let arguments = self.arguments(
                        |parser| parser.parse_expression(0),
                        vec![
                            TokenKind::LeftParenthesis,
                            TokenKind::Identifier,
                            TokenKind::Float,
                            TokenKind::Integer,
                        ],
                    )?;
                    self.cursor.consume(TokenKind::RightParenthesis)?;
                    if lhs_token.kind == TokenKind::Identifier {
                        lhs = Node::Expression(Expression::Call {
                            name: lhs_token.chunk.data,
                            arguments,
                        })
                    } else {
                        return Err(Box::new(Error::new(
                            ErrorKind::UnexpectedToken {
                                expected: vec![TokenKind::Identifier],
                                received: lhs_token.kind,
                            },
                            lhs_token.chunk.span,
                            self.source,
                        )));
                    }
                    continue;
                }
            }
            if let Some((left_binding_power, right_binding_power)) =
                Self::infix_binding_power(operator_token.kind)
            {
                if left_binding_power < minimum_binding_power {
                    break;
                }
                let operator = Operator::from(operator_token.kind);
                self.cursor.next_token()?;
                let rhs = self.parse_expression(right_binding_power)?;
                lhs = Node::Expression(Expression::Infix {
                    operator,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                });
                continue;
            }
            break;
        }
        Ok(lhs)
    }
    fn postfix_binding_power(kind: TokenKind) -> Option<(u8, ())> {
        match kind {
            TokenKind::LeftParenthesis => Some((13, ())),
            _ => None,
        }
    }
    #[inline]
    fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
        match kind {
            TokenKind::Assignment => Some((1, 2)),
            TokenKind::Plus | TokenKind::Minus => Some((2, 3)),
            TokenKind::Multiply | TokenKind::Divide => Some((4, 5)),
            TokenKind::LessThen
            | TokenKind::GreaterThen
            | TokenKind::LessThenEqual
            | TokenKind::GreaterThenEqual => Some((6, 7)),
            TokenKind::Equal => Some((8, 9)),

            _ => None,
        }
    }
    #[inline]
    fn prefix_binding_power(operator: Operator) -> Option<((), u8)> {
        match operator {
            Operator::Plus | Operator::Minus => Some(((), 12)),
            _ => None,
        }
    }
    pub fn parse_function(&mut self) {}
    pub fn arguments<F, T>(
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
