pub trait Generate {
    fn generate(&self) -> String;
}

pub struct Grammar {
    rules: Vec<Rule>,
}

impl Generate for Grammar {
    fn generate(&self) -> String {
        todo!()
    }
}

pub struct Rule {
    name: String,
    native: String,
    alters: Vec<Alter>,
}

impl Generate for Rule {
    fn generate(&self) -> String {
        todo!()
    }
}

pub struct Alter {
    items: Vec<Item>,
    inline: String,
}

impl Generate for Alter {
    fn generate(&self) -> String {
        todo!()
    }
}

pub enum Item {
    Optional(Atom),
    LoopZero(Atom),
    LoopOnce(Atom),
    Exact(Atom),
}

impl Generate for Item {
    fn generate(&self) -> String {
        match self {
            Item::Optional(_) => todo!(),
            Item::LoopZero(_) => todo!(),
            Item::LoopOnce(_) => todo!(),
            Item::Exact(_) => todo!(),
        }
    }
}

pub enum Atom {
    String(String),
    Name(String),
}

impl Generate for Atom {
    fn generate(&self) -> String {
        match self {
            Atom::String(_) => todo!(),
            Atom::Name(_) => todo!()
        }
    }
}
