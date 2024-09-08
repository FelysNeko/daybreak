use crate::cache::{Cache, Verbose};
use crate::stream::Stream;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub struct Parser<CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub stream: Stream,
    pub cache: Cache<CT, CR>,
}

impl<CT, CR> Parser<CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub fn new(code: String, v: Verbose) -> Self {
        Self {
            stream: Stream {
                body: code,
                cursor: 0,
                raw: false,
            },
            cache: Cache {
                body: HashMap::new(),
                verbose: v,
                hit: 0,
            },
        }
    }

    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        let pos = self.stream.mark();
        for ch in s.chars() {
            let n = self.stream.next();
            if !matches!(n, Some(d) if d == ch) {
                self.stream.jump(pos);
                return None;
            }
        }
        Some(s)
    }

    pub fn scan(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let pos = self.stream.mark();
        let saw = self.stream.next()?;
        if filter(saw) {
            Some(saw)
        } else {
            self.stream.jump(pos);
            None
        }
    }

    pub fn lookahead(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let saw = self.stream.body.chars()
            .nth(self.stream.cursor)
            .unwrap_or('\0');
        if filter(saw) {
            Some(saw)
        } else {
            None
        }
    }
}
