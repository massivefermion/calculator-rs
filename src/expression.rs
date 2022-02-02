use std::str::Chars;

pub struct Expression<'a> {
    pub chars: Chars<'a>,
}

impl Iterator for Expression<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some('\t') | Some(' ') => continue,
                current => return current,
            }
        }
    }
}
