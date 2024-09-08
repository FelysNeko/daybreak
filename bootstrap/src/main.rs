mod register;
mod parser;
mod ast;

use pegcore::*;
use register::*;

fn main() {
    let code = r#"Foo: "hello" \ "world""#.to_string();
    let v =Verbose::Full;
    Parser::<CacheType, CacheResult>::new(code, v).peg_atom();
}
