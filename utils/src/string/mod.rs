use crate::string::ast::PegString;
use crate::string::register::Text;
use engine::Parser;
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod register;
mod ast;
mod parser;

pub fn parse<CT, CR>(other: &mut Parser<CT, CR>) -> Option<PegString>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    let mut parser = other.export();
    let result = parser.peg_string()?;
    let mark = parser.stream.mark();
    other.stream.jump(mark);
    Some(result)
}

#[test]
fn test() {
    let mut x = Parser::<i32, i32>::new(r#"" d\td""#);
    parse(&mut x);
}
