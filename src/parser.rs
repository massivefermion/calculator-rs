use crate::tokenizer::Tokenizer;
use crate::utils::{Delimiter, Side, Token};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl Parser<'_> {
    pub fn new(expr: &str) -> Parser {
        let tokenizer = Tokenizer::new(expr).peekable();
        Parser { tokenizer }
    }

    pub fn eval(&mut self) -> f64 {
        if self.tokenizer.peek().is_none() {
            panic!("no expression given");
        }
        self.eval_exp()
    }

    fn eval_exp(&mut self) -> f64 {
        let peeked = self.tokenizer.peek().unwrap();
        let mut sign = 1.0;
        if let Token::Operator(ch) = peeked {
            if *ch == '-' {
                sign *= -1.0;
                self.tokenizer.next();
            }
        }
        let mut result = self.eval_term();
        result *= sign;
        loop {
            if self.tokenizer.peek().is_none() {
                return result;
            }
            let peeked = self.tokenizer.peek().unwrap().clone();
            match peeked {
                Token::Operator(op) => {
                    if ['+', '-'].contains(&op) {
                        self.tokenizer.next();
                        let right_side = self.eval_term();
                        match op {
                            '+' => {
                                result += right_side;
                            }
                            '-' => {
                                result -= right_side;
                            }
                            _ => {
                                return result;
                            }
                        }
                    } else {
                        return result;
                    }
                }
                _ => {
                    return result;
                }
            }
        }
    }

    fn eval_term(&mut self) -> f64 {
        let mut result = self.eval_factor();
        loop {
            if self.tokenizer.peek().is_none() {
                return result;
            }
            let peeked = self.tokenizer.peek().unwrap().clone();
            match peeked {
                Token::Operator(op) => {
                    if ['*', '/'].contains(&op) {
                        self.tokenizer.next();
                        let right_side = self.eval_factor();
                        match op {
                            '*' => {
                                result *= right_side;
                            }
                            '/' => {
                                result /= right_side;
                            }
                            _ => {
                                return result;
                            }
                        }
                    } else {
                        return result;
                    }
                }
                _ => {
                    return result;
                }
            }
        }
    }

    fn eval_factor(&mut self) -> f64 {
        let token = self.tokenizer.next().unwrap();
        let mut result = match token {
            Token::Number(n) => n,
            Token::Delimiter(delim) => match delim {
                Delimiter::Paranthesis(side) => match side {
                    Side::Open => {
                        let result = self.eval_exp();
                        let token = self.tokenizer.next();
                        match token {
                            None => panic!("unexpected end of expression"),
                            Some(peeked) => match peeked {
                                Token::Delimiter(delim) => match delim {
                                    Delimiter::Paranthesis(side) => match side {
                                        Side::Close => result,
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
                                    result = f64::powf(result, sign * exponent);
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

        return result;
    }
}
