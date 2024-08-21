use crate::parser::Parser;
use crate::structure::{Alter, Atom, Grammar, Item, Named, Rule};
use std::fs::read_to_string;
use std::time::Instant;

mod parser;
mod structure;


fn main() {
    let start = Instant::now();
    let source = read_to_string("pegen.gram").unwrap();
    let mut peg = Parser::new(source.as_str());
    println!("{:?}\n\nFinished in {:?}", peg.grammar(), start.elapsed());
}

impl Parser<'_> {
    fn grammar(&mut self) -> Option<Grammar> {
        let mut sandbox = self.clone();
        if let Some(grammar) = || -> Option<Grammar> {
            let mut rules = vec![sandbox.rule()?];

            let mut loopbox = sandbox.clone();
            while let (
                Some(()), Some(rule)
            ) = (
                loopbox.expect("\n"), loopbox.rule()
            ) {
                sandbox.update(loopbox.clone());
                rules.push(rule);
            }
            
            sandbox.eof()?;
            Some(Grammar { rules })
        }() {
            self.update(sandbox);
            return Some(grammar);
        }
        None
    }

    fn rule(&mut self) -> Option<Rule> {
        let mut sandbox = self.clone();
        if let Some(rule) = || -> Option<Rule> {
            let name = sandbox.name()?;
            let rstype = sandbox.rstype()?;
            sandbox.expect(": ")?;
            let alter = sandbox.alter()?;
            sandbox.expect("\n")?;
            Some(Rule { name, rstype, alters: vec![alter] })
        }() {
            self.update(sandbox);
            return Some(rule);
        }
        sandbox = self.clone();
        if let Some(rule) = || -> Option<Rule> {
            let name = sandbox.name()?;
            let rstype = sandbox.rstype()?;
            sandbox.expect(":\n    | ")?;
            let mut alters = vec![sandbox.alter()?];
            sandbox.expect("\n")?;

            let mut loopbox = sandbox.clone();
            while let (
                Some(()), Some(alter), Some(())
            ) = (
                loopbox.expect("    | "), loopbox.alter(), loopbox.expect("\n")
            ) {
                sandbox.update(loopbox.clone());
                alters.push(alter);
            }
            
            Some(Rule { name, rstype, alters })
        }() {
            self.update(sandbox);
            return Some(rule);
        }
        None
    }

    fn alter(&mut self) -> Option<Alter> {
        let mut sandbox = self.clone();
        if let Some(alter) = || -> Option<Alter> {
            let mut items = vec![sandbox.named()?];

            let mut loopbox = sandbox.clone();
            while let (
                Some(()), Some(alter)
            ) = (
                loopbox.expect(" "), loopbox.named()
            ) {
                sandbox.update(loopbox.clone());
                items.push(alter);
            }
            
            sandbox.expect(" ")?;
            let inline = sandbox.inline()?;
            Some(Alter { items, inline })
        }() {
            self.update(sandbox);
            return Some(alter);
        }
        None
    }
    
    fn named(&mut self) -> Option<Named> {
        let mut sandbox = self.clone();
        let mut cut = false;
        if let Some(named) = || -> Option<Named> {
            let name = sandbox.name()?;
            sandbox.expect("=")?;
            cut = true;
            let item = sandbox.item()?;
            Some(Named::Identifier(name, item))
        }() {
            self.update(sandbox);
            return Some(named);
        } else if cut {
            return None
        }
        sandbox = self.clone();
        if let Some(named) = || -> Option<Named> {
            let item = sandbox.item()?;
            Some(Named::Anonymous(item))
        }() {
            self.update(sandbox);
            return Some(named);
        }
        sandbox = self.clone();
        if let Some(named) = || -> Option<Named> {
            sandbox.expect("~")?;
            Some(Named::Cut)
        }() {
            self.update(sandbox);
            return Some(named);
        }
        None
    }

    fn item(&mut self) -> Option<Item> {
        let mut sandbox = self.clone();
        if let Some(item) = || -> Option<Item> {
            let atom = sandbox.atom()?;
            sandbox.expect("?")?;
            Some(Item::Optional(atom))
        }() {
            self.update(sandbox);
            return Some(item);
        }
        sandbox = self.clone();
        if let Some(item) = || -> Option<Item> {
            let atom = sandbox.atom()?;
            Some(Item::Optional(atom))
        }() {
            self.update(sandbox);
            return Some(item);
        }
        None
    }

    fn atom(&mut self) -> Option<Atom> {
        let mut sandbox = self.clone();
        if let Some(item) = || -> Option<Atom> {
            let string = sandbox.string()?;
            Some(Atom::String(string))
        }() {
            self.update(sandbox);
            return Some(item);
        }
        sandbox = self.clone();
        if let Some(item) = || -> Option<Atom> {
            let name = sandbox.name()?;
            Some(Atom::Name(name))
        }() {
            self.update(sandbox);
            return Some(item);
        }
        None
    }
}
