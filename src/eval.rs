use std::old_io::{stdin, stdout};

use mem::Mem;
use syntax::{Ast, Ir};

/// Reads a `char` from `stdin`.
fn read_char() -> char {
    match stdin().read_char() {
        Ok(c)  => c,
        Err(e) => panic!("{}", e),
    }
}

/// Writes a `char` to `stdout`.
fn write_char(c: char) {
    match stdout().write_char(c) {
        Ok(_)  => {},
        Err(e) => panic!("{}", e),
    }
}

/// Evaluates an `Ast` iteratively.
pub fn eval(ast: &Ast) {
    // allocated memory
    let mut mem = Mem::new();
    // stack of previous loop open indexes
    let mut stack = Vec::new();

    let mut i = 0us;
    while i < ast.len() {
        match ast[i] {
            Ir::Add(value)         => mem.add(value),
            Ir::Sub(value)         => mem.subtract(value),
            Ir::Shift(dir, steps)  => mem.shift(dir, steps),
            Ir::Read               => mem.set(read_char() as u8),
            Ir::Write              => write_char(mem.get() as char),

            // loops
            Ir::Open => {
                if mem.get() != 0 {
                    stack.push(i);
                } else {
                    // skip to end of loop
                    let mut unmatched = 1u32;
                    while unmatched > 0 && i < ast.len() {
                        i += 1;
                        match ast[i] {
                            Ir::Open  => unmatched += 1,
                            Ir::Close => unmatched -= 1,
                            _         => {}, // skip all other tokens
                        }
                    }

                    // unmatched open
                    if unmatched > 0 {
                        panic!("syntax error: malformed loop!");
                    }
                }
            },
            Ir::Close => {
                let open_index = match stack.pop() {
                    Some(index) => index,
                    None        => panic!("syntax error: malformed loop!"),
                };
                if mem.get() != 0 {
                    // move i to the open index and then the loop will increment
                    // to the next instruction
                    i = open_index;
                    stack.push(open_index);
                }
            },

            // optimizations
            Ir::Clear                   => mem.clear(),
            Ir::Scan(dir)               => mem.scan(dir),
            Ir::Copy(dir, steps)        => mem.copy(dir, steps),
            Ir::Mul(dir, steps, factor) => mem.multiply(dir, steps, factor),
        }

        i += 1; // increment the index
    }
}

