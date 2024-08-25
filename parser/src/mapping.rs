// automatically generated from templates

use serde::Serialize;
use std::fmt::{Debug, Formatter};


#[derive(Clone, Debug, Serialize)]
pub struct Grammar {
    pub insert: Insert,
    pub rules: RuleVector,
}

#[derive(Clone, Debug, Serialize)]
pub struct Insert {
    pub body: String,
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

#[macro_export]
macro_rules! chain {
    ($v:expr, $e:expr) => {
        {
            $v.push($e);
            $v
        }
    };
}

impl From<CacheResult> for Option<Grammar> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Grammar(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}

impl From<CacheResult> for Option<Insert> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Insert(inner) => inner,
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

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum CacheType {
    Expect(&'static str),
    String,
    Inline,
    Name,
    Grammar,
    Insert,
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
}

#[derive(Clone)]
pub enum CacheResult {
    Expect(Option<()>),
    String(Option<String>),
    Inline(Option<String>),
    Name(Option<String>),
    Grammar(Option<Grammar>),
    Insert(Option<Insert>),
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
            CacheResult::Expect(r) => write!(f, "{:?}", r),
            CacheResult::String(r) => write!(f, "{:?}", r),
            CacheResult::Inline(r) => write!(f, "{:?}", r),
            CacheResult::Name(r) => write!(f, "{:?}", r),
            CacheResult::Grammar(r) => write!(f, "{:?}", r),
            CacheResult::Insert(r) => write!(f, "{:?}", r),
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
        }
    }
}
