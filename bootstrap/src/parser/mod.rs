use crate::register::*;
use pegcore::Parser;

mod wrapper;
mod bootstrap;

impl Base for Parser<CacheType, CacheResult> {
    type CT = CacheType;
    type CR = CacheResult;
}
