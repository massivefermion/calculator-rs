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
        let peeked = self.tokenizer.peek();

        let op = match peeked {
            None => panic!("unexpected end of expression"),
            Some(Token::Operator('-')) => {
                self.tokenizer.next();
                Some(Op::Negation)
            }
            _ => None,
        };

        let mut node = self.eval_term();
        if let Some(Op::Negation) = op {
            node = Node::Operation((Op::Negation, vec![node]));
        }

        loop {
            let peeked = self.tokenizer.peek();
            match peeked {
                Some(Token::Operator('+')) => {
                    self.tokenizer.next();
                    node = Node::Operation((Op::Addition, vec![node, self.eval_term()]));
                }

                Some(Token::Operator('-')) => {
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
            let peeked = self.tokenizer.peek();
            match peeked {
                Some(Token::Operator('*')) => {
                    self.tokenizer.next();
                    node = Node::Operation((Op::Multiplication, vec![node, self.eval_factor()]));
                }

                Some(Token::Operator('/')) => {
                    self.tokenizer.next();
                    node = Node::Operation((Op::Division, vec![node, self.eval_factor()]));
                }

                Some(Token::Delimiter(Delimiter::Paranthesis(Side::Open)))
                | Some(Token::Number(_)) => {
                    node = Node::Operation((Op::Multiplication, vec![node, self.eval_factor()]));
                }

                _ => return node,
            }
        }
    }

    fn eval_factor(&mut self) -> Node {
        let token = self.tokenizer.next();

        let mut node = match token {
            Some(Token::Number(n)) => Node::Number(n),

            Some(Token::Delimiter(Delimiter::Paranthesis(Side::Open))) => {
                let node = self.eval_exp();
                let token = self.tokenizer.next();

                match token {
                    None => panic!("unexpected end of expression"),
                    Some(Token::Delimiter(Delimiter::Paranthesis(Side::Close))) => node,
                    _ => panic!("unexpected token `{:?}`", token),
                }
            }
            None => panic!("unexpected end of expression"),
            _ => panic!("unexpected token `{:?}`", token),
        };

        match self.tokenizer.peek() {
            Some(Token::Operator('^')) => {
                self.tokenizer.next();
                node = Node::Operation((Op::Exponentiation, vec![node, self.eval_factor()]));
            }
            _ => (),
        };

        return node;
    }
}
