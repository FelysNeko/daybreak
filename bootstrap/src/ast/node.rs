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
pub struct PegAlter {
    pub items: Vec<PegItem>,
}

#[ast]
pub enum PegItem {
    Nested(PegUnnamedRule),
    Atomic(PegAtom),
    PositiveLookahead(Box<PegItem>),
    NegativeLookahead(Box<PegItem>),
    NoneOrMore(Box<PegItem>),
    OnceOrMore(Box<PegItem>),
    Optional(Box<PegItem>),
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
