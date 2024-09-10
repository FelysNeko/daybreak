# PEG Grammar File Parser

```
grammar: identified+

identified: name ":" "\"? rule

rule: 
    \ rule "\" alter
    \ alter
    
alter: 
    \ alter " " item
    \ item
    
item:
    \ atom "+"
    \ atom "*"
    \ atom "?"
    \ "&" ~ atom
    \ "!" ~ atom
    \ "~"
    \ atom
    
atom:
    \ "(" ~ rule ")"
    \ STRING
    \ NAME
```
