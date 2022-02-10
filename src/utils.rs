use serde::Serialize;

pub const OPERATORS: [char; 5] = ['^', '*', '/', '+', '-'];
pub const DELIMITERS: [char; 2] = ['(', ')'];
pub const DECIMALS: [char; 2] = ['e', '.'];

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub token: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    Number(f64),
    Operator(char),
    Delimiter(Delimiter),
}

#[derive(Debug)]
pub enum Delimiter {
    Paranthesis(Side),
}

#[derive(Debug)]
pub enum Side {
    Open,
    Close,
}

#[derive(Debug, Serialize)]
pub struct Node {
    pub pos: usize,
    pub node: NodeType,
}

#[derive(Debug, Serialize)]
pub enum NodeType {
    Operation(Op, Vec<Node>),
    Number(f64),
}

#[derive(Debug, Serialize)]
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
