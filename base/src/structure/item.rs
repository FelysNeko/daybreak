use crate::structure::{Atom, Generate};
use std::fmt::{Debug, Formatter};

pub enum Item {
    Cut,
    Exact(Option<String>, Atom),
}

impl Generate for Item {
    fn generate(&self) -> String {
        match self {
            Item::Cut => todo!(),
            Item::Exact(_, _) => todo!(),
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::Cut => write!(f, "cut"),
            Item::Exact(n, a) => match n {
                Some(n) => write!(f, "{}={:?}", n, a),
                None => write!(f, "{:?}", a)
            },
        }
    }
}
