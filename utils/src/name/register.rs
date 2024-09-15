use crate::name::ast::PegName;

#[packrat::ct]
pub enum CacheType {
    PegName,
}

#[packrat::cr]
pub enum CacheResult {
    PegName(Option<PegName>)
}

pub trait Base {
    type CT;
    type CR;
}

pub trait Name: Base {
    fn peg_name(&mut self) -> Option<PegName>;
}
