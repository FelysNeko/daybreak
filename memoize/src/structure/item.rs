use crate::cache::CacheResult;
use crate::structure::{Atom, Generate};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum Item {
    Optional(Atom),
    Exact(Atom),
}

impl Generate for Item {
    fn generate(&self) -> String {
        match self {
            Item::Optional(_) => todo!(),
            Item::Exact(_) => todo!(),
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Optional(a) => write!(f, "{:?}?", a),
            Item::Exact(a) => write!(f, "{:?}", a),
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