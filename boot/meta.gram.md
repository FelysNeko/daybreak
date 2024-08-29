# The Base Case

This file describes the parser in the ./src folder. The intuition is to use this mini peg parser to generate a fully functional peg parser. Note that this parser does not parse itself, and this grammar file is not actuate.

```
grammar[Grammar]:
    | insert NEWLINE rule (NEWLINE rule)* EOF { Grammar { rules } }

insert[Insert]:
    | "QUOTATION" "QUOTATION" STRING "QUOTATION" "QUOTATION" NEWLINE { Insert { inline } }

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
    | NAME "=" ~ atom { Named::Identifier(name, atom) }
    | atom { Named::Anonymous(atom) }
    | "~" { Named::Cut }

atom[Atom]:
    | STRING { Atom::String(string) }
    | NAME { Atom::Name(name) }
```

The cpython [metagrammar.gram](https://github.com/python/cpython/blob/main/Tools/peg_generator/pegen/metagrammar.gram) is used as an example. By removing the metadata, `!` and `?` syntax, and extra whitespaces from it, you will get the following which is parsable by this mini parser.

```
start[Grammar]: grammar ENDMARKER { grammar }

grammar[Grammar]:
    | metas rules { Grammar(rules, metas) }
    | rules { Grammar(rules, []) }

metas[MetaList]:
    | meta metas { [meta] + metas }
    | meta { [meta] }

meta[MetaTuple]:
    | "@" NAME NEWLINE { (name.string, None) }
    | "@" a=NAME b=NAME NEWLINE { (a.string, b.string) }
    | "@" NAME STRING NEWLINE { (name.string, literal_eval(string.string)) }

rules[RuleList]:
    | rule rules { [rule] + rules }
    | rule { [rule] }

rule[Rule]:
    | rulename ":" alts NEWLINE INDENT more_alts DEDENT { Rule(rulename[0], rulename[1], Rhs(alts.alts + more_alts.alts), memo=opt) }
    | rulename ":" NEWLINE INDENT more_alts DEDENT { Rule(rulename[0], rulename[1], more_alts, memo=opt) }
    | rulename ":" alts NEWLINE { Rule(rulename[0], rulename[1], alts, memo=opt) }

rulename[RuleName]:
    | NAME annotation { (name.string, annotation) }
    | NAME { (name.string, None) }

memoflag[str]:
    | "(" "memo" ")" { "memo" }

alts[Rhs]:
    | alt "|" alts { Rhs([alt] + alts.alts)}
    | alt { Rhs([alt]) }

more_alts[Rhs]:
    | "|" alts NEWLINE more_alts { Rhs(alts.alts + more_alts.alts) }
    | "|" alts NEWLINE { Rhs(alts.alts) }

alt[Alt]:
    | items "$" action { Alt(items + [NamedItem(None, NameLeaf("ENDMARKER"))], action=action) }
    | items "$" { Alt(items + [NamedItem(None, NameLeaf("ENDMARKER"))], action=None) }
    | items action { Alt(items, action=action) }
    | items { Alt(items, action=None) }

items[NamedItemList]:
    | named_item items { [named_item] + items }
    | named_item { [named_item] }

named_item[NamedItem]:
    | NAME annotation "=" ~ item {NamedItem(name.string, item, annotation)}
    | NAME "=" ~ item {NamedItem(name.string, item)}
    | item {NamedItem(None, item)}
    | forced=forced_atom {NamedItem(None, forced)}
    | it=lookahead {NamedItem(None, it)}

forced_atom[Forced]:
    | "&''&" ~ atom {Forced(atom)}

lookahead[LookaheadOrCut]:
    | "&" ~ atom {PositiveLookahead(atom)}
    | "!" ~ atom {NegativeLookahead(atom)}
    | "~" {Cut()}

item[Item]:
    | "[" ~ alts "]" {Opt(alts)}
    | atom "?" {Opt(atom)}
    | atom "*" {Repeat0(atom)}
    | atom "+" {Repeat1(atom)}
    | sep=atom "." node=atom "+" {Gather(sep, node)}
    | atom {atom}

atom[Plain]:
    | "(" ~ alts ")" {Group(alts)}
    | NAME {NameLeaf(name.string) }
    | STRING {StringLeaf(string.string)}

action[str]: "{" ~ target_atoms "}" { target_atoms }

annotation[str]: "[" ~ target_atoms "]" { target_atoms }

target_atoms[str]:
    | target_atom target_atoms { target_atom + " " + target_atoms }
    | target_atom { target_atom }

target_atom[str]:
    | "{" ~ atoms=target_atoms? "}" { "{" + (atoms or "") + "}" }
    | "[" ~ atoms=target_atoms? "]" { "[" + (atoms or "") + "]" }
    | NAME "*" { name.string + "*" }
    | NAME { name.string }
    | NUMBER { number.string }
    | STRING { string.string }
    | "?" { "?" }
    | ":" { ":" }
```
