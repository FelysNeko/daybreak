mod atom;
mod gram;
mod alter;
mod rule;
mod named;

pub use alter::Alter;
pub use atom::Atom;
pub use gram::Grammar;
pub use named::Named;
pub use rule::Rule;


pub trait Generate {
    fn generate(&self) -> String;
}

pub fn indent(n: usize) -> String {
    "    ".repeat(n)
}
