use pegcore::*;

fn main() {
    Parser::<CacheType, CacheResult>::new("foo".to_string(), Verbose::Core);
}

#[indicator]
enum CacheType {
    Expect(&'static char)
}

#[output]
enum CacheResult {
    Expect(Option<StaticChar>),
}

type StaticChar = &'static char;
