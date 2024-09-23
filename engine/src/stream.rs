pub struct Stream<'a> {
    pub(crate) body: &'a str,
    pub cursor: usize,
    pub strict: bool,
}

impl Iterator for Stream<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let skipped = self.body.chars().skip(self.cursor);
        for ch in skipped {
            self.cursor += 1;
            if self.strict || !ch.is_whitespace() {
                return Some(ch);
            }
        }
        None
    }
}

impl Stream<'_> {
    pub fn trim(&mut self) {
        if self.strict {
            return;
        }
        for ch in self.body.chars().skip(self.cursor) {
            if ch.is_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }
}
