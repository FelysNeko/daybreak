def debug_rspegen(dotgram):
    import binding
    with open(dotgram) as file:
        grammar = file.read()
    binding.parse(grammar, True)


def init_rspegen(dotgram):
    import binding
    with open(dotgram) as file:
        grammar = file.read()
    peg = binding.parse(grammar, False)

    import pathlib
    target = pathlib.Path('peg')
    src = target.joinpath('src')
    cargo = target.joinpath('Cargo.toml')

    import subprocess
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


def update_rspegen(dotgram, target):
    import binding
    with open(dotgram) as file:
        grammar = file.read()
    peg = binding.parse(grammar, False)

    import pathlib
    target = pathlib.Path('peg')
    src = target.joinpath('src')

    from templates import rust
    match target:
        case 'parser.rs':
            with open(src.joinpath('parser.rs'), 'w') as f:
                rust.Parser(peg, f).generate()
        case 'node.rs':
            with open(src.joinpath('node.rs'), 'w') as f:
                rust.Node(peg, f).generate()
        case 'cache.rs':
            with open(src.joinpath('cache.rs'), 'w') as f:
                rust.Cache(peg, f).generate()
        case 'main.rs':
            with open(src.joinpath('main.rs'), 'w') as f:
                rust.Main(peg, f).generate()



import argparse
argp = argparse.ArgumentParser(description='Parser Generator') 
subparsers = argp.add_subparsers(dest='command')

initcmd = subparsers.add_parser('init')
debugcmd = subparsers.add_parser('debug')
updatecmd = subparsers.add_parser('update')
updatecmd.add_argument('file', choices=['parser.rs', 'node.rs', 'cache.rs', 'main.rs'])


if __name__ == '__main__':
    dotgram = 'rspegen.gram'
    args = argp.parse_args()
    if args.command == 'init':
        init_rspegen(dotgram)
    elif args.command == 'debug':
        debug_rspegen(dotgram)
    elif args.command == 'update':
        update_rspegen(dotgram, args.file)
