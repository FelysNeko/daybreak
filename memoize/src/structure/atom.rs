use crate::cache::CacheResult;
use crate::structure::Generate;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub enum Atom {
    String(String),
    Name(String),
}

impl Generate for Atom {
    fn generate(&self) -> String {
        match self {
            Atom::String(_) => todo!(),
            Atom::Name(_) => todo!()
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

impl From<CacheResult> for Option<Atom> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Atom(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}
