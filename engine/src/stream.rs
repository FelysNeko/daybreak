/// Handwritten iterator for the character stream.
pub struct Stream<'a> {
    pub(crate) body: &'a str,
    pub(crate) cursor: usize,
    pub(crate) strict: bool,
}

impl Stream<'_> {
    /// Get the current position.
    pub fn mark(&self) -> usize {
        self.cursor
    }

    /// Cursor jump the given position.
    pub fn jump(&mut self, pos: usize) {
        self.cursor = pos
    }
    
    /// Get the current strict mode.
    pub fn mode(&mut self) -> bool {
        self.strict
    }

    /// Toggle the strict mode.
    pub fn strict(&mut self, s: bool) {
        self.strict = s
    }
}

impl Iterator for Stream<'_> {
    type Item = char;
    
    /// Get the next char based on current strict mode.
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
