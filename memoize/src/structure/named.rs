use crate::cache::CacheResult;
use crate::structure::{Generate, Item};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum Named {
    Identifier(String, Item),
    Anonymous(Item),
    Cut,
}

impl Generate for Named {
    fn generate(&self) -> String {
        match self {
            Named::Identifier(n, i) => format!("let {} = {};", n, i.generate()),
            Named::Anonymous(i) => format!("{};", i.generate()),
            Named::Cut => "cut = true;".to_string()
        }
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

impl From<CacheResult> for Option<Named> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Named(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}
