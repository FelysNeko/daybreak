use pegcore::*;

#[indicator]
pub enum CacheType {
    Expect(&'static char)
}

#[output]
pub enum CacheResult {
    Expect(Option<&'static char>),
}
