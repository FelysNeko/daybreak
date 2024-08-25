from templates.shared import CLAIM, Generator

class Node(Generator):
    __body_import = '''
use crate::cache::CacheResult;
use std::fmt::{Debug, Formatter};
use serde::Serialize;
'''
    __node_template = '''
#[derive(Clone, Debug, Serialize)]
pub struct {node};

impl From<CacheResult> for Option<{node}> {{
    fn from(value: CacheResult) -> Self {{
        match value {{
            CacheResult::{node}(inner) => inner,
            _ => panic!("cache not matched")
        }}
    }}
}}
'''

    def __init__(self, peg, file = None) -> None:
        super().__init__(peg, file)

    def generate(self) -> None:
        self.print(CLAIM)
        self.print(self.__body_import)
        for each in self.node:
            self.print(self.__node_template.format(node=each))
