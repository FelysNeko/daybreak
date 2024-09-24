use crate::registry::{Syntax, CR, CT};
use daybreak::Parser;

mod ast;
mod registry;
mod parser;

const CODE: &str = r#"
grammar: nont+;

nont: name ":" "\\"? rule r"\n";

rule: 
    \ rule "\\" alter
    \ alter
    ;
    
alter: 
    \ alter " " item
    \ item
    ;
    
item:
    \ atom "+"
    \ atom "*"
    \ atom "?"
    \ "&" ~ atom
    \ "!" ~ atom
    \ "~"
    \ atom
    ;
    
atom:
    \ "(" ~ rule ")"
    \ STRING
    \ NAME
    ;
"#;

fn main() {
    let mut parser = Parser::<CT, CR>::new(CODE);
    let result = parser.grammar();
    println!("{:?}", result)
}
