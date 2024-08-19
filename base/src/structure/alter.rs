use crate::structure::{Generate, Named};
use std::fmt::{Debug, Formatter};

pub struct Alter {
    pub items: Vec<Named>,
    pub inline: String,
}

impl Generate for Alter {
    fn generate(&self) -> String {
        todo!()
    }
}

impl Debug for Alter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {{{}}}", self.items, self.inline)
    }
}
