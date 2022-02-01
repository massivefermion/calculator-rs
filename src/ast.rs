use crate::tokenizer::Tokenizer;
use crate::utils::{Delimiter, Node, Op, Side, Token};
use std::iter::Peekable;

pub struct AST<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl AST<'_> {
    pub fn new(expr: &str) -> AST {
        let tokenizer = Tokenizer::new(expr).peekable();
        AST { tokenizer }
    }

    pub fn generate(&mut self) -> Node {
        self.eval_exp()
    }

    fn eval_exp(&mut self) -> Node {
        let mut node: Node;

        let peeked = self.tokenizer.peek();
        let op = if let Some(token) = peeked {
            if let Token::Operator(ch) = token {
                if *ch == '-' {
                    self.tokenizer.next();
                    Some(Op::Negation)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            panic!("unexpected end of expression")
        };

        node = self.eval_term();
        if let Some(operator) = op {
            node = Node::Operation((operator, vec![node]));
        }

        loop {
            if self.tokenizer.peek().is_none() {
                return node;
            }
            let peeked = self.tokenizer.peek().unwrap().clone();
            match peeked {
                Token::Operator(op) => {
                    if ['+', '-'].contains(&op) {
                        self.tokenizer.next();
                        let other_operand = self.eval_term();
                        match op {
                            '+' => {
                                node = Node::Operation((Op::Addition, vec![node, other_operand]));
                            }
                            '-' => {
                                node =
                                    Node::Operation((Op::Subtraction, vec![node, other_operand]));
                            }
                            _ => {
                                return node;
                            }
                        }
                    } else {
                        return node;
                    }
                }
                _ => {
                    return node;
                }
            }
        }
    }

    fn eval_term(&mut self) -> Node {
        let mut node = self.eval_factor();

        loop {
            if self.tokenizer.peek().is_none() {
                return node;
            }
            let peeked = self.tokenizer.peek().unwrap().clone();
            match peeked {
                Token::Operator(op) => {
                    if ['*', '/'].contains(&op) {
                        self.tokenizer.next();
                        let other_operand = self.eval_factor();
                        match op {
                            '*' => {
                                node = Node::Operation((
                                    Op::Multiplication,
                                    vec![node, other_operand],
                                ));
                            }
                            '/' => {
                                node = Node::Operation((Op::Division, vec![node, other_operand]));
                            }
                            _ => {
                                return node;
                            }
                        }
                    } else {
                        return node;
                    }
                }
                _ => {
                    return node;
                }
            }
        }
    }

    fn eval_factor(&mut self) -> Node {
        let token = self.tokenizer.next().unwrap();
        let mut node = match token {
            Token::Number(n) => Node::Number(n),
            Token::Delimiter(delim) => match delim {
                Delimiter::Paranthesis(side) => match side {
                    Side::Open => {
                        let node = self.eval_exp();
                        let token = self.tokenizer.next();
                        match token {
                            None => panic!("unexpected end of expression"),
                            Some(peeked) => match peeked {
                                Token::Delimiter(delim) => match delim {
                                    Delimiter::Paranthesis(side) => match side {
                                        Side::Close => node,
                                        _ => panic!("unexpected token `{:?}`", side),
                                    },
                                },
                                _ => panic!("unexpected token `{:?}`", peeked),
                            },
                        }
                    }
                    _ => panic!("unexpected token `{:?}`", side),
                },
            },
            _ => panic!("unexpected token `{:?}`", token),
        };

        match self.tokenizer.peek() {
            Some(token) => match token {
                Token::Operator(ch) => {
                    if *ch == '^' {
                        self.tokenizer.next();
                        let exponent = self.tokenizer.next();
                        match exponent {
                            None => panic!("unexpected end of expression"),
                            Some(mut exponent_candidate) => {
                                let mut sign = 1.0;
                                if let Token::Operator(ch) = exponent_candidate {
                                    if !['+', '-'].contains(&ch) {
                                        panic!("unexpected token `{:?}`", exponent_candidate)
                                    }
                                    if ch == '-' {
                                        sign *= -1.0;
                                    }
                                    match self.tokenizer.next() {
                                        None => panic!("unexpected end of expression"),
                                        Some(exponent) => match exponent {
                                            Token::Number(_) => exponent_candidate = exponent,
                                            _ => panic!("unexpected token `{:?}`", exponent),
                                        },
                                    }
                                }
                                if let Token::Number(exponent) = exponent_candidate {
                                    if sign < 0.0 {
                                        node = Node::Operation((
                                            Op::Exponentiation,
                                            vec![
                                                node,
                                                Node::Operation((
                                                    Op::Negation,
                                                    vec![Node::Number(exponent)],
                                                )),
                                            ],
                                        ))
                                    } else {
                                        node = Node::Operation((
                                            Op::Exponentiation,
                                            vec![node, Node::Number(exponent)],
                                        ))
                                    }
                                } else {
                                    panic!("unexpected token `{:?}`", exponent_candidate)
                                }
                            }
                        }
                    }
                }
                _ => (),
            },
            None => (),
        };

        return node;
    }
}
