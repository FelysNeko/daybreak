import binding

path = 'rspegen.gram'
header = f'// generated from {path} by generate.py\n'
with open(path) as rspegen:
    content = rspegen.read()

try:
    peg = binding.parse(content, False)
except Exception as error:
    try:
        binding.parse(content, True)
    finally:
        print(f'\033[91m{error}\033[0m')
        exit(1)
    
import json
tree = json.loads(peg.json)
print(tree)

# generate node.rs
node_template = '''
#[derive(Clone)]
pub struct {nt} {{

}}

impl Debug for {nt} {{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{
        todo!()
    }}
}}

impl From<CacheResult> for Option<{nt}> {{
    fn from(value: CacheResult) -> Self {{
        match value {{
            CacheResult::{nt}(inner) => inner,
            _ => panic!("cache not matched")
        }}
    }}
}}
'''
fmt_node_list = [node_template.format(nt=each) for each in peg.node]
node_body = '\n'.join(fmt_node_list)
node = header + '''
use crate::cache::CacheResult;
use std::fmt::{{Debug, Formatter}};
''' + node_body


# generate visitor.rs
visitor_template = '''
    fn {nt_lower}(&mut self, {nt_lower}: {nt}) {{
        todo!()
    }}
'''
fmt_visitor_list = [visitor_template.format(nt_lower=each.lower(), nt=each) for each in peg.node]
visitor_body = '\n'.join(fmt_visitor_list)
visitor = header + '''
use crate::node::{''' + ', '.join(peg.node) + '''};

pub struct Visitor {
    pub indent: usize,
    pub output: Vec<String>,
}

macro_rules! indent {
    ($self:ident, $inside:block) => {
        $self.indent += 1;
        $inside
        $self.indent -= 1;
    };
}

macro_rules! lp {
    ($self:ident, $($arg:tt)*) => {
        $self.output.push(format!("{{}}{{}}", "    ".repeat($self.indent), format!($($arg)*)))
    };
}

macro_rules! p {
    ($self:ident, $($arg:tt)*) => {
        if let Some(last) = $self.output.last_mut() {
            last.push_str(format!($($arg)*).as_str())
        } else {
            lp!($self, $($arg)*)
        }
    };
}

impl Visitor {
''' + visitor_body + '''
}
'''


# generate main.rs
main_body = peg.json
main = header + '''
use crate::cache::{CacheResult, CacheType};
use crate::node::{''' + ', '.join(peg.node) + '''};
use crate::parser::Parser;
use crate::visitor::Visitor;

mod parser;
mod stream;
mod cache;
mod visitor;
mod node;

impl Parser {
    pub fn generate(&mut self) -> Option<(String, Vec<String>)> {
        todo!()
    }
}

''' + main_body


