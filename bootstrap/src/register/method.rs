use crate::ast::node::{PegAlter, PegAtom, PegGrammar, PegIdentified, PegItem, PegRule};

pub trait Base {
    type CT;
    type CR;
}

pub trait Bootstrap: Base {
    fn peg_grammar(&mut self) -> Option<PegGrammar>;
    fn peg_identified(&mut self) -> Option<PegIdentified>;
    fn peg_rule(&mut self) -> Option<PegRule>;
    fn peg_alter(&mut self) -> Option<PegAlter>;
    fn peg_item(&mut self) -> Option<PegItem>;
    fn peg_atom(&mut self) -> Option<PegAtom>;
}
