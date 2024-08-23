use crate::node::{Alter, Atom, Grammar, Named, Rule};

pub struct Visitor {
    pub indent: usize,
    pub output: Vec<String>,
}

macro_rules! indent {
    ($self:ident, $inside:block) => {
        $self.indent += 1;
        $inside
        $self.indent -= 1;
    };
}

macro_rules! lp {
    ($self:ident, $($arg:tt)*) => {
        $self.output.push(format!("{}{}", "    ".repeat($self.indent), format!($($arg)*)))
    };
}

macro_rules! p {
    ($self:ident, $($arg:tt)*) => {
        if let Some(last) = $self.output.last_mut() {
            last.push_str(format!($($arg)*).as_str())
        } else {
            lp!($self, $($arg)*)
        }
    };
}

impl Visitor {
    pub fn generate(grammar: Grammar) -> String {
        Self {
            indent: 0,
            output: Vec::new(),
        }.grammar(grammar)
    }
    
    pub fn grammar(&mut self, grammar: Grammar) -> String {
        for each in grammar.rules {
            self.rule(each)
        }
        self.output.join("\n")
    }

    pub fn rule(&mut self, rule: Rule) {
        lp!(self, "pub fn {}(&mut self) -> Option<{}> {{", rule.name, rule.rstype);
        indent!(self, {
            lp!(self, "let origin = self.stream.cursor;");
            lp!(self, "memoize!(self, CacheType::{}, CacheResult::{}, {}, {{", rule.rstype, rule.rstype, rule.rstype);
            indent!(self, {
                lp!(self, "let mut cut = false;");
                for each in rule.alters {
                    self.alter(each, rule.rstype.clone())
                }
                lp!(self, "None");
            });
            lp!(self, "}})");
        });
        lp!(self, "}}");
    }

    pub fn alter(&mut self, alter: Alter, rstype: String) {
        lp!(self, "if let Some(result) = || -> Option<{}> {{", rstype);
        indent!(self, {
            for each in alter.nameds {
                self.nameds(each)
            }
            lp!(self, "Some({})", alter.inline.trim())
        });
        lp!(self, "}}() {{");
        indent!(self, {
            lp!(self, "return Some(result)");
        });
        lp!(self, "}} else {{");
        indent!(self, {
            lp!(self, "self.stream.cursor = origin");
        });
        lp!(self, "}}");
        lp!(self, "if cut {{ return None }}");
    }

    pub fn nameds(&mut self, named: Named) {
        match named {
            Named::Identifier(n, a) => {
                lp!(self, "let {} = ", n);
                self.atom(a);
                p!(self, ";");
            }
            Named::Anonymous(a) => {
                lp!(self, "");
                self.atom(a);
                p!(self, ";");
            }
            Named::Cut => lp!(self, "cut = true;")
        }
    }

    pub fn atom(&mut self, atom: Atom) {
        match atom {
            Atom::String(s) => p!(self, "self.expect(\"{}\")", s),
            Atom::Name(s) => match s.as_str() {
                "INLINE" |
                "STRING" |
                "NAME" => p!(self, "self.{}()", s.to_lowercase()),
                _ => p!(self, "self.{}()", s)
            },
        }
    }
}
