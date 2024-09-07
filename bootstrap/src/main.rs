mod register;
mod ast;

use pegcore::*;
use register::*;

fn main() {
    Parser::<CacheType, CacheResult>::new("Foo".to_string(), Verbose::Core);
}

