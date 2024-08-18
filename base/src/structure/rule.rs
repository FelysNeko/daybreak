use crate::structure::{Alter, Generate};
use std::fmt::{Debug, Formatter};

pub struct Rule {
    pub name: String,
    pub rstype: String,
    pub alters: Vec<Alter>,
}

impl Generate for Rule {
    fn generate(&self) -> String {
        todo!()
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]: {:#?}", self.name, self.rstype, self.alters)
    }
}
