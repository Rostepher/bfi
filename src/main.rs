// Brainfuck interpreter written in Rust.

#![feature(globs)]

use std::io::File;

use byte_stream::ByteStream;
use lexer::tokenize;
use mem::Mem;
use parser::{Ast, Op, parse};

mod byte_stream;
mod lexer;
mod mem;
mod parser;

/// Reads a `char` from `stdin`.
fn read_char() -> char {
    match std::io::stdin().read_char() {
        Ok(c)  => c,
        Err(e) => panic!("{}", e),
    }
}

/// Writes a `char` to `stdout`.
fn write_char(c: char) {
    match std::io::stdout().write_char(c) {
        Ok(_)  => {},
        Err(e) => panic!("{}", e),
    }
}

/// Eval function of interpreter.
fn eval(mem: &mut Mem, ast: &Ast) {
    for mut i in range(0, ast.len()) {
        match ast[i] {
            Op::Left(steps)  => mem.move_left(steps),
            Op::Right(steps) => mem.move_right(steps),
            Op::Incr(value)  => mem.increment(value),
            Op::Decr(value)  => mem.decrement(value),
            Op::Read         => mem.set(read_char() as u8),
            Op::Write        => write_char(mem.get() as char),
            Op::Loop(box ref loop_ast) => {
                while mem.get() != 0 {
                    eval(mem, loop_ast)
                }
            },
        }
    }
}

/// Main function.
fn main() {
    // parse args
    let args = std::os::args();
    if args.len() != 2 {
        let stderr = &mut std::io::stderr();
        match writeln!(stderr, "usage: {} filename", args[0]) {
            Ok(_)  => return,
            Err(e) => panic!("{}", e)
        }
    }

    // parse file to create the abstract syntax tree and then evaluate it
    let file_name = args[1].as_slice();
    let path = Path::new(file_name);
    match File::open(&path) {
        Ok(mut file) => {
            let mut mem = Mem::new();
            let mut byte_stream = ByteStream::new(&mut file);
            let mut token_stream = tokenize(&mut byte_stream);
            let ast = parse(&mut token_stream);
            eval(&mut mem, &ast);
        },
        Err(e) => panic!("{}", e),
    }
}
