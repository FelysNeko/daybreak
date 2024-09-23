use crate::ast::*;
use crate::registry::*;
use daybreak::Parser;

impl Base for Parser<'_, CT, CR> {
    type CT = CT;
    type CR = CR;
}

impl Syntax for Parser<'_, CT, CR> {
    #[daybreak::memoize(Gram)]
    fn grammar(&mut self) -> Option<Gram> {
        todo!()
    }

    #[daybreak::memoize(NonT)]
    fn non_terminal(&mut self) -> Option<NonT> {
        todo!()
    }

    #[daybreak::memoize(Rule)]
    fn rule(&mut self) -> Option<Rule> {
        todo!()
    }

    #[daybreak::memoize(Alter)]
    fn alter(&mut self) -> Option<Alter> {
        todo!()
    }

    #[daybreak::memoize(Item)]
    fn item(&mut self) -> Option<Item> {
        todo!()
    }

    #[daybreak::memoize(Atom)]
    fn atom(&mut self) -> Option<Atom> {
        todo!()
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