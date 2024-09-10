use utils::name::PegName;
use utils::string::PegString;

#[packrat::ast]
pub struct PegGrammar {
    pub rules: Vec<PegIdentified>,
}

#[packrat::ast]
pub struct PegIdentified {
    pub name: PegName,
    pub rule: PegRule,
}

#[packrat::ast]
pub enum PegRule {
    Rec {
        left: Box<PegRule>,
        right: PegAlter,
    },
    Plain(PegAlter),
}

#[packrat::ast]
pub enum PegAlter {
    Rec {
        left: Box<PegAlter>,
        right: PegItem,
    },
    Plain(PegItem),
}

#[packrat::ast]
pub enum PegItem {
    OnceOrMore(PegAtom),
    ZeroOrMore(PegAtom),
    Optional(PegAtom),
    PositiveLookahead(PegAtom),
    NegativeLookahead(PegAtom),
    Plain(PegAtom),
    Cut,
}

#[packrat::ast]
pub enum PegAtom {
    Parentheses(Box<PegRule>),
    String(PegString),
    Name(PegName),
}
