mod register;
mod parser;
mod ast;

use pegcore::*;
use register::*;

fn main() {
    let code = "Foo".to_string();
    let v =Verbose::Core;
    let mut peg = Parser::<CacheType, CacheResult>::new(code, v);
    todo!()
}
