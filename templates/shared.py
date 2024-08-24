from abc import abstractmethod
from contextlib import contextmanager
from typing import Iterator
import json

CLAIM = '// automatically generated from templates'

class Generator:
    def __init__(self, peg) -> None:
        self.json = json.loads(peg.json)
        self.node = peg.node
        self.level = 0

    @abstractmethod
    def generate(self) -> None: ...

    @contextmanager
    def indent(self, n: int = 1) -> Iterator[None]:
        try:
            self.level += n
            yield
        finally:
            self.level -= n

    def print(self, *args: str) -> None:
        if args:
            print('    ' * self.level, end='')
            print(*args)
        else:
            print()
