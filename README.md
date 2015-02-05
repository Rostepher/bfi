# brainfuck

An interpreter for the esoteric language [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)
written in [Rust](https://rust-lang.org) that utilizes a handful of optimization
strategies to drastically improve runtime of executed programs.

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
I claim no ownership of the programs in the `examples` directory. I will gladly
remove any program on request of the author or possible copyright holder.
