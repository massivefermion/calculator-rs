use crate::expression::Expression;
use crate::utils::{make_delimiter, Token, DECIMALS, DELIMITERS, OPERATORS};
use std::iter::Peekable;

pub struct Tokenizer<'a> {
    expression: Peekable<Expression<'a>>,
    storage: String,
}

impl Tokenizer<'_> {
    pub fn new(expr: &str) -> Tokenizer {
        let iter_expression = Expression {
            chars: expr.chars(),
        };
        let expression = iter_expression.peekable();
        Tokenizer {
            expression,
            storage: String::new(),
        }
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

            let current = current.unwrap();
            if !current.is_digit(10)
                && !DECIMALS.contains(&current)
                && !DELIMITERS.contains(&current)
                && !OPERATORS.contains(&current)
                && !current.is_whitespace()
            {
                panic!("invalid character");
            }

            let delim = make_delimiter(current);
            if delim.is_some() {
                return Some(Token::Delimiter(delim.unwrap()));
            }

            if OPERATORS.contains(&current) {
                if current != '-'
                    || self.storage.len() == 0
                    || self.storage.chars().last().unwrap() != 'e'
                {
                    return Some(Token::Operator(current));
                }
            }

            if (current == 'e' && self.storage.contains('e'))
                || (current == '.' && self.storage.contains('.'))
            {
                panic!("invalid sequence `{}{}`", self.storage, current);
            }

            let peeked = self.expression.peek();
            self.storage.push(current);
            match peeked {
                None => (),
                Some(ch) => {
                    if ch.is_whitespace() || OPERATORS.contains(ch) || DELIMITERS.contains(ch) {
                        if *ch == '-' && current == 'e' {
                            continue;
                        }
                        ()
                    } else {
                        continue;
                    }
                }
            }

            let token = Some(Token::Number(self.storage.parse().unwrap()));
            self.storage = String::new();
            return token;
        }
    }
}
