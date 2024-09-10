use crate::ast::node::{PegAlter, PegAtom, PegGrammar, PegIdentified, PegItem, PegRule};
use crate::register::cache::{CacheResult, CacheType};
use crate::register::method::Bootstrap;
use engine::Parser;
use utils::{name, string};

impl Bootstrap for Parser<'_, CacheType, CacheResult> {
    fn peg_grammar(&mut self) -> Option<PegGrammar> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegGrammar> {
            let first = self.peg_identified()?;
            let mut rules = vec![first];
            while let Some(rule) = self.peg_identified() {
                rules.push(rule)
            }
            if self.stream.next().is_some() {
                return None;
            }
            Some(PegGrammar { rules })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[packrat::memoize(PegIdentified)]
    fn peg_identified(&mut self) -> Option<PegIdentified> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegIdentified> {
            let name = name::parse(self)?;
            self.expect(":")?;
            self.expect("\\");
            let rule = self.peg_rule()?;
            self.expect(";")?;
            Some(PegIdentified { name, rule })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[packrat::lecursion(PegRule)]
    fn peg_rule(&mut self) -> Option<PegRule> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegRule> {
            let left = self.peg_rule()?;
            self.expect("\\")?;
            let right = self.peg_alter()?;
            Some(PegRule::Rec { left: Box::new(left), right })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegRule> {
            let plain = self.peg_alter()?;
            Some(PegRule::Plain(plain))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[packrat::strict]
    #[packrat::lecursion(PegAlter)]
    fn peg_alter(&mut self) -> Option<PegAlter> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegAlter> {
            let left = self.peg_alter()?;
            self.stream.strict(true);
            self.expect(" ")?;
            self.stream.strict(false);
            let right = self.peg_item()?;
            Some(PegAlter::Rec { left: Box::new(left), right })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        self.stream.strict(false);
        if let Some(result) = || -> Option<PegAlter> {
            let plain = self.peg_item()?;
            Some(PegAlter::Plain(plain))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[packrat::memoize(PegItem)]
    fn peg_item(&mut self) -> Option<PegItem> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<PegItem> {
            let atom = self.peg_atom()?;
            self.expect("+")?;
            Some(PegItem::OnceOrMore(atom))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            let atom = self.peg_atom()?;
            self.expect("*")?;
            Some(PegItem::ZeroOrMore(atom))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            let atom = self.peg_atom()?;
            self.expect("?")?;
            Some(PegItem::Optional(atom))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            self.expect("&")?;
            cut = true;
            let atom = self.peg_atom()?;
            Some(PegItem::PositiveLookahead(atom))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut {
            return None;
        }
        if let Some(result) = || -> Option<PegItem> {
            self.expect("!")?;
            cut = true;
            let atom = self.peg_atom()?;
            Some(PegItem::NegativeLookahead(atom))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut {
            return None;
        }
        if let Some(result) = || -> Option<PegItem> {
            self.expect("~")?;
            Some(PegItem::Cut)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            let atom = self.peg_atom()?;
            Some(PegItem::Plain(atom))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[packrat::memoize(PegAtom)]
    fn peg_atom(&mut self) -> Option<PegAtom> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<PegAtom> {
            self.expect("(")?;
            cut = true;
            let rule = self.peg_rule()?;
            self.expect(")")?;
            Some(PegAtom::Parentheses(Box::new(rule)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut {
            return None;
        }
        if let Some(result) = || -> Option<PegAtom> {
            let string = string::parse(self)?;
            Some(PegAtom::String(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegAtom> {
            let name = name::parse(self)?;
            Some(PegAtom::Name(name))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
