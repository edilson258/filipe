# Welcome to Filipe version 0.1 alpha 7

An interpreted, static typed, single threaded programming language powered by Rust.

Filipe still primitive and limited although it has support for:
- Arithmetic operations
- variables declaration
- if-else statments
- for-loop statments
- Native Data types `int`, `float`, `boolean`, `string`, `null`
- Built-in function `len`, `typeof`, `print`
- User defined functions

Filipe was design to be a high level programming language and beginner friendly, that why it has a clear syntax and an enhanced error reporting mechanisms.

# Examples

1. hello world
```python
print("Hello, world!")
```

2. for loop and if-else statments
```python
for c in range(0, 10) {
  if (c % 2 == 0) {
    print(c, " is even")
  } else {
    print(c, " is odd")
  }
}
```

3. more examples...
```python
define sayHelloTo(name: string): null {
  print("Hello, ", name)
}

sayHelloTo("Edilson")
sayHelloTo("Mom, foo and bar")

let x = 34
let y: int = 35

define sumTwoInts(x: int, y: int): int {
  return x + y
}

print("34 + 35 = ", sumTwoInts(x, y))

for x in range(0, 10) {
  if (x % 2 == 0) {
    print(x, " is even")
  } else {
    print(x, " is odd")
  }
}

if ("hello" == "hell0") {
  print("hello and hell0 are equal")
}

print("10 has type ", typeof(10))
print("10.6 has type ", typeof(10.6))

print("Length of 'Hi Mom' is ", len("Hi Mom"))
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
4. run from file (*.fl)
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
