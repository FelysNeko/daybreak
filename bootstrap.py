PEG = 'parser'
DOTGRAM = 'rspegen.gram'

def debug_rspegen():
    import binding
    with open(DOTGRAM) as file:
        grammar = file.read()
    binding.parse(grammar, True)


def init_rspegen():
    import binding
    with open(DOTGRAM) as file:
        grammar = file.read()
    peg = binding.parse(grammar, False)

    import pathlib
    target = pathlib.Path(PEG)
    src = target.joinpath('src')
    cargo = target.joinpath('Cargo.toml')

    import subprocess
    subprocess.run(['cargo', 'init', target])
    subprocess.run(['cargo', 'add', 'colored', '--manifest-path', cargo])
    subprocess.run(['cargo', 'add', 'serde', '--features', 'derive', '--manifest-path', cargo])
    subprocess.run(['cargo', 'add', 'serde_json', '--manifest-path', cargo])

    from templates import mini
    with open(src.joinpath('main.rs'), 'w') as f:
        mini.Main(peg, f).generate()
    with open(src.joinpath('mapping.rs'), 'w') as f:
        mini.Mapping(peg, f).generate()
    with open(src.joinpath('stable.rs'), 'w') as f:
        mini.Stable(peg, f).generate()


def update_rspegen():
    import binding
    with open(DOTGRAM) as file:
        grammar = file.read()
    peg = binding.parse(grammar, False)

    import pathlib
    target = pathlib.Path(PEG)
    src = target.joinpath('src')

    from templates import mini
    with open(src.joinpath('main.rs'), 'w') as f:
        mini.Main(peg, f).generate()
    with open(src.joinpath('mapping.rs'), 'w') as f:
        mini.Mapping(peg, f).generate()
    with open(src.joinpath('stable.rs'), 'w') as f:
        mini.Stable(peg, f).generate()


import argparse
argp = argparse.ArgumentParser(description='Parser Generator') 
subparsers = argp.add_subparsers(dest='command')

initcmd = subparsers.add_parser('init')
debugcmd = subparsers.add_parser('debug')
updatecmd = subparsers.add_parser('update')


if __name__ == '__main__':
    args = argp.parse_args()
    if args.command == 'init':
        init_rspegen()
    elif args.command == 'debug':
        debug_rspegen()
    elif args.command == 'update':
        update_rspegen()
