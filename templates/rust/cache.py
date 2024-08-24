from templates.shared import CLAIM, Generator

class Cache(Generator):
    __body_import = '''
use crate::node::*;
use colored::Colorize;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
'''
    __body_macro = '''
#[macro_export]
macro_rules! memoize {
    ($self:ident, $ct:expr, $cr1:ident::$cr2:ident, $t:ty, $func:block) => {
        {
            let origin = $self.stream.cursor;
            let ct = $ct;

            if let Some((result, end)) = $self.cache.get(origin, ct) {
                $self.stream.cursor = end;
                return result.into();
            }

            let result = || -> Option<$t> {$func}();

            let cr = $cr1::$cr2(result.clone());
            $self.cache.insert(origin, ct, cr, $self.stream.cursor);
            result
        }
    };
}
'''
    __body_cache = '''
pub struct Cache {
    pub body: HashMap<(usize, CacheType), (CacheResult, usize)>,
    pub verbose: bool,
    pub hit: usize,
}

impl Cache {
    pub fn get(&mut self, pos: usize, ct: CacheType) -> Option<(CacheResult, usize)> {
        if let Some((res, end)) = self.body.get(&(pos, ct)) {
            if self.verbose {
                let log = format!("{}\\t{}\\t{:?} => {:?}", pos, end, ct, res);
                println!("{}", log.truecolor(0xff, 0xc6, 0xf4));
            }
            self.hit += 1;
            Some((res.clone(), end.to_owned()))
        } else {
            None
        }
    }

    pub fn insert(&mut self, pos: usize, ct: CacheType, res: CacheResult, end: usize) {
        if self.verbose {
            println!("{}\\t{}\\t{:?} => {:?}", pos, end, ct, res);
        }
        if self.body.insert((pos, ct), (res, end)).is_some() {
            panic!("cache conflicted")
        }
    }
}
'''

    def __init__(self, peg) -> None:
        super().__init__(peg)

    def generate(self) -> None:
        self.print(CLAIM)
        self.print(self.__body_import)
        self.print(self.__body_macro)
        self.print(self.__body_cache)

        self.print()
        self.print('#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]')
        self.print('pub enum CacheType {')
        with self.indent():
            self.print('Expect(&\'static str),')
            self.print('String,')
            self.print('Inline,')
            self.print('Name,')
            for each in self.node:
                self.print(f'{each},')
        self.print('}')
        self.print()

        self.print()
        self.print('#[derive(Clone)]')
        self.print('pub enum CacheResult {')
        with self.indent():
            self.print('Expect(Option<()>)')
            self.print('String(Option<String>)')
            self.print('Inline(Option<String>)')
            self.print('Name(Option<String>)')
            for each in self.node:
                self.print(f'{each}(Option<String>)')
        self.print('}')
        self.print()

        self.print()
        self.print('impl Debug for CacheResult {')
        with self.indent():
            self.print('fn fmt(&self, f: &mut Formatter<\'_>) -> std::fmt::Result {')
            with self.indent():
                self.print('match self {')
                with self.indent():
                    self.print('CacheResult::Expect(r) => write!(f, "{{:?}}", r),')
                    self.print('CacheResult::String(r) => write!(f, "{{:?}}", r),')
                    self.print('CacheResult::Inline(r) => write!(f, "{{:?}}", r),')
                    self.print('CacheResult::Name(r) => write!(f, "{{:?}}", r),')
                    for each in self.node:
                        self.print(f'CacheResult::{each}(r) => write!(f, "{{:?}}", r),')
                self.print('}')
            self.print('}')
        self.print('}')
        self.print()
