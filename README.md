 # RsPEGen

PEG parser generator for multiple languages. 

## Setup

Prerequisite: Python and Rust environment installed

```sh
python3 -m venv venv
source venv/bin/activate
pip3 install maturin
maturin develop --manifest-path binding/Cargo.toml
```

Then run `python3 boot.py update` will update the three files in [parser/src](parser/src) based on [rspegen.gram](rspegen.gram). You can configure the constants in [boot.py](boot.py) to change the target folder and grammar file location. If the update failed, you may want to use `python3 boot.py debug` to see all the parsing result verbosely. The pink logs represent a cache hit. Lastly, run `python3 boot.py init` to generate to the whole [parser](parser) cargo with all dependencies. Again configure the constants to adjust its behaviour.

To see whether things get compiled, run:

```sh
cargo run --manifest-path parser/Cargo.toml
```

**NOTE**: you don't need `--manifest-path <PATH>` if you don't bother `cd` into directories.

## Todo

Only the the parser AST is now ready to go. More python scripts are needed to generate actual code that make this parser work. The mini parser generator is not powerful nor flexible enough, and does not bootstrap.

Or simple re-implement the whole project, since there are quite a few immature designs.

## Appendix

The original [packrat parsing thesis](https://pdos.csail.mit.edu/~baford/packrat/thesis/thesis.pdf) by Bryan Ford.

Blogs [PEG parsing series overview](https://medium.com/@gvanrossum_83706/peg-parsing-series-de5d41b2ed60) by Guido van Rossum.

This is the prerequisite of future [Felys](https://github.com/felys-lang/felys) syntax, inspired by [PEP-617](https://peps.python.org/pep-0617/).

## License

Distributed under the terms of the [MIT License](https://github.com/FelysNeko/rspegen/blob/main/LICENSE).

## Copyright

© All rights reserved by FelysNeko

