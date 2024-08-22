use crate::cache::CacheResult;
use crate::structure::{Generate, Rule};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
}

impl Generate for Grammar {
    fn generate(&self) -> String {
        self.rules.iter()
            .map(|x| { x.generate() })
            .collect::<Vec<String>>()
            .join("\n\n")
            .to_string()
    }
}

impl Debug for Grammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.rules)
    }
}

impl From<CacheResult> for Option<Grammar> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Grammar(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}
