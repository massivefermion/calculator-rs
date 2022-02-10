use crate::tokenizer::Tokenizer;
use crate::utils::{Delimiter, Node, NodeType, Op, Side, Token, TokenType};
use std::iter::Peekable;

pub struct AST<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl AST<'_> {
    pub fn new(expr: &str) -> AST {
        let tokenizer = Tokenizer::new(expr);
        AST { tokenizer }
    }

    pub fn generate(&mut self) -> Node {
        self.eval_exp()
    }

    fn eval_exp(&mut self) -> Node {
        let (pos, peeked_token) = match self.tokenizer.peek() {
            None => panic!("unexpected end of expression"),
            Some(Token { pos, token }) => (pos, token),
        };
        let pos = *pos;

        let mut node = match peeked_token {
            TokenType::Operator('-') => {
                self.tokenizer.next();
                Node {
                    pos,
                    node: NodeType::Operation(Op::Negation, vec![self.eval_term()]),
                }
            }
            _ => self.eval_term(),
        };

        loop {
            let (pos, peeked_token) = match self.tokenizer.peek() {
                None => return node,
                Some(Token { pos, token }) => (pos, token),
            };
            let pos = *pos;

            match peeked_token {
                TokenType::Operator('+') => {
                    self.tokenizer.next();
                    node = Node {
                        pos,
                        node: NodeType::Operation(Op::Addition, vec![node, self.eval_term()]),
                    }
                }

                TokenType::Operator('-') => {
                    self.tokenizer.next();
                    node = Node {
                        pos,
                        node: NodeType::Operation(Op::Subtraction, vec![node, self.eval_term()]),
                    }
                }

                _ => return node,
            }
        }
    }

    fn eval_term(&mut self) -> Node {
        let mut node = self.eval_factor();

        loop {
            let (pos, peeked_token) = match self.tokenizer.peek() {
                None => return node,
                Some(Token { pos, token }) => (pos, token),
            };
            let pos = *pos;

            match peeked_token {
                TokenType::Operator('*') => {
                    self.tokenizer.next();
                    node = Node {
                        pos,
                        node: NodeType::Operation(
                            Op::Multiplication,
                            vec![node, self.eval_factor()],
                        ),
                    };
                }

                TokenType::Operator('/') => {
                    self.tokenizer.next();
                    node = Node {
                        pos,
                        node: NodeType::Operation(Op::Division, vec![node, self.eval_factor()]),
                    };
                }

                TokenType::Delimiter(Delimiter::Paranthesis(Side::Open)) | TokenType::Number(_) => {
                    node = Node {
                        pos,
                        node: NodeType::Operation(
                            Op::Multiplication,
                            vec![node, self.eval_factor()],
                        ),
                    };
                }

                _ => return node,
            }
        }
    }

    fn eval_factor(&mut self) -> Node {
        let token = self.tokenizer.next();

        let node = match token {
            Some(Token {
                pos,
                token: TokenType::Number(n),
            }) => Node {
                pos,
                node: NodeType::Number(n),
            },

            Some(Token {
                pos: _,
                token: TokenType::Delimiter(Delimiter::Paranthesis(Side::Open)),
            }) => {
                let node = self.eval_exp();
                let token = self.tokenizer.next();

                match token {
                    None => panic!("unexpected end of expression"),
                    Some(Token {
                        pos: _,
                        token: TokenType::Delimiter(Delimiter::Paranthesis(Side::Close)),
                    }) => node,
                    _ => panic!("unexpected token `{:?}`", token),
                }
            }
            Some(Token { pos, token }) => panic!("unexpected token `{:?}` at {}", token, pos),
            None => panic!("unexpected end of expression"),
        };

        let (pos, peeked_token) = match self.tokenizer.peek() {
            None => return node,
            Some(Token { pos, token }) => (pos, token),
        };
        let pos = *pos;

        match peeked_token {
            TokenType::Operator('^') => {
                self.tokenizer.next();
                Node {
                    pos,
                    node: NodeType::Operation(Op::Exponentiation, vec![node, self.eval_factor()]),
                }
            }
            _ => return node,
        }
    }
}
