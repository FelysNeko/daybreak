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
    __body_cachetype = '''
#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum CacheType {{
    Expect(&'static str),
    String,
    Inline,
    Name,
{cachetype}
}}
'''
    __cachetype_template = '    {node},'
    __body_cacheresult = '''
#[derive(Clone)]
pub enum CacheResult {{
    Expect(Option<()>),
    String(Option<String>),
    Inline(Option<String>),
    Name(Option<String>),
{cacheresult}
}}
'''
    __cacheresult_template = '    {node}(Option<{node}>),'
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
    __body_debug_cachereuslt = '''
impl Debug for CacheResult {{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
        match self {{
            CacheResult::Expect(r) => write!(f, "{{:?}}", r),
            CacheResult::String(r) => write!(f, "{{:?}}", r),
            CacheResult::Inline(r) => write!(f, {{:?}}", r),
            CacheResult::Name(r) => write!(f, "{{:?}}", r),
{debug}
        }}
    }}
}}
'''
    __debug_cachereuslt_template = ' '*12 + 'CacheResult::{node}(r) => write!(f, "{{:?}}", r),'

    def __init__(self, peg) -> None:
        super().__init__(peg)

    def generate(self) -> None:
        self.print(CLAIM)
        self.print(self.__body_import)
        self.print(self.__body_macro)
        cachetype = '\n'\
            .join(self.__cachetype_template.format(node=each) for each in self.node)
        self.print(self.__body_cachetype.format(cachetype=cachetype))
        cacheresult = '\n'\
            .join(self.__cacheresult_template.format(node=each) for each in self.node)
        self.print(self.__body_cacheresult.format(cacheresult=cacheresult))
        self.print(self.__body_cache)
        debug = '\n'\
            .join(self.__debug_cachereuslt_template.format(node=each) for each in self.node)
        self.print(self.__body_debug_cachereuslt.format(debug=debug))
