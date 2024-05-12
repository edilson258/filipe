# Welcome to Filipe version 0.1 alpha 7

An interpreted, static typed, single threaded programming language powered by Rust.

Filipe still primitive and limited although it has support for:

- Arithmetic operations
- variables declaration
- if-else statments
- loop statments
- Native Data types `int`, `float`, `boolean`, `string`, `null`
- Built-in function `len`, `typeof`, `print`
- User defined functions
- Arrays (experimental)

Filipe was design to be a high level programming language and beginner friendly, that why it has a clear syntax and an enhanced error reporting mechanisms.

# Examples

1. hello world

```python
import IO

define main(): void {
  IO.puts("Hello, world!")
}

main()
```

2. more examples...

```python
let name: string = "Jonh Harvard"
let age = 87
let height: float = 1.86
let favoriteLangs = ["Haskell", "Ocaml", "Rust"]

if height >= 1.90 {
    IO.puts(name, " is tall above avg.")
} else {
    IO.puts(name, " isn't so much tall")
}

for x in range(0, 10) {
    if x % 2 == 0 {
        IO.puts(x, " is even")
    }
}

IO.puts(name)
IO.puts(height)

IO.puts("Favorite prog. lang.: ")
for lang in favoriteLangs {
    IO.puts(lang)
}

```

# Try it now

Just for Linux users, are you on windows? if so figure out your self sorry 😂

1. clone this repository

```shell
git clone https://github.com/edilson258/filipe.git
```

2. build it

```shell
cd filipe
cargo build
```

3. start REPL

```shell
cargo run
```

4. run from file (\*.fl)

```shell
cargo run run <path_to_file>
```

Note: replace `<path_to_file>` with path to filipe script

# Testing

as a php developer with Quick and Dirty mindset i only wrote few tests 😂

```shell
cargo test
```

# Contributions

Feel free to fork it and play with it.
