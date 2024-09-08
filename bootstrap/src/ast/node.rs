use pegcore::*;

#[ast]
pub struct PegGrammar {
    pub rules: Vec<PegAlter>,
}

#[ast]
pub struct PegRule {
    pub name: PegName,
    pub alters: Vec<PegAlter>,
}

#[ast]
pub enum PegAlter {
    Rec {
        prior: Box<PegAlter>,
        lower: PegItem,
    },
    Plain(PegItem),
}

#[ast]
pub enum PegItem {
    Nested(PegUnnamedRule),
    PositiveLookahead(Box<PegItem>),
    NegativeLookahead(Box<PegItem>),
    NoneOrMore(Box<PegItem>),
    OnceOrMore(Box<PegItem>),
    Optional(Box<PegItem>),
    Atomic(PegAtom),
    Cut,
}

#[ast]
pub struct PegUnnamedRule {
    pub alters: Vec<PegAlter>,
}

#[ast]
pub enum PegAtom {
    String(PegString),
    Name(PegName),
}

pub type PegString = String;
pub type PegName = String;
