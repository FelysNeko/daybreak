// automatically generated from templates
// modification required

use crate::cache::CacheResult;
use std::fmt::Debug;
use serde::Serialize;


#[derive(Clone, Debug, Serialize)]
pub struct Grammar {
    pub rules: RuleVector,
}


pub type RuleVector = Vec<Rule>;


#[derive(Clone, Debug, Serialize)]
pub struct Rule {
    pub name: RuleName,
    pub alters: AlterCollapse,
}


#[derive(Clone, Debug, Serialize)]
pub struct RuleName {
    pub name: String,
    pub rstype: String,
}


pub type AlterCollapse = Vec<AlterVector>;


pub type AlterVector = Vec<Alter>;


#[derive(Clone, Debug, Serialize)]
pub struct Alter{
    pub nameds: NamedVector,
    pub inline: String,
}


pub type NamedVector = Vec<Named>;


#[derive(Clone, Debug, Serialize)]
pub enum Named {
    Identifier {
        name: String,
        item: Item
    },
    Anonymous(Item),
    Lookahead(Lookahead)
}


#[derive(Clone, Debug, Serialize)]
pub enum Lookahead {
    Succeed(Atom),
    Fail(Atom),
    Cut,
}


#[derive(Clone, Debug, Serialize)]
pub enum Item {
    MultiOptional(AlterVector),
    Optional(Atom),
    LoopZero(Atom),
    LoopOne(Atom),
    Atom(Atom),
}


#[derive(Clone, Debug, Serialize)]
pub enum Atom {
    Priority(AlterVector),
    String(String),
    Name(String),
}


impl From<CacheResult> for Option<Grammar> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Grammar(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


impl From<CacheResult> for Option<RuleVector> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::RuleVector(inner) => inner,
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


impl From<CacheResult> for Option<RuleName> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::RuleName(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


impl From<CacheResult> for Option<AlterCollapse> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::AlterCollapse(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


impl From<CacheResult> for Option<AlterVector> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::AlterVector(inner) => inner,
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


impl From<CacheResult> for Option<NamedVector> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::NamedVector(inner) => inner,
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


impl From<CacheResult> for Option<Lookahead> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Lookahead(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}


impl From<CacheResult> for Option<Item> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Item(inner) => inner,
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

