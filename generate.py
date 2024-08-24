import binding

with open('rspegen.gram') as file:
    grammar = file.read()

peg = binding.parse(grammar, False)

from templates import rust
rust.Parser(peg).generate()
rust.Node(peg).generate()
rust.Cache(peg).generate()
