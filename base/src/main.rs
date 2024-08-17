mod parser;

use std::fs::read_to_string;
use parser::*;

fn main() {
    let source = read_to_string("example.gram").unwrap();
    let mut peg = Parser::new(source.as_str());
    println!("{:?}", peg.parse());
}

