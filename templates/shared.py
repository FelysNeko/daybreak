from abc import abstractmethod
from contextlib import contextmanager
from typing import IO, Iterator, Optional, Text
import json

CLAIM = '// automatically generated from templates'

class Generator:
    def __init__(self, peg, file: Optional[IO[Text]] = None) -> None:
        self.json = json.loads(peg.json)
        self.file = file
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

    def print(self, *args: str, end: str = '\n') -> None:
        if args:
            print('    ' * self.level, end='', file=self.file)
            print(*args, end=end, file=self.file)
        else:
            print(end=end, file=self.file)
