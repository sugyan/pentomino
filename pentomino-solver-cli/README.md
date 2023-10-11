# Pentomino solver CLI application

```
Usage: pentomino-solver-cli [OPTIONS]

Options:
  -c, --color            Color mode
  -q, --quiet            Quiet mode
  -u, --unique           Unique mode (Discard solutions that are rotations or reflections of others)
  -b, --board <BOARD>    Board type [default: rect6x10] [possible values: rect3x20, rect4x15, rect5x12, rect6x10, rect8x8-2x2]
  -s, --solver <SOLVER>  Solver type [default: default] [possible values: simple, default, optimized-small, optimized-large]
  -h, --help             Print help
  -V, --version          Print version
```