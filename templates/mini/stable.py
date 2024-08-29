from typing import IO
from templates.shared import CLAIM, Generator


class Stable(Generator):
    __body = '''
use crate::mapping::{CacheResult, CacheType};
use colored::Colorize;
use std::collections::HashMap;
use std::iter::Skip;
use std::str::Chars;


pub struct Parser {
    pub stream: Stream,
    pub cache: Cache,
}

pub struct Stream {
    pub body: String,
    pub cursor: usize,
}

pub struct Cache {
    pub body: HashMap<(usize, CacheType), (CacheResult, usize)>,
    pub verbose: bool,
    pub hit: usize,
}


impl Stream {
    pub fn skip(&mut self) -> Skip<Chars<'_>> {
        self.body.chars().skip(self.cursor)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.body.chars().nth(self.cursor)
    }
}

impl Cache {
    pub fn get(&mut self, pos: usize, ct: CacheType) -> Option<(CacheResult, usize)> {
        if let Some((res, end)) = self.body.get(&(pos, ct)) {
            if self.verbose {
                let log = format!("{}\t{}\t{:?} => {:?}", pos, end, ct, res);
                println!("{}", log.truecolor(0xff, 0xc6, 0xf4));
            }
            self.hit += 1;
            Some((res.clone(), end.to_owned()))
        } else {
            None
        }
    }

    pub fn insert(&mut self, pos: usize, ct: CacheType, res: CacheResult, end: usize) {
        if self.verbose {
            println!("{}\t{}\t{:?} => {:?}", pos, end, ct, res);
        }
        if self.body.insert((pos, ct), (res, end)).is_some() {
            panic!("cache conflicted")
        }
    }
}


#[macro_export]
macro_rules! memoize {
    ($self:ident, $ct:expr, $cr1:ident::$cr2:ident, $t:ty, $func:block) => {
        {
            let origin = $self.stream.cursor;
            let ct = $ct;

            if let Some((result, end)) = $self.cache.get(origin, ct) {
                $self.stream.cursor = end;
                return result.into();
            }

            let result = || -> Option<$t> {$func}();

            let cr = $cr1::$cr2(result.clone());
            $self.cache.insert(origin, ct, cr, $self.stream.cursor);
            result
        }
    };
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
                "NEWLINE" => "\\n",
                "QUOTATION" => "\\"",
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
                if matches!(ch, '\"') {
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
'''

    def __init__(self, peg, file: IO[str] | None = None) -> None:
        super().__init__(peg, file)

    def generate(self) -> None:
        self.print(CLAIM)
        self.print(self.__body)
