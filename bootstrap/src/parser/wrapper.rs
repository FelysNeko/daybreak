use crate::register::*;
use pegcore::*;

impl Wrapper for Parser<CacheType, CacheResult> {
    #[memoize(PegExpect)]
    fn peg_expect(&mut self, s: &'static str) -> Option<&'static str> {
        self.expect(s)
    }
}
