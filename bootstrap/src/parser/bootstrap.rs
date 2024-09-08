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

    #[memoize(PegUnnamedRule)]
    fn peg_unnamed_rule(&mut self) -> Option<PegUnnamedRule> {
        todo!()
    }

    #[memoize(PegAlter)]
    fn peg_alter(&mut self) -> Option<PegAlter> {
        todo!()
    }

    #[memoize(PegItem)]
    fn peg_item(&mut self) -> Option<PegItem> {
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
            Some(PegAtom::Name(string))
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
            let first = self.scan(|c| c.is_ascii_alphabetic())?;
            let mut string = String::from(first);
            while let Some(ch) = self.scan(|c| c.is_ascii_alphanumeric()) {
                string.push(ch)
            }
            Some(string)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
