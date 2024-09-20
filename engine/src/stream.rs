pub struct Stream<'a> {
    pub(crate) body: &'a str,
    pub cursor: usize,
}

impl Iterator for Stream<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.body.chars().nth(self.cursor)?;
        self.cursor += 1;
        Some(ch)
    }
}

impl Stream<'_> {
    pub fn trim(&mut self) {
        for ch in self.body.chars().skip(self.cursor) {
            if ch.is_whitespace() {
                self.cursor += 1;
            } else {
                break;
            }
        }
    }
}
