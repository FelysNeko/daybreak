use crate::cache::{CacheResult, CacheType};
use crate::node::{Alter, Atom, Grammar, Named, Rule};
pub use crate::parser::Parser;
use crate::visitor::Visitor;

mod parser;
mod stream;
mod cache;
mod visitor;
mod node;

impl Parser {
    pub fn generate(&mut self) -> Option<(String, Vec<String>)> {
        let top = self.grammar()?;
        let types = top.rules.iter()
            .map(|x| x.rstype.clone())
            .collect::<Vec<String>>();
        let result = Visitor {
            indent: 0,
            output: vec![],
        }.grammar(top);
        Some((result, types))
    }

    fn grammar(&mut self) -> Option<Grammar> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Grammar, CacheResult::Grammar, Grammar, {
            if let Some(grammar) = || -> Option<Grammar> {
                let mut rules = vec![self.rule()?];

                let mut checkpoint = self.stream.cursor;
                while let (
                    Some(()), Some(rule)
                ) = (
                    self.expect("\n"), self.rule()
                ) {
                    checkpoint = self.stream.cursor;
                    rules.push(rule);
                }
                self.stream.cursor = checkpoint;
                
                self.expect("EOF")?;
                Some(Grammar { rules })
            }() {
                return Some(grammar);
            } else {
                self.stream.cursor = origin
            }
            None
        })
    }

    fn rule(&mut self) -> Option<Rule> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Rule, CacheResult::Rule, Rule, {
            if let Some(rule) = || -> Option<Rule> {
                let name = self.name()?;
                self.expect("[")?;
                let rstype = self.name()?;
                self.expect("]: ")?;
                let alter = self.alter()?;
                self.expect("\n")?;
                Some(Rule { name, rstype, alters: vec![alter] })
            }() {
                return Some(rule);
            } else {
                self.stream.cursor = origin
            }
            if let Some(rule) = || -> Option<Rule> {
                let name = self.name()?;
                self.expect("[")?;
                let rstype = self.name()?;
                self.expect("]:\n    | ")?;
                let mut alters = vec![self.alter()?];
                self.expect("\n")?;

                let mut checkpoint = self.stream.cursor;
                while let (
                    Some(()), Some(alter), Some(())
                ) = (
                    self.expect("    | "), self.alter(), self.expect("\n")
                ) {
                    checkpoint = self.stream.cursor;
                    alters.push(alter);
                }
                self.stream.cursor = checkpoint;

                Some(Rule { name, rstype, alters })
            }() {
                return Some(rule);
            } else {
                self.stream.cursor = origin
            }
            None
        })
    }

    fn alter(&mut self) -> Option<Alter> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Alter, CacheResult::Alter, Alter, {
            if let Some(alter) = || -> Option<Alter> {
                let mut nameds = vec![self.named()?];

                let mut checkpoint = self.stream.cursor;
                while let (
                    Some(()), Some(named)
                ) = (
                    self.expect(" "), self.named()
                ) {
                    checkpoint = self.stream.cursor;
                    nameds.push(named);
                }
                self.stream.cursor = checkpoint;

                self.expect(" ")?;
                let inline = self.inline()?;
                Some(Alter { nameds, inline })
            }() {
                return Some(alter);
            } else {
                self.stream.cursor = origin
            }
            None
        })
    }

    fn named(&mut self) -> Option<Named> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Named, CacheResult::Named, Named, {
            let mut cut = false;
            if let Some(named) = || -> Option<Named> {
                let name = self.name()?;
                self.expect("=")?;
                cut = true;
                let atom = self.atom()?;
                Some(Named::Identifier(name, atom))
            }() {
                return Some(named);
            } else {
                self.stream.cursor = origin;
            }
            if cut {
                return None;
            }
            if let Some(named) = || -> Option<Named> {
                let atom = self.atom()?;
                Some(Named::Anonymous(atom))
            }() {
                return Some(named);
            } else {
                self.stream.cursor = origin;
            }
            if let Some(named) = || -> Option<Named> {
                self.expect("~")?;
                Some(Named::Cut)
            }() {
                return Some(named);
            } else {
                self.stream.cursor = origin;
            }
            None
        })
    }

    fn atom(&mut self) -> Option<Atom> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Atom, CacheResult::Atom, Atom, {
            if let Some(atom) = || -> Option<Atom> {
                let string = self.string()?;
                Some(Atom::String(string))
            }() {
                return Some(atom);
            } else {
                self.stream.cursor = origin
            }
            if let Some(atom) = || -> Option<Atom> {
                let name = self.name()?;
                Some(Atom::Name(name))
            }() {
                return Some(atom);
            } else {
                self.stream.cursor = origin
            }
            None
        })
    }
}
