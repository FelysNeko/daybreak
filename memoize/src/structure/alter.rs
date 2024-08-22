use crate::cache::CacheResult;
use crate::structure::{indent, Generate, Named};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Alter {
    pub nameds: Vec<Named>,
    pub inline: String,
}

impl Generate for Alter {
    fn generate(&self) -> String {
        let body = self.nameds.iter()
            .map(|x| { format!("{}{}", indent(3), x.generate()) })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "{}\n\
            {}Some({})", 
            body,
            indent(3), self.inline
        )
    }
}

impl Debug for Alter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {{{}}}", self.nameds, self.inline)
    }
}

impl From<CacheResult> for Option<Alter> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Alter(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}
