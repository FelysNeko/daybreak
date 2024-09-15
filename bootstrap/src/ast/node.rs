use utils::name::PegName;
use utils::string::PegString;

#[daybreak::ast]
pub struct PegGrammar {
    pub rules: Vec<PegIdentified>,
}

#[daybreak::ast]
pub struct PegIdentified {
    pub name: PegName,
    pub rule: PegRule,
}

#[daybreak::ast]
pub enum PegRule {
    Rec {
        left: Box<PegRule>,
        right: PegAlter,
    },
    Plain(PegAlter),
}

#[daybreak::ast]
pub enum PegAlter {
    Rec {
        left: Box<PegAlter>,
        right: PegItem,
    },
    Plain(PegItem),
}

#[daybreak::ast]
pub enum PegItem {
    OnceOrMore(PegAtom),
    ZeroOrMore(PegAtom),
    Optional(PegAtom),
    PositiveLookahead(PegAtom),
    NegativeLookahead(PegAtom),
    Plain(PegAtom),
    Cut,
}

#[daybreak::ast]
pub enum PegAtom {
    Parentheses(Box<PegRule>),
    String(PegString),
    Name(PegName),
}
