use crate::{
    common::error::{Error, ErrorKind, Result},
    lexer::token::TokenKind,
};

use super::{
    ast::{
        expression::{Expression, Operator},
        node::Node,
    },
    Parser,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_expression(&mut self, minimum_binding_power: u8) -> Result<'a, Node<'a>> {
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
    #[inline]
    pub(super) fn postfix_binding_power(kind: TokenKind) -> Option<(u8, ())> {
        match kind {
            TokenKind::LeftParenthesis => Some((13, ())),
            _ => None,
        }
    }
    #[inline]
    pub(super) fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
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
    pub(super) fn prefix_binding_power(operator: Operator) -> Option<((), u8)> {
        match operator {
            Operator::Plus | Operator::Minus => Some(((), 12)),
            _ => None,
        }
    }
}
