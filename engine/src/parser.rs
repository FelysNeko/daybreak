use crate::cache::{Cache, Verbose};
use crate::stream::Stream;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// The base parser that can be extended.
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
    /// Change the verbose level.
    pub fn v(mut self, v: Verbose) -> Self {
        self.cache.verbose = v;
        self
    }

    /// Expect a static string from the stream.
    ///
    /// It runs under strict mode except for the first character.
    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        let mode = self.stream.mode();
        let pos = self.stream.mark();
        let mut sc = s.chars();
        if self.stream.next() != sc.next() {
            self.stream.jump(pos);
            return None;
        }
        self.stream.strict(true);
        let result = || -> Option<&'static str> {
            for ch in sc {
                let ns = self.stream.next()?;
                if ns != ch {
                    self.stream.jump(pos);
                    return None;
                }
            }
            Some(s)
        }();
        self.stream.strict(mode);
        result
    }

    /// Scan a character based on given closure.
    ///
    /// `self.scan(|_| true)` is equivalent to `self.stream.next()`,
    /// and the latter is preferred.
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

    /// Lookahead without advancing the stream.
    ///
    /// If the stream reaches the end, a `'\0'` will show up.
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
    /// Build a new parser with given type generics.
    pub fn new(code: &'a str) -> Self {
        Self {
            stream: Stream {
                body: code,
                cursor: 0,
                strict: false,
            },
            cache: Cache {
                body: HashMap::new(),
                verbose: Verbose::Drop,
            },
        }
    }

    /// Clone itself without the cache, and accept new type generics.
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
            },
        }
    }
}
