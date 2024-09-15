use std::fmt::{Display, Formatter};

#[daybreak::ast]
pub struct PegName {
    pub name: String,
}

impl Display for PegName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
