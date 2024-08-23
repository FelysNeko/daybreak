use crate::cache::{Cache, CacheResult, CacheType};
use crate::memoize;
use crate::stream::Stream;
use std::collections::HashMap;

pub struct Parser {
    pub stream: Stream,
    pub cache: Cache,
}

impl Parser {
    pub fn new(input: String, v: bool) -> Self {
        Self {
            stream: Stream {
                body: input,
                cursor: 0,
            },
            cache: Cache {
                body: HashMap::new(),
                verbose: v,
                hit: 0,
            },
        }
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_str() {
        let mut parser = Parser::new("12345".to_string(), false);
        let result = parser.expect("1234");
        assert_eq!(result, Some(()));
        assert_eq!(parser.stream.cursor, 4);

        let mut parser = Parser::new("1234".to_string(), false);
        let result = parser.expect("12345");
        assert_eq!(result, None);
        assert_eq!(parser.stream.cursor, 0);
    }

    #[test]
    fn name_lexing() {
        let mut parser = Parser::new("grammar[Grammar]".to_string(), false);
        let result = parser.name();
        assert_eq!(result, Some("grammar".to_string()));
        assert_eq!(parser.stream.cursor, 7);

        let mut parser = Parser::new("[Grammar]".to_string(), false);
        let result = parser.name();
        assert_eq!(result, None);
        assert_eq!(parser.stream.cursor, 0);
    }

    #[test]
    fn string_lexing() {
        let mut parser = Parser::new("\"if\"".to_string(), false);
        let result = parser.string();
        assert_eq!(result, Some("if".to_string()));
        assert_eq!(parser.stream.cursor, 4);

        let mut parser = Parser::new("\"grammar".to_string(), false);
        let result = parser.string();
        assert_eq!(result, None);
        assert_eq!(parser.stream.cursor, 0);
    }
}
