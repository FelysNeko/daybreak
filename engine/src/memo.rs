use std::collections::HashMap;
use std::hash::Hash;

pub struct Memo<CT: Eq + Hash, CR: Clone> {
    pub(crate) body: HashMap<(usize, bool, CT), (usize, CR)>,
}

impl<CT: Eq + Hash, CR: Clone> Memo<CT, CR> {
    pub fn get(&self, p: usize, s: bool, ct: CT) -> Option<(usize, CR)> {
        self.body.get(&(p, s, ct)).cloned()
    }

    pub fn insert(&mut self, p: usize, s: bool, ct: CT, e: usize, cr: CR) {
        self.body.insert((p, s, ct), (e, cr));
    }
}
