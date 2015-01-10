use std::io::{stdin, stdout};

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

/// Evaluates an `Ast`.
pub fn eval(mem: &mut Mem, ast: &Ast) {
    for ir in ast.iter() {
        match *ir {
            Ir::Add(value)         => mem.add(value),
            Ir::Sub(value)         => mem.subtract(value),
            Ir::MoveLeft(steps)    => mem.move_left(steps),
            Ir::MoveRight(steps)   => mem.move_right(steps),
            Ir::Read               => mem.set(read_char() as u8),
            Ir::Write              => write_char(mem.get() as char),
            Ir::Loop(box ref loop_ast) => {
                while mem.get() != 0 {
                    eval(mem, loop_ast)
                }
            },

            // optimizations
            Ir::Clear              => mem.clear(),
            Ir::Copy(steps)        => mem.copy(steps),
            Ir::Mul(steps, factor) => mem.multiply(steps, factor),
            Ir::ScanLeft           => mem.scan_left(),
            Ir::ScanRight          => mem.scan_right(),
        }
    }
}

