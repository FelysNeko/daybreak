// automatically generated from templates

use crate::mapping::*;
use crate::stable::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;

mod stable;
mod mapping;


fn main() -> std::io::Result<()> {
    let input = read_to_string("../rspegen.gram")?;
    let v = true;

    let grammar = Parser {
        stream: Stream {
            body: input,
            cursor: 0,
        },
        cache: Cache {
            body: HashMap::new(),
            verbose: v,
            hit: 0,
        },
    }.grammar();
    
    if let Some(gram) = grammar {
        let json = serde_json::to_string(&gram)?;
        let mut file = File::create("ast.json")?;
        file.write_all(json.as_bytes())?;
    }

    Ok(())
}

#[allow(unused_mut)]
impl Parser {
    fn grammar(&mut self) -> Option<Grammar> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Grammar, CacheResult::Grammar, Grammar, {
            let mut cut = false;

            if let Some(result) = || -> Option<Grammar> {
                let mut i = self.insert()?;
                self.expect("NEWLINE")?;
                let mut rv = self.rule_vector()?;
                self.expect("EOF")?;
                Some(Grammar { insert: i, rules: rv })
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn insert(&mut self) -> Option<Insert> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Insert, CacheResult::Insert, Insert, {
            let mut cut = false;

            if let Some(result) = || -> Option<Insert> {
                self.expect("QUOTATION")?;
                self.expect("QUOTATION")?;
                let mut s = self.string()?;
                self.expect("QUOTATION")?;
                self.expect("QUOTATION")?;
                self.expect("NEWLINE")?;
                Some(Insert { body: s })
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn rule_vector(&mut self) -> Option<RuleVector> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::RuleVector, CacheResult::RuleVector, RuleVector, {
            let mut cut = false;

            if let Some(result) = || -> Option<RuleVector> {
                let mut r = self.rule()?;
                self.expect("NEWLINE")?;
                let mut rv = self.rule_vector()?;
                Some(chain!(rv, r))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<RuleVector> {
                let mut r = self.rule()?;
                Some(vec![r])
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn rule(&mut self) -> Option<Rule> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Rule, CacheResult::Rule, Rule, {
            let mut cut = false;

            if let Some(result) = || -> Option<Rule> {
                let mut rn = self.rule_name()?;
                self.expect(":")?;
                self.expect("NEWLINE")?;
                let mut ac = self.alter_collapse()?;
                self.expect("NEWLINE")?;
                Some(Rule { name: rn, alters: ac })
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Rule> {
                let mut rn = self.rule_name()?;
                self.expect(": ")?;
                let mut av = self.alter_vector()?;
                self.expect("NEWLINE")?;
                Some(Rule { name: rn, alters: vec![av] })
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn rule_name(&mut self) -> Option<RuleName> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::RuleName, CacheResult::RuleName, RuleName, {
            let mut cut = false;

            if let Some(result) = || -> Option<RuleName> {
                let mut n = self.name()?;
                self.expect("[")?;
                let mut rst = self.name()?;
                self.expect("]")?;
                Some(RuleName { name: n, rstype: rst })
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn alter_collapse(&mut self) -> Option<AlterCollapse> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::AlterCollapse, CacheResult::AlterCollapse, AlterCollapse, {
            let mut cut = false;

            if let Some(result) = || -> Option<AlterCollapse> {
                self.expect("    | ")?;
                let mut av = self.alter_vector()?;
                self.expect("NEWLINE")?;
                let mut ac = self.alter_collapse()?;
                Some(chain!(ac, av))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<AlterCollapse> {
                self.expect("    | ")?;
                let mut av = self.alter_vector()?;
                Some(vec![av])
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn alter_vector(&mut self) -> Option<AlterVector> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::AlterVector, CacheResult::AlterVector, AlterVector, {
            let mut cut = false;

            if let Some(result) = || -> Option<AlterVector> {
                let mut a = self.alter()?;
                self.expect(" | ")?;
                let mut ac = self.alter_vector()?;
                Some(chain!(ac, a))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<AlterVector> {
                let mut a = self.alter()?;
                Some(vec![a])
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn alter(&mut self) -> Option<Alter> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Alter, CacheResult::Alter, Alter, {
            let mut cut = false;

            if let Some(result) = || -> Option<Alter> {
                let mut nv = self.named_vector()?;
                self.expect(" ")?;
                let mut i = self.inline()?;
                Some(Alter { nameds: nv, inline: i })
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn named_vector(&mut self) -> Option<NamedVector> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::NamedVector, CacheResult::NamedVector, NamedVector, {
            let mut cut = false;

            if let Some(result) = || -> Option<NamedVector> {
                let mut n = self.named()?;
                self.expect(" ")?;
                let mut nv = self.named_vector()?;
                Some(chain!(nv, n))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<NamedVector> {
                let mut n = self.named()?;
                Some(vec![n])
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn named(&mut self) -> Option<Named> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Named, CacheResult::Named, Named, {
            let mut cut = false;

            if let Some(result) = || -> Option<Named> {
                let mut n = self.name()?;
                self.expect("=")?;
                cut = true;
                let mut i = self.item()?;
                Some(Named::Identifier { name: n, item: i })
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Named> {
                let mut i = self.item()?;
                Some(Named::Anonymous(i))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Named> {
                let mut l = self.lookahead()?;
                Some(Named::Lookahead(l))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn lookahead(&mut self) -> Option<Lookahead> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Lookahead, CacheResult::Lookahead, Lookahead, {
            let mut cut = false;

            if let Some(result) = || -> Option<Lookahead> {
                self.expect("&")?;
                cut = true;
                let mut a = self.atom()?;
                Some(Lookahead::Succeed(a))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Lookahead> {
                self.expect("!")?;
                cut = true;
                let mut a = self.atom()?;
                Some(Lookahead::Fail(a))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Lookahead> {
                self.expect("~")?;
                Some(Lookahead::Cut)
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn item(&mut self) -> Option<Item> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Item, CacheResult::Item, Item, {
            let mut cut = false;

            if let Some(result) = || -> Option<Item> {
                self.expect("[")?;
                cut = true;
                let mut av = self.alter_vector()?;
                self.expect("]")?;
                Some(Item::MultiOptional(av))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Item> {
                let mut a = self.atom()?;
                self.expect("?")?;
                Some(Item::Optional(a))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Item> {
                let mut a = self.atom()?;
                self.expect("*")?;
                Some(Item::LoopZero(a))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Item> {
                let mut a = self.atom()?;
                self.expect("+")?;
                Some(Item::LoopOne(a))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Item> {
                let mut a = self.atom()?;
                Some(Item::Atom(a))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

    fn atom(&mut self) -> Option<Atom> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Atom, CacheResult::Atom, Atom, {
            let mut cut = false;

            if let Some(result) = || -> Option<Atom> {
                self.expect("(")?;
                cut = true;
                let mut av = self.alter_vector()?;
                self.expect(")")?;
                Some(Atom::Priority(av))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Atom> {
                let mut n = self.name()?;
                Some(Atom::Name(n))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            if let Some(result) = || -> Option<Atom> {
                let mut s = self.string()?;
                Some(Atom::String(s))
            }() {
                return Some(result)
            } else {
                self.stream.cursor = origin
            }
            if cut { return None }

            None
        })
    }

}
