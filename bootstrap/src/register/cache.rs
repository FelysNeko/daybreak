use crate::ast::node::{PegAlter, PegAtom, PegIdentified, PegItem, PegRule};

#[packrat::index]
pub enum CacheType {
    PegAlter,
    PegAtom,
    PegIdentified,
    PegItem,
    PegRule,
}

#[packrat::output]
pub enum CacheResult {
    PegAlter(Option<PegAlter>),
    PegAtom(Option<PegAtom>),
    PegIdentified(Option<PegIdentified>),
    PegItem(Option<PegItem>),
    PegRule(Option<PegRule>),
}
