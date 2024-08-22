use crate::cache::CacheResult;
use crate::structure::{indent, Alter, Generate};
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub rstype: String,
    pub alters: Vec<Alter>,
}

impl Generate for Rule {
    fn generate(&self) -> String {
        let body = self.alters.iter()
            .map(|x| { format!(
                "{}if let Some(result) = || -> Option<{}> {{\n\
                    {}\n\
                {}}}() {{\n\
                    {}return Some(result)\n\
                {}}} else {{\n\
                    {}self.stream.cursor = origin\n\
                {}}}\n\
                {}if cut {{ return None }}\n",
                indent(2), self.rstype,
                x.generate(),
                indent(2),
                indent(3),
                indent(2),
                indent(3),
                indent(2),
                indent(2),
            ) })
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            "pub fn {}(&mut self) -> {} {{\n\
                {}let origin = self.stream.cursor;\n\
                {}memoize!(self, CacheType::{}, CacheResult::{}, {}, {{\n\
                    {}let mut cut = false;\n\
                    {}\n\
                    {}None\n\
                {}}})\n\
            }}",
            self.name, self.rstype,
            indent(1),
            indent(1), self.rstype, self.rstype, self.rstype,
            indent(2),
            body,
            indent(2),
            indent(1)
        )
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]: {:#?}", self.name, self.rstype, self.alters)
    }
}

impl From<CacheResult> for Option<Rule> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Rule(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}