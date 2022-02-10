use std::iter::Enumerate;
use std::str::Chars;

pub struct Expression<'a> {
    pub chars: Enumerate<Chars<'a>>,
}

impl Expression<'_> {
    pub fn new(expr: &str) -> Expression {
        Expression {
            chars: expr.chars().enumerate(),
        }
    }
}

impl Iterator for Expression<'_> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some((_, '\t')) | Some((_, ' ')) => continue,
                current => return current,
            }
        }
    }
}
