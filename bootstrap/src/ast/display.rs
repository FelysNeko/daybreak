use crate::ast::node::*;
use std::fmt::{Display, Formatter};

impl Display for PegGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .rules
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n\n");
        write!(f, "{}", s)
    }
}

impl Display for PegRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .alters
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n    \\ ");
        write!(f, "{}:{}", self.name, s)
    }
}

impl Display for PegAlter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .items
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", s)
    }
}

impl Display for PegItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegItem::Nested(x) => write!(f, "{}", x),
            PegItem::Atomic(x) => write!(f, "{}", x),
            PegItem::PositiveLookahead(x) => write!(f, "&{}", x),
            PegItem::NegativeLookahead(x) => write!(f, "!{}", x),
            PegItem::NoneOrMore(x) => write!(f, "{}*", x),
            PegItem::OnceOrMore(x) => write!(f, "{}+", x),
            PegItem::Optional(x) => write!(f, "{}?", x),
            PegItem::Cut => write!(f, "~"),
        }
    }
}

impl Display for PegUnnamedRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .alters
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", s)
    }
}

impl Display for PegAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegAtom::String(x) => write!(f, "\"{}\"", x),
            PegAtom::Name(x) => write!(f, "{}", x),
        }
    }
}
