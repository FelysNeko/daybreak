use std::fmt::{Display, Formatter};

#[daybreak::ast]
pub enum PegString {
    Raw(String),
    Plain(Vec<PegChar>),
}

impl Display for PegString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegString::Raw(x) => write!(f, "r\"{}\"", x),
            PegString::Plain(vx) => {
                let s = vx.iter()
                    .map(|x| { x.to_string() })
                    .collect::<Vec<String>>()
                    .join("");
                write!(f, "\"{}\"", s)
            }
        }
    }
}

#[daybreak::ast]
pub enum PegChar {
    Plain(char),
    Backlash,
    Quotation,
    Newline,
    Return,
    Tab,
}

impl Display for PegChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PegChar::Plain(x) => write!(f, "{}", x),
            PegChar::Backlash => write!(f, "\\"),
            PegChar::Quotation => write!(f, "\""),
            PegChar::Newline => writeln!(f),
            PegChar::Return => write!(f, "\r"),
            PegChar::Tab => write!(f, "\t"),
        }
    }
}
