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
        todo!()
    }

    #[memoize(PegString)]
    fn peg_string(&mut self) -> Option<PegString> {
        todo!()
    }

    #[memoize(PegName)]
    fn peg_name(&mut self) -> Option<PegName> {
        todo!()
    }
}
