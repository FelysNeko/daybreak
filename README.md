 # PEGParser

A generated PEG grammar parser

## Usage

The following command will parse the grammar file that generates itself and output an `ast.json`.

```sh
cd parser
cargo run
```

## Dev Setup

Prerequisite: Python and Rust environment installed

```sh
python3 -m venv venv
source venv/bin/activate
pip3 install maturin
maturin develop --manifest-path binding/Cargo.toml
```

Then run `python3 boot.py update` will update the three files in [parser/src](parser/src) based on [rspegen.gram](rspegen.gram). You can configure the constants in [boot.py](boot.py) to change the target folder and grammar file location. If the update failed, you may want to use `python3 boot.py debug` to see all the parsing result verbosely. The pink logs represent a cache hit. Lastly, run `python3 boot.py init` to generate the whole [parser](parser) cargo with all dependencies. Again configure the constants to adjust its behaviour.

## Appendix

The original [packrat parsing thesis](https://pdos.csail.mit.edu/~baford/packrat/thesis/thesis.pdf) by Bryan Ford.

Blogs [PEG parsing series overview](https://medium.com/@gvanrossum_83706/peg-parsing-series-de5d41b2ed60) by Guido van Rossum.

This is the prerequisite of future [Felys](https://github.com/felys-lang/felys) syntax, inspired by [PEP-617](https://peps.python.org/pep-0617/).

## License

Distributed under the terms of the [MIT License](LICENSE).

## Copyright

© All rights reserved by FelysNeko