# generate parser.rs
parser = header + '''
use crate::cache::{Cache, CacheResult, CacheType};
use crate::memoize;
use crate::stream::Stream;
use std::collections::HashMap;

pub struct Parser {
    pub(crate) stream: Stream,
    pub(crate) cache: Cache,
}

impl Parser {
    pub fn new(input: String, v: bool) -> Self {
        Self {
            stream: Stream {
                body: input,
                cursor: 0,
            },
            cache: Cache {
                body: HashMap::new(),
                verbose: v,
                hit: 0,
            },
        }
    }
}

#[allow(clippy::redundant_closure_call)]
impl Parser {
    pub(crate) fn expect(&mut self, s: &'static str) -> Option<()> {
        memoize!(self, CacheType::Expect(s), CacheResult::Expect, (), {
            if s == "EOF" {
                return if self.stream.peek().is_none() {
                    Some(())
                } else {
                    None
                }
            }
            
            let length = s.len();
            let mut lhs = self.stream.skip();
            let mut rhs = s.chars();
            for _ in 0..length {
                if lhs.next() != rhs.next() {
                    return None;
                }
            }
            self.stream.cursor += length;
            Some(())
        })
    }

    pub(crate) fn name(&mut self) -> Option<String> {
        memoize!(self, CacheType::Name, CacheResult::Name, String, {
            let mut buffer = String::new();
            while let Some(ch) = self.stream.peek() {
                if matches!(ch, 'a'..='z' | 'A'..='Z' | '_') {
                    self.stream.cursor += 1;
                    buffer.push(ch);
                } else {
                    break;
                }
            }
            if buffer.is_empty() {
                None
            } else {
                Some(buffer)
            }
        })
    }

    pub(crate) fn string(&mut self) -> Option<String> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::String, CacheResult::String, String, {
            if self.stream.peek() == Some('"') {
                self.stream.cursor += 1;
            } else {
                return None;
            }
            let mut buffer = String::new();
            while let Some(ch) = self.stream.peek() {
                self.stream.cursor += 1;
                if matches!(ch, '\\"') {
                    return Some(buffer);
                } else {
                    buffer.push(ch);
                }
            }
            self.stream.cursor = origin;
            None
        })
    }

    pub(crate) fn inline(&mut self) -> Option<String> {
        let origin = self.stream.cursor;
        memoize!(self, CacheType::Inline, CacheResult::Inline, String, {
            if self.stream.peek() == Some('{') {
                self.stream.cursor += 1;
            } else {
                return None;
            }
            let mut counter = 0;
            let mut buffer = String::new();
            while let Some(ch) = self.stream.peek() {
                self.stream.cursor += 1;
                match ch {
                    '{' => counter += 1,
                    '}' => counter -= 1,
                    _ => ()
                }
                if counter == -1 {
                    return Some(buffer.trim().to_string());
                } else {
                    buffer.push(ch)
                }
            }
            self.stream.cursor = origin;
            None
        })
    }
}


impl From<CacheResult> for Option<String> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::String(inner) => inner,
            CacheResult::Name(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}

impl From<CacheResult> for Option<()> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Expect(inner) => inner,
            _ => panic!("cache not matched")
        }
    }
}
'''


# generate stream.rs
stream = header + '''
use std::iter::Skip;
use std::str::Chars;

pub struct Stream {
    pub body: String,
    pub cursor: usize,
}

impl Stream {
    pub fn skip(&mut self) -> Skip<Chars<'_>> {
        self.body.chars().skip(self.cursor)
    }

    pub fn peek(&mut self) -> Option<char> {
        self.body.chars().nth(self.cursor)
    }
}
'''


# generate cache.rs
ct_template = '    {nt},'
fmt_ct_list = [ct_template.format(nt=each) for each in peg.node]
ct_body = '\n'.join(fmt_ct_list)

cr_template = '    {nt}(Option<{nt}>),'
fmt_cr_list = [cr_template.format(nt=each) for each in peg.node]
cr_body = '\n'.join(fmt_cr_list)

cr_debug_template = '            CacheResult::{nt}(r) => write!(f, "{{:?}}", r),'
fmt_cr_debug_list = [cr_debug_template.format(nt=each) for each in peg.node]
cr_debug_body = '\n'.join(fmt_cr_debug_list)

cache = header + '''
use crate::node::{''' + ', '.join(peg.node) + '''};
use colored::Colorize;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

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

            let result = || -> Option<$t> {{$func}}();

            let cr = $cr1::$cr2(result.clone());
            $self.cache.insert(origin, ct, cr, $self.stream.cursor);
            result
        }
    };
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum CacheType {
    Expect(&'static str),
    String,
    Inline,
    Name,
''' + ct_body + '''
}

#[derive(Clone)]
pub enum CacheResult {
    Expect(Option<()>),
    String(Option<String>),
    Inline(Option<String>),
    Name(Option<String>),
''' + cr_body + '''
}

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


impl Debug for CacheResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheResult::Expect(r) => write!(f, "{:?}", r),
            CacheResult::String(r) => write!(f, "{:?}", r),
            CacheResult::Inline(r) => write!(f, "{:?}", r),
            CacheResult::Name(r) => write!(f, "{:?}", r),
''' + cr_debug_body + '''
        }
    }
}
'''

import subprocess
import os

if 'parser' not in os.listdir():
    subprocess.run(['cargo', 'init', 'parser', '--lib'])


with open('./parser/src/node.rs', 'w') as file:
    file.write(node)
with open('./parser/src/visitor.rs', 'w') as file:
    file.write(visitor)
with open('./parser/src/lib.rs', 'w') as file:
    file.write(main)
with open('./parser/src/parser.rs', 'w') as file:
    file.write(parser)
with open('./parser/src/stream.rs', 'w') as file:
    file.write(stream)
with open('./parser/src/cache.rs', 'w') as file:
    file.write(cache)
