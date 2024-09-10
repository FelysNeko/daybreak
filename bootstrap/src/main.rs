use crate::register::cache::{CacheResult, CacheType};
use crate::register::method::Bootstrap;
use engine::Parser;

mod parser;
mod register;
mod ast;

fn main() {
    let code = r#"  
    grammar: identified+;
    
    identified: name ":" "\\"? rule r"\n";
    
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
    let mut x = Parser::<CacheType, CacheResult>::new(code);
    if let Some(grammar) = x.peg_grammar() {
        println!();
        println!("{}", grammar);
    } else {
        println!("parsing failed")
    }
}
