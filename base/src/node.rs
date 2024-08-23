use crate::cache::CacheResult;
use std::fmt::{Debug, Formatter};


#[derive(Clone)]
pub struct Alter {
    pub nameds: Vec<Named>,
    pub inline: String,
}


impl Debug for Alter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {{ {} }}", self.nameds, self.inline)
    }
}

impl From<CacheResult> for Option<Alter> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Alter(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


#[derive(Clone)]
pub enum Atom {
    String(String),
    Name(String),
}

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(s) => write!(f, "\"{}\"", s),
            Atom::Name(s) => write!(f, "{}", s),
        }
    }
}

impl From<CacheResult> for Option<Atom> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Atom(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


#[derive(Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}

impl Debug for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.rules)
    }
}

impl From<CacheResult> for Option<Grammar> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Grammar(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


#[derive(Clone)]
pub enum Named {
    Identifier(String, Atom),
    Anonymous(Atom),
    Cut,
}

impl Debug for Named {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Named::Identifier(n, i) => write!(f, "{}={:?}", n, i),
            Named::Anonymous(i) => write!(f, "{:?}", i),
            Named::Cut => write!(f, "~"),
        }
    }
}

impl From<CacheResult> for Option<Named> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Named(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub rstype: String,
    pub alters: Vec<Alter>,
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]: {:#?}", self.name, self.rstype, self.alters)
    }
}

impl From<CacheResult> for Option<Rule> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Rule(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}
