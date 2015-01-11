// Brainfuck interpreter written in Rust.

#![feature(box_syntax)]

use std::default::Default;
use std::io::File;

use byte_stream::ByteStream;
use eval::eval;
use optimizer::optimize;
use parser::parse;
use syntax::{Ast, Ir};

mod byte_stream;
mod eval;
mod mem;
mod optimizer;
mod parser;
mod syntax;

fn interpret(file_name: &str) {
    let path = Path::new(file_name);
    match File::open(&path) {
        Ok(mut file) => {
            let mut byte_stream = ByteStream::new(&mut file);
            let opt_config = Default::default();
            let ast = optimize(&opt_config, &parse(&mut byte_stream));
            println!("{:?}", ast);
            eval(&ast);
        },
        Err(e) => panic!("{}", e),
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

    interpret(args[1].as_slice());
}
