from templates.shared import CLAIM, Generator

class Main(Generator):
    __body_import = '''
use std::collections::HashMap;
use crate::cache::{Cache, CacheResult, CacheType};
use crate::node::*;
use crate::parser::{Parser, Stream};

mod parser;
mod cache;
mod node;
'''

    def __init__(self, peg) -> None:
        super().__init__(peg)

    def generate(self) -> None:
        self.print(CLAIM)
        self.print(self.__body_import)
        self.print('impl Parser {')
        with self.indent():
            self.grammar(self.json)
        self.print('}')


    def grammar(self, grammar: dict) -> None:
        for rule in grammar['rules']:
            self.rule(rule)

    def rule(self, rule: dict) -> None:
        self.print()
        rstype = rule['rstype']
        self.print(f'fn {rstype.lower()}(&mut self) -> Option<{rstype}> {{')
        with self.indent():
            self.print('let origin = self.stream.cursor;')
            self.print(f'memoize!(self, CacheType::{rstype}, CacheResult::{rstype}, {rstype}, {{')
            with self.indent():
                self.print('let mut cut = false;')
                self.print()
                for alter in rule['alters']:
                    self.alter(alter, rstype)
                self.print('None')
            self.print('}')
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
            self.print('let cut = true;')
        elif n := named.get('Identifier'):
            self.print(f'let {n[0]} = ', end='')
            self.atom(n[1])
        elif n := named.get('Anonymous'):
            self.print('', end='')
            self.atom(n)
        else:
            raise

    def atom(self, atom: dict) -> None:
        if a := atom.get('String'):
            print(f'self.expect("{a}")?;')
        elif a := atom.get('Name'):
            if a=='STRING' or a=='INLINE' or a=='NAME':
                print(f'self.{a.lower()}()?;')
            else: 
                print(f'self.{a}()?;')
        else:
            raise
