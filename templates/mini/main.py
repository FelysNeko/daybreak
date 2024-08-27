from typing import IO
from templates.shared import CLAIM, Generator

class Main(Generator):
    __prelude = '''
use crate::mapping::*;
use crate::stable::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;

mod stable;
mod mapping;


fn main() -> std::io::Result<()> {
    let input = read_to_string("../meta.gram")?;
    let v = true;

    let grammar = Parser {
        stream: Stream {
            body: input,
            cursor: 0,
        },
        cache: Cache {
            body: HashMap::new(),
            verbose: v,
            hit: 0,
        },
    }.grammar();
    
    if let Some(gram) = grammar {
        let json = serde_json::to_string(&gram)?;
        let mut file = File::create("ast.json")?;
        file.write_all(json.as_bytes())?;
    }

    Ok(())
}
'''

    def __init__(self, peg, file: IO[str] | None = None) -> None:
        super().__init__(peg, file)

    def generate(self) -> None:
        self.print(CLAIM)
        self.print(self.__prelude)
        self.print('#[allow(unused_mut)]')
        self.print('impl Parser {')
        with self.indent():
            self.grammar(self.json)
        self.print('}')


    def grammar(self, grammar: dict) -> None:
        for rule in grammar['rules']:
            self.rule(rule)

    def rule(self, rule: dict) -> None:
        rstype = rule['rstype']
        self.print(f'fn {rule['name'].lower()}(&mut self) -> Option<{rstype}> {{')
        with self.indent():
            self.print('let origin = self.stream.cursor;')
            self.print(f'memoize!(self, CacheType::{rstype}, CacheResult::{rstype}, {rstype}, {{')
            with self.indent():
                self.print('let mut cut = false;')
                self.print()
                for alter in rule['alters']:
                    self.alter(alter, rstype)
                self.print('None')
            self.print('})')
        self.print('}')
        self.print()

    def alter(self, alter: dict, rstype: str) -> None:
        self.print(f'if let Some(result) = || -> Option<{rstype}> {{')
        with self.indent():
            for named in alter['nameds']:
                self.named(named)
            self.print(f'Some({alter['inline']})')
        self.print('}() {')
        with self.indent():
            self.print('return Some(result)')
        self.print('} else {')
        with self.indent():
            self.print('self.stream.cursor = origin')
        self.print('}')
        self.print('if cut { return None }')
        self.print()

    def named(self, named: dict) -> None:
        if named == 'Cut':
            self.print('cut = true;')
        elif n := named.get('Identifier'):
            self.print(f'let mut {n[0]} = ', end='')
            self.atom(n[1])
        elif n := named.get('Anonymous'):
            self.print('', end='')
            self.atom(n)
        else:
            raise

    def atom(self, atom: dict) -> None:
        if a := atom.get('String'):
            print(f'self.expect("{a}")?;', file=self.file)
        elif a := atom.get('Name'):
            if a=='STRING' or a=='INLINE' or a=='NAME':
                print(f'self.{a.lower()}()?;', file=self.file)
            else: 
                print(f'self.{a}()?;', file=self.file)
        else:
            raise
