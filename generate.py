import binding

with open('rspegen.gram') as file:
    grammar = file.read()

peg = binding.parse(grammar, False)


import subprocess
import pathlib

target = pathlib.Path('peg')
src = target.joinpath('src')
cargo = target.joinpath('Cargo.toml')

subprocess.run(['cargo', 'init', target])
subprocess.run(['cargo', 'add', 'colored', '--manifest-path', cargo])
subprocess.run(['cargo', 'add', 'serde', '--features', 'derive', '--manifest-path', cargo])
subprocess.run(['cargo', 'add', 'serde_json', '--manifest-path', cargo])


from templates import rust
with open(src.joinpath('parser.rs'), 'w') as f:
    rust.Parser(peg, f).generate()
with open(src.joinpath('node.rs'), 'w') as f:
    rust.Node(peg, f).generate()
with open(src.joinpath('cache.rs'), 'w') as f:
    rust.Cache(peg, f).generate()
with open(src.joinpath('main.rs'), 'w') as f:
    rust.Main(peg, f).generate()
