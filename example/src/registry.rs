use crate::ast::*;
use daybreak::FromCR;

#[derive(Eq, PartialEq, Hash)]
pub enum CT {
    Rule,
    Alter,
    Item,
    Atom,
    Name,
    Str,
}

#[derive(Clone, FromCR)]
pub enum CR {
    Rule(Option<Rule>),
    Alter(Option<Alter>),
    Item(Option<Item>),
    Atom(Option<Atom>),
    Name(Option<Name>),
    Str(Option<Str>),
}

pub trait Base {
    type CT;
    type CR;
}

pub trait Syntax: Base {
    fn grammar(&mut self) -> Option<Gram>;
    fn non_terminal(&mut self) -> Option<NonT>;
    fn rule(&mut self) -> Option<Rule>;
    fn alternative(&mut self) -> Option<Alter>;
    fn item(&mut self) -> Option<Item>;
    fn atom(&mut self) -> Option<Atom>;
    fn name(&mut self) -> Option<Name>;
    fn str(&mut self) -> Option<Str>;
    fn char(&mut self) -> Option<Char>;
}
