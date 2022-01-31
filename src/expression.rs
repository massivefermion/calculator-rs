use std::str::Chars;

pub struct Expression<'a> {
    pub chars: Chars<'a>,
}

impl Iterator for Expression<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.chars.next();
            if current.is_none() {
                return None;
            };

            let current = current.unwrap();
            if ['\t', ' '].contains(&current) {
                continue;
            }
            return Some(current);
        }
    }
}
