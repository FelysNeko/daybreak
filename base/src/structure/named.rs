use crate::structure::{Item, Generate};
use std::fmt::{Debug, Formatter};

pub enum Named {
    Identifier(String, Item),
    Anonymous(Item),
    Cut
}

impl Generate for Named {
    fn generate(&self) -> String {
        match self {
            Named::Identifier(_, _) => todo!(),
            Named::Anonymous(_) => todo!(),
            Named::Cut => todo!()
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
