# The Base Case

This file is describe to the parser in the ./src folder. This parse is able to parse the [metagrammar.gram](https://github.com/python/cpython/blob/main/Tools/peg_generator/pegen/metagrammar.gram) with some small changes: [pegen.gram](pegen.gram), but it cannot parse itself. The intuition is to use this mini peg parser to generate a fully functional peg parser. However, the [pegen.gram](pegen.gram) will not be the meta grammar for this project. I will build something similar but more general.

```
grammar[Grammar]:
    | rule (NEWLINE rule)* EOF { Grammar { rules } }

rule[Rule]:
    | NAME RSTYPE ": " alter NEWLINE {
        Rule { name, rstype, alters: vec![alter] }
    }
    | NAME RSTYPE ":" NEWLINE ("    | " alter NEWLINE)+ {
        Rule { name, rstype, alters }
    }

alter[Alter]:
    | named (" " named)* " " INLINE { Alter { items, inline } }

named[Named]:
    | NAME "=" ~ item { Named::Identifier(name, item) }
    | item { Named::Anonymous(item) }
    | "~" { Named::Cut }

item[Item]:
    | atom "?" { Item::Optional(atom) }
    | atom { Item::Exact(atom) }

atom[Atom]:
    | STRING { Atom::String(string) }
    | NAME { Atom::Name(name) }
```

By removing the metadata, `!` syntax, and extra whitespaces from the metagrammar.gram, you will get [pegen.gram](pegen.gram) which is parsable by this base case parser.
