use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone)]
pub struct Parser<'a> {
    pub cursor: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { cursor: input.chars().peekable() }
    }

    pub fn update(&mut self, sandbox: Parser<'a>) {
        self.cursor = sandbox.cursor
    }
}

impl Parser<'_> {
    pub fn name(&mut self) -> Option<String> {
        let mut sandbox = self.clone();
        let mut buffer = String::new();
        while let Some(ch) = sandbox.cursor.peek() {
            if matches!(ch, 'a'..='z' | 'A'..='Z' | '_') {
                buffer.push(ch.to_owned());
                sandbox.cursor.next();
            } else {
                break;
            }
        }
        if buffer.is_empty() {
            None
        } else {
            self.update(sandbox);
            Some(buffer)
        }
    }

    pub fn string(&mut self) -> Option<String> {
        let mut sandbox = self.clone();
        let mut buffer = String::new();
        sandbox.expect("\"")?;
        while let Some(ch) = sandbox.cursor.peek() {
            if !matches!(ch, '"') {
                buffer.push(ch.to_owned());
                sandbox.cursor.next();
            } else {
                break;
            }
        }
        sandbox.expect("\"")?;
        if buffer.is_empty() {
            None
        } else {
            self.update(sandbox);
            Some(buffer)
        }
    }

    pub fn inline(&mut self) -> Option<String> {
        let mut sandbox = self.clone();
        let mut buffer = String::new();
        let mut counter = 0;
        sandbox.expect("{")?;
        while let Some(ch) = sandbox.cursor.peek() {
            match ch {
                '{' => counter += 1,
                '}' => counter -= 1,
                _ => ()
            }
            if counter == -1 {
                break;
            } else {
                buffer.push(ch.to_owned());
                sandbox.cursor.next();
            }
        }
        sandbox.expect("}")?;
        if buffer.is_empty() {
            None
        } else {
            self.update(sandbox);
            Some(buffer)
        }
    }

    pub fn rstype(&mut self) -> Option<String> {
        let mut sandbox = self.clone();
        let mut buffer = String::new();
        sandbox.expect("[")?;
        while let Some(ch) = sandbox.cursor.peek() {
            if !matches!(ch, ']') {
                buffer.push(ch.to_owned());
                sandbox.cursor.next();
            } else {
                break;
            }
        }
        sandbox.expect("]")?;
        if buffer.is_empty() {
            None
        } else {
            self.update(sandbox);
            Some(buffer)
        }
    }

    pub fn expect(&mut self, s: &str) -> Option<()> {
        let mut sandbox = self.clone();
        for ch in s.chars() {
            if Some(&ch) == sandbox.cursor.peek() {
                sandbox.cursor.next();
            } else {
                return None;
            }
        }
        self.update(sandbox);
        Some(())
    }

    pub fn eof(&mut self) -> Option<()> {
        if self.cursor.peek().is_none() {
            Some(())
        } else {
            None
        }
    }
}
