use std::iter::Peekable;
use std::str::Chars;

pub struct Parser<'a> {
    cursor: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { cursor: input.chars().peekable() }
    }

    pub fn sandbox(&self) -> Self {
        Self { cursor: self.cursor.clone() }
    }

    pub fn update(&mut self, sandbox: Parser<'a>) {
        self.cursor = sandbox.cursor
    }
}

impl Parser<'_> {
    pub fn name(&mut self) -> Option<String> {
        todo!()
    }

    pub fn string(&mut self) -> Option<String> {
        todo!()
    }
    
    pub fn inline(&mut self) -> Option<String> {
        todo!()
    }
    
    pub fn rstype(&mut self) -> Option<String> {
        todo!()
    }
}
