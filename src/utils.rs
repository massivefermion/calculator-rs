pub const OPERATORS: [char; 5] = ['^', '*', '/', '+', '-'];
pub const DELIMITERS: [char; 2] = ['(', ')'];
pub const DECIMALS: [char; 2] = ['e', '.'];

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Operator(char),
    Delimiter(Delimiter),
}

#[derive(Debug, Clone)]
pub enum Delimiter {
    Paranthesis(Side),
}

#[derive(Debug, Clone)]
pub enum Side {
    Open,
    Close,
}

#[derive(Debug)]
pub enum Node {
    Operation((Op, Vec<Node>)),
    Number(f64),
}

#[derive(Debug)]
pub enum Op {
    Exponentiation,
    Negation,
    Multiplication,
    Division,
    Addition,
    Subtraction,
}

pub fn make_delimiter(ch: char) -> Option<Delimiter> {
    match ch {
        '(' => Some(Delimiter::Paranthesis(Side::Open)),
        ')' => Some(Delimiter::Paranthesis(Side::Close)),
        _ => None,
    }
}
