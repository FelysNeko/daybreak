pub use crate::name::ast::PegName;
use crate::name::register::Name;
use engine::Parser;
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod register;
mod ast;
mod parser;

pub fn parse<CT, CR>(other: &mut Parser<CT, CR>) -> Option<PegName>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    let mut parser = other.export();
    let result = parser.peg_name()?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}

#[test]
fn test() {
    let cases = [
        ("elysia", "elysia"),
    ];

    for (case, expect) in cases {
        let mut x = Parser::<i32, i32>::new(case);
        let result = parse(&mut x).unwrap().to_string();
        assert_eq!(expect, result.as_str());
        assert_eq!(x.stream.mark(), case.len())
    }
}
