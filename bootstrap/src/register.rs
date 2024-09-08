use crate::ast::node::*;
use pegcore::*;

#[indicator]
pub enum CacheType {
    PegExpect(&'static str),
    PegGrammar,
    PegRule,
    PegUnnamedRule,
    PegAlter,
    PegItem,
    PegAtom,
    PegString,
    PegName,
}

#[output]
pub enum CacheResult {
    PegExpect(Option<&'static str>),
    PegGrammar(Option<PegGrammar>),
    PegRule(Option<PegRule>),
    PegUnnamedRule(Option<PegUnnamedRule>),
    PegAlter(Option<PegAlter>),
    PegItem(Option<PegItem>),
    PegAtom(Option<PegAtom>),
    PegString(Option<String>),
    PegName(Option<String>),
}

pub trait Wrapper: Base {
    fn peg_expect(&mut self, s: &'static str) -> Option<&'static str>;
}

pub trait Bootstrap: Base {
    fn peg_grammar(&mut self) -> Option<PegGrammar>;
    fn peg_rule(&mut self) -> Option<PegRule>;
    fn peg_unnamed_rule(&mut self) -> Option<PegUnnamedRule>;
    fn peg_alter(&mut self) -> Option<PegAlter>;
    fn peg_item(&mut self) -> Option<PegItem>;
    fn peg_atom(&mut self) -> Option<PegAtom>;
    fn peg_string(&mut self) -> Option<PegString>;
    fn peg_name(&mut self) -> Option<PegName>;
}

pub trait Base {
    type CT;
    type CR;
}
