pub type Gram = Vec<NonT>;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct NonT {
    pub name: Name,
    pub rule: Rule,
}

pub type Rule = Vec<Alter>;

pub type Alter = Vec<Item>;

#[derive(Clone, Debug)]
pub enum Item {
    OnceOrMore(Atom),
    ZeroOrMore(Atom),
    Optional(Atom),
    PositiveLookahead(Atom),
    NegativeLookahead(Atom),
    Plain(Atom),
    Cut,
}

#[derive(Clone, Debug)]
pub enum Atom {
    Parentheses(Box<Rule>),
    String(Str),
    Name(Name),
}

pub type Name = String;

#[derive(Clone, Debug)]
pub enum Str {
    Raw(String),
    Plain(Vec<Char>),
}

#[derive(Clone, Debug)]
pub enum Char {
    Plain(char),
    Backlash,
    Quotation,
    Newline,
    Return,
    Tab,
}
