// Brainfuck interpreter written in Rust.

#![feature(globs)]

use std::io::File;

use byte_stream::ByteStream;
use mem::Mem;
use parser::*;

mod byte_stream;
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
    let mut i = 0u;
    while i < ast.len() {
        //println!("{} : {}", mem, ast[i]);
        match ast[i] {
            LShift => mem.left_shift(),
            RShift => mem.right_shift(),
            Inc    => mem.increment(),
            Dec    => mem.decrement(),
            Write  => write_char(mem.get() as char),
            Read   => mem.set(read_char() as u8),

            // jump back to the command after the matching '[' if the value
            // in mem is not 0, otherwise continue to the next command.
            JmpB => {
                if mem.get() != 0 {
                    i = 0;
                    continue;
                }
                break;
            },

            // jump forward to the command after the matching ']' if the value
            // in mem is 0, otherwise continue to the next command.
            JmpF(ref loop_ast) => {
                if mem.get() == 0 {
                    continue;
                }
                eval(mem, loop_ast)
            },
        }
        i += 1;
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
            let ast = parse(&mut byte_stream);
            eval(&mut mem, &ast);
        },
        Err(e) => panic!("{}", e),
    }
}
