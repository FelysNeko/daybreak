use crate::cache::{Cache, Verbose};
use crate::stream::Stream;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub struct Parser<'a, CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub stream: Stream<'a>,
    pub cache: Cache<CT, CR>,
}

impl<CT, CR> Parser<'_, CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub fn v(&mut self, v: Verbose) {
        self.cache.verbose = v
    }

    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        let mut sc = s.chars();
        if self.stream.next() != sc.next() {
            return None;
        }
        let prev = self.stream.mode();
        self.stream.strict(true);
        let result = || -> Option<&'static str> {
            for ch in sc {
                let ns = self.stream.next()?;
                if ns != ch {
                    return None;
                }
            }
            Some(s)
        }();
        self.stream.strict(prev);
        result
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
        let pos = self.stream.mark();
        let saw = self.stream.next().unwrap_or('\0');
        self.stream.jump(pos);
        if filter(saw) {
            Some(saw)
        } else {
            None
        }
    }
}

impl<'a, CT, CR> Parser<'a, CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub fn new(code: &'a str) -> Self {
        Self {
            stream: Stream {
                body: code,
                cursor: 0,
                strict: false,
            },
            cache: Cache {
                body: HashMap::new(),
                verbose: Verbose::Core,
                hit: 0,
            },
        }
    }

    pub fn export<OCT, OCR>(&self) -> Parser<OCT, OCR>
    where
        OCT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
        OCR: Display + Debug + Clone,
    {
        Parser {
            stream: Stream {
                body: self.stream.body,
                cursor: self.stream.cursor,
                strict: self.stream.strict,
            },
            cache: Cache {
                body: HashMap::new(),
                verbose: self.cache.verbose,
                hit: 0,
            },
        }
    }
}
