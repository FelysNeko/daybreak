use crate::name::ast::PegName;
use crate::name::register::Name;
use crate::name::register::{Base, CacheResult, CacheType};
use engine::Parser;

impl Base for Parser<'_, CacheType, CacheResult> {
    type CT = CacheType;
    type CR = CacheResult;
}

impl Name for Parser<'_, CacheType, CacheResult> {
    #[packrat::strict]
    #[packrat::memoize(PegName)]
    fn peg_name(&mut self) -> Option<PegName> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<PegName> {
            let first = self.scan(|c| c.is_ascii_alphabetic())?;
            self.stream.strict(true);
            let mut string = String::from(first);
            while let Some(ch) = self.scan(|c| c.is_ascii_alphanumeric()) {
                string.push(ch)
            }
            Some(PegName { name: string })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
