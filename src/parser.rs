use crate::tokenizer::Tokenizer;
use crate::utils::{Delimiter, Side, Token, TokenType};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl Parser<'_> {
    pub fn new(expr: &str) -> Parser {
        let tokenizer = Tokenizer::new(expr);
        Parser { tokenizer }
    }
    pub fn eval(&mut self) -> f64 {
        self.eval_exp()
    }

    fn eval_exp(&mut self) -> f64 {
        let peeked = self.tokenizer.peek();

        let sign = match peeked {
            None => panic!("unexpected end of expression"),
            Some(Token {
                pos: _,
                token: TokenType::Operator('-'),
            }) => {
                self.tokenizer.next();
                -1.0
            }
            _ => 1.0,
        };

        let mut result = self.eval_term();
        result *= sign;

        loop {
            let peeked = self.tokenizer.peek();
            match peeked {
                Some(Token {
                    pos: _,
                    token: TokenType::Operator('+'),
                }) => {
                    self.tokenizer.next();
                    result += self.eval_term();
                }

                Some(Token {
                    pos: _,
                    token: TokenType::Operator('-'),
                }) => {
                    self.tokenizer.next();
                    result -= self.eval_term();
                }

                _ => return result,
            }
        }
    }

    fn eval_term(&mut self) -> f64 {
        let mut result = self.eval_factor();

        loop {
            let peeked = self.tokenizer.peek();
            match peeked {
                Some(Token {
                    pos: _,
                    token: TokenType::Operator('*'),
                }) => {
                    self.tokenizer.next();
                    result *= self.eval_factor();
                }

                Some(Token {
                    pos: _,
                    token: TokenType::Operator('/'),
                }) => {
                    self.tokenizer.next();
                    result /= self.eval_factor();
                }

                Some(Token {
                    pos: _,
                    token: TokenType::Delimiter(Delimiter::Paranthesis(Side::Open)),
                })
                | Some(Token {
                    pos: _,
                    token: TokenType::Number(_),
                }) => {
                    result *= self.eval_factor();
                }

                _ => return result,
            }
        }
    }

    fn eval_factor(&mut self) -> f64 {
        let token = self.tokenizer.next();

        let mut result = match token {
            Some(Token {
                pos: _,
                token: TokenType::Number(n),
            }) => n,

            Some(Token {
                pos: _,
                token: TokenType::Delimiter(Delimiter::Paranthesis(Side::Open)),
            }) => {
                let result = self.eval_exp();
                let token = self.tokenizer.next();

                match token {
                    None => panic!("unexpected end of expression"),
                    Some(Token {
                        pos: _,
                        token: TokenType::Delimiter(Delimiter::Paranthesis(Side::Close)),
                    }) => result,
                    Some(Token { pos, token }) => {
                        panic!("unexpected token `{:?}` at {}", token, pos)
                    }
                }
            }
            None => panic!("unexpected end of expression"),
            Some(Token { pos, token }) => panic!("unexpected token `{:?}` at {}", token, pos),
        };

        match self.tokenizer.peek() {
            Some(Token {
                pos: _,
                token: TokenType::Operator('^'),
            }) => {
                self.tokenizer.next();
                result = f64::powf(result, self.eval_factor());
            }
            _ => (),
        };

        return result;
    }
}
