# rustfk

A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in rust. Implements 8 symbols: 
```
><+-.,[]
```
Values in the memory tape overflow/underflow at the bounds of an unsigned byte. Any attempt to move the pointer beyond the ends of the tape results in a error.


# Installing
```
cargo install rustfk
```

# Usage
```
$ rustfk examples/helloworld.b
Hello world!
```