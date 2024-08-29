from typing import IO
from templates.shared import CLAIM, Generator


class Mapping(Generator):
    __prelude = '''
use serde::Serialize;
use std::fmt::{Debug, Formatter};
'''

    def __init__(self, peg, file: IO[str] | None = None) -> None:
        super().__init__(peg, file)

    def generate(self) -> None:
        self.print(CLAIM)
        self.print(self.__prelude)
        self.print(self.json['insert'])
        for each in self.node:
            self.print(f'impl From<CacheResult> for Option<{each}> {{')
            with self.indent():
                self.print('fn from(value: CacheResult) -> Self {')
                with self.indent():
                    self.print('match value {')
                    with self.indent():
                        self.print(f'CacheResult::{each}(inner) => inner,')
                        self.print('_ => panic!("cache not matched")')
                    self.print('}')
                self.print('}')
            self.print('}')
            self.print()

        self.print('#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]')
        self.print('pub enum CacheType {')
        with self.indent():
            for each in ['Expect(&\'static str)', 'String', 'Inline', 'Name'] + self.node:
                self.print(f'{each},')
        self.print('}')
        self.print()
        self.print('#[derive(Clone)]')
        self.print('pub enum CacheResult {')
        with self.indent():
            self.print('Expect(Option<()>),')
            self.print('String(Option<String>),')
            self.print('Inline(Option<String>),')
            self.print('Name(Option<String>),')
            for each in self.node:
                self.print(f'{each}(Option<{each}>),')
        self.print('}')
        self.print()
        self.print('impl Debug for CacheResult {')
        with self.indent():
            self.print('fn fmt(&self, f: &mut Formatter<\'_>) -> std::fmt::Result {')
            with self.indent():
                self.print('match self {')
                with self.indent():
                    for each in ['Expect', 'String', 'Inline', 'Name'] + self.node:
                        self.print(f'CacheResult::{each}(r) => write!(f, "{{:?}}", r),')
                self.print('}')
            self.print('}')
        self.print('}')
