use crate::cache::{CacheResult, CacheType};
use crate::parser::Parser;
use crate::structure::{Alter, Atom, Generate, Grammar, Item, Named, Rule};
use colored::Colorize;
use std::fs::read_to_string;
use std::time::Instant;

mod parser;
mod stream;
mod cache;
mod structure;

fn main() {
    let start = Instant::now();
    let source = read_to_string("rspegen.gram").unwrap();
    let mut peg = Parser::new(source, true);
    if peg.cache.verbose {
        println!("Start\tEnd\tResult")
    }
    if let Some(grammar) = peg.grammar() {
        println!(
            "\n\n{}\n\n{}\n\nFinished in {:?} with {} hit from {} cache",
            "Generation Result".bold().truecolor(0xff, 0xc6, 0xf4),
            grammar.generate(), start.elapsed(),
            peg.cache.hit, peg.cache.body.len()
        )
    }
}


impl Parser {
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
                let item = self.item()?;
                Some(Named::Identifier(name, item))
            }() {
                return Some(named);
            } else {
                self.stream.cursor = origin;
            }
            if cut {
                return None;
            }
            if let Some(named) = || -> Option<Named> {
                let item = self.item()?;
                Some(Named::Anonymous(item))
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

    fn item(&mut self) -> Option<Item> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Item, CacheResult::Item, Item, {
            if let Some(item) = || -> Option<Item> {
                let atom = self.atom()?;
                self.expect("?")?;
                Some(Item::Optional(atom))
            }() {
                return Some(item);
            } else {
                self.stream.cursor = origin
            }
            if let Some(item) = || -> Option<Item> {
                let atom = self.atom()?;
                Some(Item::Exact(atom))
            }() {
                return Some(item);
            } else {
                self.stream.cursor = origin
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
