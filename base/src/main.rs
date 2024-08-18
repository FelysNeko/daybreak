use crate::parser::Parser;
use crate::structure::{Alter, Atom, Grammar, Item, Rule};
use std::fs::read_to_string;

mod parser;
mod structure;


fn main() {
    let source = read_to_string("example.gram").unwrap();
    let mut peg = Parser::new(source.as_str());
    println!("{:?}", peg.grammar());
}

impl Parser<'_> {
    fn grammar(&mut self) -> Option<Grammar> {
        let mut sandbox = self.clone();
        let mut rules = vec![sandbox.rule()?];
        while let Some(rule) = sandbox.rule() {
            rules.push(rule)
        }
        sandbox.eof()?;
        self.update(sandbox);
        Some(Grammar { rules })
    }

    fn rule(&mut self) -> Option<Rule> {
        let mut sandbox = self.clone();
        let name = sandbox.name()?;
        let rstype = sandbox.rstype()?;
        sandbox.expect(':')?;
        sandbox.expect('\n')?;
        sandbox.expect(' ')?;
        sandbox.expect(' ')?;
        sandbox.expect(' ')?;
        sandbox.expect(' ')?;
        sandbox.expect('|')?;
        sandbox.expect(' ')?;
        let mut alters = vec![sandbox.alter()?];
        sandbox.expect('\n')?;
        while sandbox.expect(' ').is_some() {
            sandbox.expect(' ')?;
            sandbox.expect(' ')?;
            sandbox.expect(' ')?;
            sandbox.expect('|')?;
            sandbox.expect(' ')?;
            alters.push(sandbox.alter()?);
            sandbox.expect('\n')?;
        }
        sandbox.expect('\n')?;
        self.update(sandbox);
        Some(Rule { name, rstype, alters })
    }

    fn alter(&mut self) -> Option<Alter> {
        let mut sandbox = self.clone();
        let mut items = vec![sandbox.item()?];
        sandbox.expect(' ')?;
        while let Some(item) = sandbox.item() {
            items.push(item);
            sandbox.expect(' ')?;
        }
        let inline = sandbox.inline()?;
        self.update(sandbox);
        Some(Alter { items, inline })
    }

    fn item(&mut self) -> Option<Item> {
        let mut sandbox = self.clone();
        let atom = sandbox.atom()?;
        self.update(sandbox);
        Some(Item::Exact(atom))
    }

    fn atom(&mut self) -> Option<Atom> {
        let mut sandbox = self.clone();
        if let Some(string) = sandbox.string() {
            self.update(sandbox);
            return Some(Atom::String(string));
        }
        if let Some(name) = sandbox.name() {
            self.update(sandbox);
            return Some(Atom::Name(name));
        }
        None
    }
}
