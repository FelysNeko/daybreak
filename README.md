# Daybreak

A lightweight macro-based framework that helps build reusable packrat parsers with support for left recursion.

Why daybreak? because `「旭光」之后便是满天的「繁星」`.

## Usage

Refer to [example](example) for a full implementation that parses the meta syntax of packrat rules. It is designed for future [felys](https://github.com/felys-lang/felys) syntax, so it is not registered to crates.io. Add the following to `Cargo.toml` to if you want to try it:

```
daybreak = { git = "https://github.com/FelysNeko/daybreak.git" }
```

## References

This project is inspired by [pegen](https://github.com/python/cpython/tree/main/Tools/peg_generator) the new cpython parser, and the following links are some relevant resources.

- [PEG Parsing Series Overview](https://medium.com/@gvanrossum_83706/peg-parsing-series-de5d41b2ed60)
- [PEP 617 – New PEG parser for CPython](https://peps.python.org/pep-0617/)
- [Packrat Parsing: a Practical Linear-Time Algorithm with Backtracking](https://pdos.csail.mit.edu/~baford/packrat/thesis/thesis.pdf)

## License

Distributed under the terms of the [MIT License](LICENSE).

## Copyright

© All rights reserved by FelysNeko
