use crate::structure::{Generate, Rule};
use std::fmt::{Debug, Formatter};

pub struct Grammar {
    pub rules: Vec<Rule>,
}

impl Generate for Grammar {
    fn generate(&self) -> String {
        todo!()
    }
}

impl Debug for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.rules)
    }
}
