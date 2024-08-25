// automatically generated from templates

use std::iter::Skip;
use std::str::Chars;
use crate::cache::{Cache, CacheResult, CacheType};
use crate::memoize;

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


pub struct Parser {
    pub stream: Stream,
    pub cache: Cache,
}

#[allow(clippy::redundant_closure_call)]
impl Parser {
    pub fn expect(&mut self, s: &'static str) -> Option<()> {
        memoize!(self, CacheType::Expect(s), CacheResult::Expect, (), {
            if s == "EOF" {
                return if self.stream.peek().is_none() {
                    Some(())
                } else {
                    None
                }
            }

            let s = match s {
                "NEWLINE" => "\n",
                other => other
            };
            
            let length = s.len();
            let mut lhs = self.stream.skip();
            let mut rhs = s.chars();
            for _ in 0..length {
                if lhs.next() != rhs.next() {
                    return None;
                }
            }
            self.stream.cursor += length;
            Some(())
        })
    }

    pub fn name(&mut self) -> Option<String> {
        memoize!(self, CacheType::Name, CacheResult::Name, String, {
            let mut buffer = String::new();
            while let Some(ch) = self.stream.peek() {
                if matches!(ch, 'a'..='z' | 'A'..='Z' | '_') {
                    self.stream.cursor += 1;
                    buffer.push(ch);
                } else {
                    break;
                }
            }
            if buffer.is_empty() {
                None
            } else {
                Some(buffer)
            }
        })
    }

    pub fn string(&mut self) -> Option<String> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::String, CacheResult::String, String, {
            if self.stream.peek() == Some('"') {
                self.stream.cursor += 1;
            } else {
                return None;
            }
            let mut buffer = String::new();
            while let Some(ch) = self.stream.peek() {
                self.stream.cursor += 1;
                if matches!(ch, '"') {
                    return Some(buffer);
                } else {
                    buffer.push(ch);
                }
            }
            self.stream.cursor = origin;
            None
        })
    }

    pub fn inline(&mut self) -> Option<String> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Inline, CacheResult::Inline, String, {
            if self.stream.peek() == Some('{') {
                self.stream.cursor += 1;
            } else {
                return None;
            }
            let mut counter = 0;
            let mut buffer = String::new();
            while let Some(ch) = self.stream.peek() {
                self.stream.cursor += 1;
                match ch {
                    '{' => counter += 1,
                    '}' => counter -= 1,
                    _ => ()
                }
                if counter == -1 {
                    return Some(buffer.trim().to_string());
                } else {
                    buffer.push(ch)
                }
            }
            self.stream.cursor = origin;
            None
        })
    }
}


impl From<CacheResult> for Option<String> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::String(inner) => inner,
            CacheResult::Name(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}

impl From<CacheResult> for Option<()> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Expect(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}

