use std::iter::Skip;
use std::str::Chars;

pub struct Stream {
    pub body: String,
    pub cursor: usize,
}

impl Stream {
    pub fn skip(&mut self) -> Skip<Chars<'_>> {
        self.body.chars().skip(self.cursor)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.body.chars().nth(self.cursor)
    }
}
