use crate::structure::{Alter, Generate};
use std::fmt::{Debug, Formatter};

pub struct Rule {
    name: String,
    rstype: String,
    alters: Vec<Alter>,
}

impl Generate for Rule {
    fn generate(&self) -> String {
        todo!()
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
