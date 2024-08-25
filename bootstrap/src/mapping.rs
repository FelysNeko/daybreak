use serde::Serialize;
use std::fmt::{Debug, Formatter};

// type mapping

#[derive(Clone, Serialize)]
pub struct Grammar {
    pub insert: String,
    pub rules: Vec<Rule>,
}

#[derive(Clone, Serialize)]
pub struct Alter {
    pub nameds: Vec<Named>,
    pub inline: String,
}

#[derive(Clone, Serialize)]
pub struct Rule {
    pub name: String,
    pub rstype: String,
    pub alters: Vec<Alter>,
}

#[derive(Clone, Serialize)]
pub enum Named {
    Identifier(String, Atom),
    Anonymous(Atom),
    Cut,
}

#[derive(Clone, Serialize)]
pub enum Atom {
    String(String),
    Name(String),
}

// debug mapping

impl Debug for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\"\"\"{}\"\"\"\n{:#?}", self.insert, self.rules)
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]: {:#?}", self.name, self.rstype, self.alters)
    }
}

impl Debug for Alter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {{ {} }}", self.nameds, self.inline)
    }
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

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::String(s) => write!(f, "\"{}\"", s),
            Atom::Name(s) => write!(f, "{}", s),
        }
    }
}

// unwrap cache result mapping

impl From<CacheResult> for Option<Grammar> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Grammar(inner) => inner,
            _ => panic!("cache not matched")
        }
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

impl From<CacheResult> for Option<Alter> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Alter(inner) => inner,
            _ => panic!("cache not matched")
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

impl From<CacheResult> for Option<Atom> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Atom(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}

// cache type mapping

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum CacheType {
    Expect(&'static str),
    Grammar,
    String,
    Inline,
    Named,
    Alter,
    Name,
    Rule,
    Atom,
}

// cache result mapping

#[derive(Clone)]
pub enum CacheResult {
    Expect(Option<()>),
    Grammar(Option<Grammar>),
    String(Option<String>),
    Inline(Option<String>),
    Named(Option<Named>),
    Alter(Option<Alter>),
    Name(Option<String>),
    Rule(Option<Rule>),
    Atom(Option<Atom>),
}

// cache result debug mapping

impl Debug for CacheResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheResult::Expect(r) => write!(f, "{:?}", r),
            CacheResult::Grammar(r) => write!(f, "{:?}", r),
            CacheResult::String(r) => write!(f, "{:?}", r),
            CacheResult::Inline(r) => write!(f, "{:?}", r),
            CacheResult::Named(r) => write!(f, "{:?}", r),
            CacheResult::Alter(r) => write!(f, "{:?}", r),
            CacheResult::Name(r) => write!(f, "{:?}", r),
            CacheResult::Rule(r) => write!(f, "{:?}", r),
            CacheResult::Atom(r) => write!(f, "{:?}", r),
        }
    }
}
