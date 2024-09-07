use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub struct Cache<CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub(crate) body: HashMap<(usize, CT), (usize, CR)>,
    pub(crate) verbose: Verbose,
    pub(crate) hit: usize,
}

impl<CT, CR> Cache<CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub fn get(&mut self, pos: usize, ct: CT) -> Option<(usize, CR)> {
        let cache = self.body.get(&(pos, ct));
        if let Some((end, cr)) = cache {
            if self.verbose >= Verbose::Core {
                println!("> hit\t\t{:<11} {:<23} {:<11} {}", pos, ct, end, cr)
            }
            self.hit += 1;
            cache.cloned()
        } else {
            if self.verbose >= Verbose::Full {
                println!("> miss\t\t{:<11} {:<23}", pos, ct)
            }
            None
        }
    }

    pub fn insert(&mut self, pos: usize, ct: CT, end: usize, cr: CR) {
        if self.verbose >= Verbose::Core {
            println!("> cache\t\t{:<11} {:<23} {:<11} {}", pos, ct, end, cr)
        }
        if let Some(cache) = self.body.insert((pos, ct), (end, cr)) {
            let (end, cr) = cache;
            if self.verbose >= Verbose::Core {
                println!("> drop\t\t{:<11} {:<23} {:<11} {}", pos, ct, end, cr)
            }
        }
    }
}

#[derive(PartialOrd, PartialEq)]
pub enum Verbose {
    None,
    Core,
    Full,
}
