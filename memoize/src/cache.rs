use crate::structure::{Alter, Atom, Grammar, Item, Named, Rule};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

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
    Item,
}

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
    Item(Option<Item>),
}

pub struct Cache {
    pub body: HashMap<(usize, CacheType), (CacheResult, usize)>,
    pub hit: usize,
}

impl Cache {
    pub fn get(&mut self, pos: usize, ct: CacheType) -> Option<(CacheResult, usize)> {
        if let Some((res, end)) = self.body.get(&(pos, ct)) {
            println!("hit:\t[{}] {:?} => {:?} [{}]", pos, ct, res, end);
            self.hit += 1;
            Some((res.clone(), end.to_owned()))
        } else {
            None
        }
    }

    pub fn insert(&mut self, pos: usize, ct: CacheType, res: CacheResult, end: usize) {
        if self.body.insert((pos, ct), (res, end)).is_some() {
            panic!("cache replaced")
        }
    }
}


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
            CacheResult::Item(r) => write!(f, "{:?}", r),
        }
    }
}
