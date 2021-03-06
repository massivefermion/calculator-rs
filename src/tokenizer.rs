use crate::expression::Expression;
use crate::utils::{make_delimiter, Token, TokenType, DECIMALS, DELIMITERS, OPERATORS};
use std::iter::Peekable;

pub struct Tokenizer<'a> {
    expression: Peekable<Expression<'a>>,
    storage: String,
    token_start: Option<usize>,
}

impl Tokenizer<'_> {
    pub fn new(expr: &str) -> Peekable<Tokenizer> {
        let expression = Expression::new(expr).peekable();
        Tokenizer {
            expression,
            storage: String::new(),
            token_start: None,
        }
        .peekable()
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.expression.next();
            if current.is_none() {
                return None;
            }

            let (pos, current) = current.unwrap();
            if !current.is_digit(10)
                && !DECIMALS.contains(&current)
                && !DELIMITERS.contains(&current)
                && !OPERATORS.contains(&current)
                && !current.is_whitespace()
            {
                panic!("invalid character {} at {}", current, pos);
            }

            let delim = make_delimiter(current);
            if delim.is_some() {
                return Some(Token {
                    pos,
                    token: TokenType::Delimiter(delim.unwrap()),
                });
            }

            if OPERATORS.contains(&current) {
                if current != '-'
                    || self.storage.len() == 0
                    || self.storage.chars().last().unwrap() != 'e'
                {
                    return Some(Token {
                        pos,
                        token: TokenType::Operator(current),
                    });
                }
            }

            if (current == 'e' && self.storage.contains('e'))
                || (current == '.' && self.storage.contains('.'))
            {
                panic!("invalid sequence `{}{}` at {}", self.storage, current, pos);
            }

            let peeked = self.expression.peek();
            if self.storage.is_empty() {
                self.token_start = Some(pos);
            }
            self.storage.push(current);
            match peeked {
                None => {
                    if current == 'e' || current == '-' {
                        panic!("unexpected end of expression");
                    }
                }
                Some((_, ch)) => {
                    if ch.is_whitespace() || OPERATORS.contains(ch) || DELIMITERS.contains(ch) {
                        if current == 'e' {
                            if *ch == '-' {
                                continue;
                            }
                        }
                    } else {
                        continue;
                    }
                }
            }

            let token = Some(Token {
                pos: self.token_start.unwrap(),
                token: TokenType::Number(self.storage.parse().unwrap()),
            });
            self.storage = String::new();
            self.token_start = None;
            return token;
        }
    }
}
