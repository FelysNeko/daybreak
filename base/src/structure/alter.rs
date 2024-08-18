use crate::structure::{Generate, Item};
use std::fmt::{Debug, Formatter};

pub struct Alter {
    items: Vec<Item>,
    inline: String,
}

impl Generate for Alter {
    fn generate(&self) -> String {
        todo!()
    }
}

impl Debug for Alter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
