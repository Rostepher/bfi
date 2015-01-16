# brainfuck

An optimized [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter
written in [Rust](https://rust-lang.org) capable of also emitting the internal
Intermediate Representation (IR) of the Abstract Syntax Tree (AST) and
transpiling brainfuck programs into optimized C and Rust.

## Optimizations

A number of optimizations are implemented in the `optimizer.rs` module. These
include:
* 'contracting' adjacent operations
* multiply/copy loop replacement
* scan loop replacement
* clear loop replacement
* comment loop and unused loop replacement

The optimizations implemented are inspired by the article
[Optimizing Brainfuck](http://calmerthanyouare.org/2015/01/07/optimizing-brainfuck.html)
written by Matz Linander.

## Example Programs

There is a collection of example brainfuck programs in the examples directory.
I claim no ownership of the programs in the examples directory. I will gladly
remove any program on request of the author or possible copyright holder.
