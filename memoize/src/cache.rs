use crate::node::{Alter, Atom, Grammar, Named, Rule};
use colored::Colorize;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

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

pub struct Cache {
    pub body: HashMap<(usize, CacheType), (CacheResult, usize)>,
    pub verbose: bool,
    pub hit: usize,
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
