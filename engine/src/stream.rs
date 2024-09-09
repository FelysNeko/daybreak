pub struct Stream<'a> {
    pub(crate) body: &'a str,
    pub(crate) cursor: usize,
    pub(crate) strict: bool,
}

impl Stream<'_> {
    pub fn mark(&self) -> usize {
        self.cursor
    }

    pub fn jump(&mut self, pos: usize) {
        self.cursor = pos
    }

    pub fn mode(&mut self) -> bool {
        self.strict
    }

    pub fn strict(&mut self, s: bool) {
        self.strict = s
    }
}

impl Iterator for Stream<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let it = self.body.chars().skip(self.cursor);
        for ch in it {
            self.cursor += 1;
            if self.strict || !ch.is_whitespace() {
                return Some(ch);
            }
        }
        None
    }
}
