use crate::string::ast::{PegChar, PegString};

#[daybreak::ct]
pub enum CacheType {
    PegString,
    PegChar,
}

#[daybreak::cr]
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
