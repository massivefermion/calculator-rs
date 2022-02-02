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
                Token::Operator('+') => {
                    self.tokenizer.next();
                    node = Node::Operation((Op::Addition, vec![node, self.eval_term()]));
                }

                Token::Operator('-') => {
                    self.tokenizer.next();
                    node = Node::Operation((Op::Subtraction, vec![node, self.eval_term()]));
                }

                _ => return node,
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
                Token::Operator('*') => {
                    self.tokenizer.next();
                    node = Node::Operation((Op::Multiplication, vec![node, self.eval_factor()]));
                }

                Token::Operator('/') => {
                    self.tokenizer.next();
                    node = Node::Operation((Op::Division, vec![node, self.eval_factor()]));
                }

                Token::Delimiter(Delimiter::Paranthesis(Side::Open)) | Token::Number(_) => {
                    node = Node::Operation((Op::Multiplication, vec![node, self.eval_factor()]));
                }

                _ => return node,
            }
        }
    }

    fn eval_factor(&mut self) -> Node {
        let token = self.tokenizer.next().unwrap();
        let mut node = match token {
            Token::Number(n) => Node::Number(n),
            Token::Delimiter(Delimiter::Paranthesis(Side::Open)) => {
                let node = self.eval_exp();
                let token = self.tokenizer.next();
                match token {
                    None => panic!("unexpected end of expression"),
                    Some(Token::Delimiter(Delimiter::Paranthesis(Side::Close))) => node,
                    _ => panic!("unexpected token `{:?}`", token),
                }
            }
            _ => panic!("unexpected token `{:?}`", token),
        };

        match self.tokenizer.peek() {
            Some(Token::Operator('^')) => {
                self.tokenizer.next();
                node = Node::Operation((Op::Exponentiation, vec![node, self.eval_term()]));
            }
            _ => (),
        };

        return node;
    }
}
