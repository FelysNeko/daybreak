use crate::string::ast::{PegChar, PegString};

#[packrat::index]
pub enum CacheType {
    PegString,
    PegChar,
}

#[packrat::output]
pub enum CacheResult {
    PegString(Option<PegString>),
    PegChar(Option<PegChar>),
}

pub trait Base {
    type CT;
    type CR;
}


pub trait Text: Base {
    fn peg_string(&mut self) -> Option<PegString>;
    fn peg_char(&mut self) -> Option<PegChar>;
}
