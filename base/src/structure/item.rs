use crate::structure::{Atom, Generate};
use std::fmt::{Debug, Formatter};

pub enum Item {
    Optional(Atom),
    LoopZero(Atom),
    LoopOnce(Atom),
    Exact(Atom),
}

impl Generate for Item {
    fn generate(&self) -> String {
        match self {
            Item::Optional(_) => todo!(),
            Item::LoopZero(_) => todo!(),
            Item::LoopOnce(_) => todo!(),
            Item::Exact(_) => todo!(),
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Optional(a) => write!(f, "{:?}", a),
            Item::LoopZero(a) => write!(f, "{:?}", a),
            Item::LoopOnce(a) => write!(f, "{:?}", a),
            Item::Exact(a) => write!(f, "{:?}", a),
        }
    }
}
