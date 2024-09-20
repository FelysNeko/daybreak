use crate::memo::Memo;
use crate::stream::Stream;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct Parser<'a, CR: Eq + Hash, CT: Clone> {
    pub stream: Stream<'a>,
    pub memo: Memo<CR, CT>,
}

impl<CR: Eq + Hash, CT: Clone> Parser<'_, CR, CT> {
    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        let pos = self.stream.cursor;
        if s.chars().all(|ch| self.stream.next() == Some(ch)) {
            Some(s)
        } else {
            self.stream.cursor = pos;
            None
        }
    }

    pub fn scan(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let pos = self.stream.cursor;
        let saw = self.stream.next()?;
        if filter(saw) {
            Some(saw)
        } else {
            self.stream.cursor = pos;
            None
        }
    }

    pub fn lookahead(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let pos = self.stream.cursor;
        let saw = self.stream.next().unwrap_or('\0');
        self.stream.cursor = pos;
        if filter(saw) {
            Some(saw)
        } else {
            None
        }
    }
}

impl<'a, CR: Eq + Hash, CT: Clone> Parser<'a, CR, CT> {
    pub fn new(code: &'a str) -> Self {
        Self {
            stream: Stream {
                body: code,
                cursor: 0,
            },
            memo: Memo {
                body: HashMap::default(),
            },
        }
    }
}
