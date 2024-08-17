use std::fmt::{Debug, Formatter};
use std::iter::Peekable;
use std::str::Chars;


pub struct Grammar {
    body: Vec<Rule>,
}

impl Debug for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.body)
    }
}

pub struct Rule {
    name: String,
    body: Vec<Alter>,
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.name, self.body)
    }
}

pub struct Alter {
    body: Vec<Item>,
}

impl Debug for Alter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.body)
    }
}

pub enum Item {
    String(String),
    Name(String),
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::String(s) => write!(f, "\"{}\"", s),
            Item::Name(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Clone)]
pub struct Parser<'a> {
    cursor: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { cursor: input.chars().peekable() }
    }

    fn eat(&mut self, c: char) -> bool {
        self.cursor.next() == Some(c)
    }

    fn eof(&mut self) -> bool {
        self.cursor.next().is_none()
    }

    fn catchup(&mut self, sandbox: Parser<'a>) {
        self.cursor = sandbox.cursor
    }
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Option<Grammar> {
        self.grammar()
    }

    fn grammar(&mut self) -> Option<Grammar> {
        let mut sandbox = self.clone();
        let mut body = vec![sandbox.rule()?];
        while let Some(rule) = sandbox.rule() {
            body.push(rule)
        }
        if !sandbox.eof() {
            return None;
        }
        self.catchup(sandbox);
        Some(Grammar { body })
    }

    fn rule(&mut self) -> Option<Rule> {
        let mut sandbox = self.clone();
        let name = sandbox.name()?;
        if !sandbox.eat(':') {
            return None;
        }
        let mut body = vec![sandbox.alter()?];
        while sandbox.cursor.peek() == Some(&'|') {
            sandbox.cursor.next();
            body.push(sandbox.alter()?)
        }
        if !sandbox.eat('\n') {
            return None;
        }
        self.catchup(sandbox);
        Some(Rule { name, body })
    }

    fn alter(&mut self) -> Option<Alter> {
        let mut sandbox = self.clone();
        let mut body = vec![sandbox.item()?];
        while let Some(item) = sandbox.item() {
            body.push(item)
        }
        self.catchup(sandbox);
        Some(Alter { body })
    }

    fn item(&mut self) -> Option<Item> {
        let mut sandbox = self.clone();
        let item = if let Some(name) = sandbox.name() {
            Item::Name(name)
        } else if let Some(string) = sandbox.string() {
            Item::String(string)
        } else {
            return None;
        };
        self.catchup(sandbox);
        Some(item)
    }

    fn name(&mut self) -> Option<String> {
        let mut sandbox = self.clone();
        let mut body = String::new();
        while let Some(c) = sandbox.cursor.peek() {
            if matches!(c, 'a'..='z' | 'A'..='Z' | '_') {
                body.push(*c);
                sandbox.cursor.next();
            } else {
                break;
            }
        }
        if body.is_empty() {
            return None;
        }
        self.catchup(sandbox);
        Some(body)
    }

    fn string(&mut self) -> Option<String> {
        let mut sandbox = self.clone();
        let mut body = String::new();
        if !sandbox.eat('\'') {
            return None;
        }
        for c in sandbox.cursor.by_ref() {
            if c != '\'' {
                body.push(c)
            } else {
                break;
            }
        }
        self.catchup(sandbox);
        Some(body)
    }
}
