use crate::ast::*;
use crate::registry::*;
use daybreak::Parser;

impl Base for Parser<'_, CT, CR> {
    type CT = CT;
    type CR = CR;
}

impl Syntax for Parser<'_, CT, CR> {
    fn grammar(&mut self) -> Option<Gram> {
        let (res, cut) = self.alter(|x| {
            let first = x.non_terminal()?;
            let mut rules = vec![first];
            while let Some(rule) = x.non_terminal() {
                rules.push(rule)
            }
            if x.stream.next().is_some() {
                return None;
            }
            Some(rules)
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }
    
    fn non_terminal(&mut self) -> Option<NonT> {
        let (res, cut) = self.alter(|x| {
            let name = x.name()?;
            x.expect(":")?;
            x.expect("\\");
            let rule = x.rule()?;
            x.expect(";")?;
            Some(NonT { name, rule })
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    #[daybreak::lecursion(Rule)]
    fn rule(&mut self) -> Option<Rule> {
        let (res, cut) = self.alter(|x| {
            let mut left = x.rule()?;
            x.expect("\\")?;
            let right = x.alternative()?;
            left.push(right);
            Some(left)
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            let item = x.alternative()?;
            Some(vec![item])
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    #[daybreak::lecursion(Alter)]
    fn alternative(&mut self) -> Option<Alter> {
        let (res, cut) = self.alter(|x| {
            let mut left = x.alternative()?;
            x.stream.strict = true;
            x.expect(" ")?;
            x.stream.strict = false;
            let right = x.item()?;
            left.push(right);
            Some(left)
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            let item = x.item()?;
            Some(vec![item])
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    #[daybreak::memoize(Item)]
    fn item(&mut self) -> Option<Item> {
        let (res, cut) = self.alter(|x| {
            let atom = x.atom()?;
            x.expect("+")?;
            Some(Item::OnceOrMore(atom))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            let atom = x.atom()?;
            x.expect("*")?;
            Some(Item::ZeroOrMore(atom))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            let atom = x.atom()?;
            x.expect("?")?;
            Some(Item::Optional(atom))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("&")?;
            let atom = x.atom()?;
            Some(Item::PositiveLookahead(atom))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("!")?;
            let atom = x.atom()?;
            Some(Item::NegativeLookahead(atom))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("~")?;
            Some(Item::Cut)
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            let atom = x.atom()?;
            Some(Item::Plain(atom))
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    #[daybreak::memoize(Atom)]
    fn atom(&mut self) -> Option<Atom> {
        let (res, cut) = self.alter(|x| {
            x.expect("(")?;
            x.cut = true;
            let rule = x.rule()?;
            x.expect(")")?;
            Some(Atom::Parentheses(Box::new(rule)))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            let string = x.str()?;
            Some(Atom::String(string))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            let name = x.name()?;
            Some(Atom::Name(name))
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    #[daybreak::memoize(Name)]
    fn name(&mut self) -> Option<Name> {
        let (res, cut) = self.alter(|x| {
            x.stream.trim();
            x.stream.strict = true;
            let first = x.scan(|c| c.is_ascii_alphabetic())?;
            let mut name = String::from(first);
            while let Some(ch) = x.scan(|c| c.is_ascii_alphanumeric()) {
                name.push(ch)
            }
            Some(name)
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    #[daybreak::memoize(Str)]
    fn str(&mut self) -> Option<Str> {
        let (res, cut) = self.alter(|x| {
            x.stream.trim();
            x.stream.strict = true;
            x.expect("r")?;
            x.cut = true;
            x.expect("\"")?;
            let mut string = String::new();
            while let Some(ch) = x.scan(|c| c != '"') {
                string.push(ch)
            }
            x.expect("\"")?;
            Some(Str::Raw(string))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.stream.trim();
            x.stream.strict = true;
            x.expect("\"")?;
            let mut string = Vec::new();
            while x.lookahead(|c| c != '"').is_some() {
                let ch = x.char()?;
                string.push(ch)
            }
            x.expect("\"")?;
            Some(Str::Plain(string))
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }

    fn char(&mut self) -> Option<Char> {
        let (res, cut) = self.alter(|x| {
            let ch = x.scan(|c| !matches!(c, '\\' | '\n' | '\t' | '\r' | '"'))?;
            Some(Char::Plain(ch))
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("\\\\")?;
            Some(Char::Backlash)
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("\\\"")?;
            Some(Char::Quotation)
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("\\n")?;
            Some(Char::Newline)
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("\\r")?;
            Some(Char::Return)
        });
        if cut || res.is_some() {
            return res;
        }
        let (res, cut) = self.alter(|x| {
            x.expect("\\t")?;
            Some(Char::Tab)
        });
        if cut || res.is_some() {
            return res;
        }
        None
    }
}