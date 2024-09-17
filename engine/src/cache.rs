use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// A wrapper over HashMap to cache parsing results.
pub struct Cache<CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    pub(crate) body: HashMap<(usize, bool, CT), (usize, CR)>,
    pub(crate) verbose: Verbose,
    pub(crate) hit: usize,
}

impl<CT, CR> Cache<CT, CR>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    /// Get the logging verboseness
    pub fn verbose(&self) -> Verbose {
        self.verbose
    }
    
    /// Try to get the cache result.
    pub fn get(&mut self, pos: usize, s: bool, ct: CT) -> Option<(usize, CR)> {
        let cache = self.body.get(&(pos, s, ct));
        if let Some((end, cr)) = cache {
            if self.verbose >= Verbose::Hit {
                println!("> hit\t\t{:<11} {:<11} {:<23} {:<11} {}", pos, s, ct.to_string(), end, cr)
            }
            self.hit += 1;
            cache.cloned()
        } else {
            if self.verbose >= Verbose::Miss {
                println!("> miss\t\t{:<11} {:<11} {:<23}", pos, s, ct.to_string())
            }
            None
        }
    }

    /// Insert or update the cache result.
    pub fn insert(&mut self, pos: usize, s: bool, ct: CT, end: usize, cr: CR) {
        let crs = cr.to_string();
        if let Some(cache) = self.body.insert((pos, s, ct), (end, cr)) {
            let (end, cr) = cache;
            if self.verbose >= Verbose::Drop {
                println!("> drop\t\t{:<11} {:<11} {:<23} {:<11} {}", pos, s, ct.to_string(), end, cr)
            }
        }
        if self.verbose >= Verbose::Cache {
            println!("> cache\t\t{:<11} {:<11} {:<23} {:<11} {}", pos, s, ct.to_string(), end, crs)
        }
    }
}

/// Cache log verboseness.
#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum Verbose {
    Off,
    Cache,
    Drop,
    Hit,
    Miss,
}
