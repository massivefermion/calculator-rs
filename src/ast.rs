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
        let sign = self.eval_sign(1.0);
        let mut node = if sign == -1.0 {
            self.tokenizer.next();
            Node {
                pos: 0.0 as usize,
                node: NodeType::Operation(Op::Negation, vec![self.eval_term()]),
            }
        } else {
            self.eval_term()
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
                    let sign = self.eval_sign(1.0);
                    node = if sign == -1.0 {
                        Node {
                            pos,
                            node: NodeType::Operation(
                                Op::Addition,
                                vec![Node {
                                    pos: pos - 1,
                                    node: NodeType::Operation(Op::Negation, vec![self.eval_term()]),
                                }],
                            ),
                        }
                    } else {
                        Node {
                            pos,
                            node: NodeType::Operation(Op::Addition, vec![node, self.eval_term()]),
                        }
                    };
                }

                TokenType::Operator('-') => {
                    self.tokenizer.next();
                    let sign = self.eval_sign(1.0);
                    node = if sign == -1.0 {
                        Node {
                            pos,
                            node: NodeType::Operation(
                                Op::Subtraction,
                                vec![Node {
                                    pos: pos - 1,
                                    node: NodeType::Operation(Op::Negation, vec![self.eval_term()]),
                                }],
                            ),
                        }
                    } else {
                        Node {
                            pos,
                            node: NodeType::Operation(
                                Op::Subtraction,
                                vec![node, self.eval_term()],
                            ),
                        }
                    };
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
                    let sign = self.eval_sign(1.0);
                    node = if sign == -1.0 {
                        Node {
                            pos,
                            node: NodeType::Operation(
                                Op::Multiplication,
                                vec![Node {
                                    pos: pos - 1,
                                    node: NodeType::Operation(Op::Negation, vec![self.eval_term()]),
                                }],
                            ),
                        }
                    } else {
                        Node {
                            pos,
                            node: NodeType::Operation(
                                Op::Multiplication,
                                vec![node, self.eval_term()],
                            ),
                        }
                    };
                }

                TokenType::Operator('/') => {
                    self.tokenizer.next();
                    let sign = self.eval_sign(1.0);
                    node = if sign == -1.0 {
                        Node {
                            pos,
                            node: NodeType::Operation(
                                Op::Addition,
                                vec![Node {
                                    pos: pos - 1,
                                    node: NodeType::Operation(Op::Division, vec![self.eval_term()]),
                                }],
                            ),
                        }
                    } else {
                        Node {
                            pos,
                            node: NodeType::Operation(Op::Division, vec![node, self.eval_term()]),
                        }
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
        let node = match self.tokenizer.next() {
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
                match self.tokenizer.next() {
                    None => panic!("unexpected end of expression"),
                    Some(Token {
                        pos: _,
                        token: TokenType::Delimiter(Delimiter::Paranthesis(Side::Close)),
                    }) => node,
                    token => panic!("unexpected token `{:?}`", token),
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

    fn eval_sign(&mut self, sign: f64) -> f64 {
        match self.tokenizer.peek() {
            None => panic!("unexpected end of expression"),
            Some(Token {
                pos: _,
                token: TokenType::Operator('-'),
            }) => {
                self.tokenizer.next();
                self.eval_sign(-1.0 * sign)
            }
            Some(Token {
                pos: _,
                token: TokenType::Operator('+'),
            }) => {
                self.tokenizer.next();
                self.eval_sign(sign)
            }
            _ => sign,
        }
    }
}
