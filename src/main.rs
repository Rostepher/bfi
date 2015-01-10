// Brainfuck interpreter written in Rust.

#![feature(box_syntax)]

use std::default::Default;
use std::io::File;

use byte_stream::ByteStream;
use eval::eval;
use lexer::tokenize;
use mem::Mem;
use optimizer::{OptConfig, optimize};
use parser::parse;
use syntax::{Ast, Ir};

mod byte_stream;
mod eval;
mod lexer;
mod mem;
mod optimizer;
mod parser;
mod syntax;

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
            let mut ast = parse(&mut token_stream);
            let opt_config = Default::default();
            optimize(&opt_config, &mut ast);
            eval(&mut mem, &ast);
        },
        Err(e) => panic!("{}", e),
    }
}
