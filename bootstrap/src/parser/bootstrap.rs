use crate::ast::node::*;
use crate::register::*;
use pegcore::*;

impl Bootstrap for Parser<CacheType, CacheResult> {
    #[memoize(PegGrammar)]
    fn peg_grammar(&mut self) -> Option<PegGrammar> {
        todo!()
    }

    #[memoize(PegRule)]
    fn peg_rule(&mut self) -> Option<PegRule> {
        todo!()
    }

    #[lecursion(PegAlter)]
    fn peg_alter(&mut self) -> Option<PegAlter> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegAlter> {
            let prior = self.peg_alter()?;
            self.lookahead(|c| c.is_whitespace())?;
            let lower = self.peg_item()?;
            Some(PegAlter::Rec { prior: Box::new(prior), lower })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegAlter> {
            let lower = self.peg_item()?;
            Some(PegAlter::Plain(lower))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(PegItem)]
    fn peg_item(&mut self) -> Option<PegItem> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<PegItem> {
            self.peg_expect("(")?;
            cut = true;
            let rule = self.peg_unnamed_rule()?;
            self.peg_expect(")")?;
            Some(PegItem::Nested(rule))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<PegItem> {
            self.peg_expect("&")?;
            cut = true;
            let inner = self.peg_item()?;
            Some(PegItem::PositiveLookahead(Box::new(inner)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<PegItem> {
            self.peg_expect("!")?;
            cut = true;
            let inner = self.peg_item()?;
            Some(PegItem::NegativeLookahead(Box::new(inner)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<PegItem> {
            let inner = self.peg_item()?;
            self.expect("*")?;
            Some(PegItem::NoneOrMore(Box::new(inner)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            let inner = self.peg_item()?;
            self.expect("+")?;
            Some(PegItem::OnceOrMore(Box::new(inner)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            let inner = self.peg_item()?;
            self.expect("?")?;
            Some(PegItem::Optional(Box::new(inner)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            let atom = self.peg_atom()?;
            Some(PegItem::Atomic(atom))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegItem> {
            self.expect("~")?;
            Some(PegItem::Cut)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(PegUnnamedRule)]
    fn peg_unnamed_rule(&mut self) -> Option<PegUnnamedRule> {
        todo!()
    }

    #[memoize(PegAtom)]
    fn peg_atom(&mut self) -> Option<PegAtom> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegAtom> {
            let name = self.peg_name()?;
            Some(PegAtom::Name(name))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<PegAtom> {
            let string = self.peg_string()?;
            Some(PegAtom::String(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(PegString)]
    fn peg_string(&mut self) -> Option<PegString> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegString> {
            self.peg_expect("\"")?;
            self.stream.raw(true);
            let mut string = String::new();
            while self.lookahead(|c| c != '"').is_some() {
                let ch = self.stream.next()?;
                string.push(ch)
            }
            self.peg_expect("\"")?;
            Some(string)
        }() {
            self.stream.raw(false);
            return Some(result);
        } else {
            self.stream.raw(false);
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(PegName)]
    fn peg_name(&mut self) -> Option<PegName> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegName> {
            let first = self.scan(|c| c.is_ascii_alphabetic() || c == '_')?;
            self.stream.raw(true);
            let mut string = String::from(first);
            while let Some(ch) = self.scan(|c| c.is_ascii_alphanumeric() || c == '_') {
                string.push(ch)
            }
            Some(string)
        }() {
            self.stream.raw(false);
            return Some(result);
        } else {
            self.stream.raw(false);
            self.stream.jump(pos)
        }
        None
    }
}
