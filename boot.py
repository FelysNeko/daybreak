import argparse
import pathlib
import subprocess

import binding

from templates import mini

TARGET = 'parser'
DOTGRAM = 'meta.gram'


def debug_rspegen():
    with open(DOTGRAM) as file:
        grammar = file.read()
    binding.parse(grammar, True)


def init_rspegen():
    with open(DOTGRAM) as file:
        grammar = file.read()
    peg = binding.parse(grammar, False)

    target = pathlib.Path(TARGET)
    src = target.joinpath('src')
    cargo = target.joinpath('Cargo.toml')

    subprocess.run(['cargo', 'init', target])
    subprocess.run(['cargo', 'add', 'colored', '--manifest-path', cargo])
    subprocess.run(['cargo', 'add', 'serde', '--features', 'derive', '--manifest-path', cargo])
    subprocess.run(['cargo', 'add', 'serde_json', '--manifest-path', cargo])

    with open(src.joinpath('main.rs'), 'w') as f:
        mini.Main(peg, f).generate()
    with open(src.joinpath('mapping.rs'), 'w') as f:
        mini.Mapping(peg, f).generate()
    with open(src.joinpath('stable.rs'), 'w') as f:
        mini.Stable(peg, f).generate()


def update_rspegen():
    with open(DOTGRAM) as file:
        grammar = file.read()
    peg = binding.parse(grammar, False)

    target = pathlib.Path(TARGET)
    src = target.joinpath('src')

    with open(src.joinpath('main.rs'), 'w') as f:
        mini.Main(peg, f).generate()
    with open(src.joinpath('mapping.rs'), 'w') as f:
        mini.Mapping(peg, f).generate()
    with open(src.joinpath('stable.rs'), 'w') as f:
        mini.Stable(peg, f).generate()


parser = argparse.ArgumentParser(description='Parser Generator')
subparsers = parser.add_subparsers(dest='command')

subparsers.add_parser('init')
subparsers.add_parser('debug')
subparsers.add_parser('update')

if __name__ == '__main__':
    args = parser.parse_args()
    if args.command == 'init':
        init_rspegen()
    elif args.command == 'debug':
        debug_rspegen()
    elif args.command == 'update':
        update_rspegen()
