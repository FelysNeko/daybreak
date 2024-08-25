// automatically generated from templates

use crate::node::*;
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


#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum CacheType {
    Grammar,
    RuleVector,
    Rule,
    RuleName,
    AlterCollapse,
    AlterVector,
    Alter,
    NamedVector,
    Named,
    Lookahead,
    Item,
    Atom,
    Expect(&'static str),
    String,
    Inline,
    Name,
}


#[derive(Clone)]
pub enum CacheResult {
    Expect(Option<()>),
    String(Option<String>),
    Inline(Option<String>),
    Name(Option<String>),
    Grammar(Option<Grammar>),
    RuleVector(Option<RuleVector>),
    Rule(Option<Rule>),
    RuleName(Option<RuleName>),
    AlterCollapse(Option<AlterCollapse>),
    AlterVector(Option<AlterVector>),
    Alter(Option<Alter>),
    NamedVector(Option<NamedVector>),
    Named(Option<Named>),
    Lookahead(Option<Lookahead>),
    Item(Option<Item>),
    Atom(Option<Atom>),
}


impl Debug for CacheResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheResult::Grammar(r) => write!(f, "{:?}", r),
            CacheResult::RuleVector(r) => write!(f, "{:?}", r),
            CacheResult::Rule(r) => write!(f, "{:?}", r),
            CacheResult::RuleName(r) => write!(f, "{:?}", r),
            CacheResult::AlterCollapse(r) => write!(f, "{:?}", r),
            CacheResult::AlterVector(r) => write!(f, "{:?}", r),
            CacheResult::Alter(r) => write!(f, "{:?}", r),
            CacheResult::NamedVector(r) => write!(f, "{:?}", r),
            CacheResult::Named(r) => write!(f, "{:?}", r),
            CacheResult::Lookahead(r) => write!(f, "{:?}", r),
            CacheResult::Item(r) => write!(f, "{:?}", r),
            CacheResult::Atom(r) => write!(f, "{:?}", r),
            CacheResult::Expect(r) => write!(f, "{:?}", r),
            CacheResult::String(r) => write!(f, "{:?}", r),
            CacheResult::Inline(r) => write!(f, "{:?}", r),
            CacheResult::Name(r) => write!(f, "{:?}", r),
        }
    }
}

