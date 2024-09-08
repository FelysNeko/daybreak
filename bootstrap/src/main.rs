mod register;
mod parser;
mod ast;

use pegcore::*;
use register::*;

fn main() {
    let code = r#""f" f   vv"#.to_string();
    let v =Verbose::Full;
    Parser::<CacheType, CacheResult>::new(code, v).peg_alter();
}
