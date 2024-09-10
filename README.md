# Daybreak

A lightweight macro-based framework that helps build reusable packrat parsers with support for left recursion.

Why daybreak? because `「旭光」之后便是满天的「繁星」`.

## Usage

There will be a CLI the generate the project skeleton for you in the future. For now, just refer to the [bootstrap](bootstrap) for main parser structure and [utils](utils) for sub-parsers structure. This project is yet to be registered to crates.io, but will be soon.

By the way, [bootstrap](bootstrap) does not actually bootstrap itself, because the code generation is not implemented and will not be implemented. Since the PEG parser does not require a standalone lexer, this framework takes advantage of that. However, handling whitespace becomes an issue when you attempt to write it into a grammar file. Additionally, the engine API is quite flexible, so programmers should utilize this to optimize the parser and make it more human-readable.

## References

This project is inspired by [pegen](https://github.com/python/cpython/tree/main/Tools/peg_generator) the new cpython parser, and the following links are some relevant resources.

- [PEG Parsing Series Overview](https://medium.com/@gvanrossum_83706/peg-parsing-series-de5d41b2ed60)
- [PEP 617 – New PEG parser for CPython](https://peps.python.org/pep-0617/)
- [Packrat Parsing: a Practical Linear-Time Algorithm with Backtracking](https://pdos.csail.mit.edu/~baford/packrat/thesis/thesis.pdf)

## Appendix

This is framework is initially designed for future [felys](https://github.com/felys-lang/felys) front-end.

## License

Distributed under the terms of the [MIT License](LICENSE).

## Copyright

© All rights reserved by FelysNeko
