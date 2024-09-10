use crate::register::cache::{CacheResult, CacheType};
use crate::register::method::Base;
use engine::Parser;

pub mod bootstrap;

impl Base for Parser<'_, CacheType, CacheResult> {
    type CT = CacheType;
    type CR = CacheResult;
}
