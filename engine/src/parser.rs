use crate::memo::Memo;
use crate::stream::Stream;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Parser<'a, CT: Eq + Hash, CR: Clone> {
    pub stream: Stream<'a>,
    pub memo: Memo<CT, CR>,
    pub cut: bool,
}

impl<CT: Eq + Hash, CR: Clone> Parser<'_, CT, CR> {
    pub fn alter<T, F>(&mut self, f: F) -> (Option<T>, bool)
    where
        F: Fn(&mut Parser<CT, CR>) -> Option<T>,
    {
        self.cut = false;
        let pos = self.stream.cursor;
        let mode = self.stream.strict;

        let result = f(self);
        let cut = self.cut;

        self.cut = false;
        if result.is_none() {
            self.stream.cursor = pos;
        }
        self.stream.strict = mode;
        (result, cut)
    }

    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        let (res, cut) = self.alter(|x| {
            x.stream.trim();
            x.stream.strict = true;
            if s.chars().all(|c| x.stream.next() == Some(c)) {
                Some(s)
            } else {
                None
            }
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    pub fn scan<F>(&mut self, filter: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        let pos = self.stream.cursor;
        let saw = self.stream.next()?;
        if filter(saw) {
            Some(saw)
        } else {
            self.stream.cursor = pos;
            None
        }
    }

    pub fn lookahead<F>(&mut self, filter: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
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
                strict: false,
            },
            memo: Memo {
                body: HashMap::default(),
            },
            cut: false,
        }
    }
}


#[test]
fn test() {
    let mut x = Parser::<i32, i32>::new(" 123 ");
    x.expect("123");
    println!("{} {}", x.stream.cursor, x.stream.strict)
}