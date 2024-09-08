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
    \ "(" ~ rule ")"
    \ item "+"
    \ item "*"
    \ item "?"
    \ "&" ~ item
    \ "!" ~ item
    \ "~"
    \ atomic
    
atomic:
    \ string
    \ name
    
string: "\"" CHAR* "\""

name: CHAR CHAR*
```
