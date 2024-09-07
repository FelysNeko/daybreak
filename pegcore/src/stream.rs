pub struct Stream {
    pub(crate) body: String,
    pub(crate) cursor: usize,
    pub(crate) raw: bool,
}

impl Stream {
    pub fn mark(&self) -> usize {
        self.cursor
    }

    pub fn jump(&mut self, pos: usize) {
        self.cursor = pos
    }

    pub fn raw(&mut self, s: bool) {
        self.raw = s
    }
}

impl Iterator for Stream {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let it = self.body.chars().skip(self.cursor);
        for ch in it {
            self.cursor += 1;
            if self.raw || !ch.is_whitespace() {
                return Some(ch);
            }
        }
        None
    }
}
