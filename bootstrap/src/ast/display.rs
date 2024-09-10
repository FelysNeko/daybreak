use crate::ast::node::{PegAlter, PegAtom, PegGrammar, PegIdentified, PegItem, PegRule};
use std::fmt::{Display, Formatter};

impl Display for PegGrammar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.rules.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Display for PegIdentified {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {};", self.name, self.rule)
    }
}

impl Display for PegRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegRule::Rec {
                left,
                right
            } => write!(f, "{} \\ {}", left, right),
            PegRule::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for PegAlter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegAlter::Rec {
                left,
                right
            } => write!(f, "{} {}", left, right),
            PegAlter::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for PegItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegItem::OnceOrMore(x) => write!(f, "{}+", x),
            PegItem::ZeroOrMore(x) => write!(f, "{}*", x),
            PegItem::Optional(x) => write!(f, "{}?", x),
            PegItem::PositiveLookahead(x) => write!(f, "&{}", x),
            PegItem::NegativeLookahead(x) => write!(f, "!{}", x),
            PegItem::Plain(x) => write!(f, "{}", x),
            PegItem::Cut => write!(f, "~"),
        }
    }
}

impl Display for PegAtom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegAtom::Parentheses(x) => write!(f, "({})", x),
            PegAtom::String(x) => write!(f, "{}", x),
            PegAtom::Name(x) => write!(f, "{}", x),
        }
    }
}
