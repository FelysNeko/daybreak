use crate::ast::node::{PegAlter, PegAtom, PegIdentified, PegItem, PegRule};

#[daybreak::ct]
pub enum CacheType {
    PegAlter,
    PegAtom,
    PegIdentified,
    PegItem,
    PegRule,
}

#[daybreak::cr]
pub enum CacheResult {
    PegAlter(Option<PegAlter>),
    PegAtom(Option<PegAtom>),
    PegIdentified(Option<PegIdentified>),
    PegItem(Option<PegItem>),
    PegRule(Option<PegRule>),
}
