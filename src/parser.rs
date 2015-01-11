use byte_stream::ByteStream;
use syntax::{Ast, Ir};

/// Parses a `TokenStream` and returns a vaid `Ast`, panics if there is a
/// syntax error.
pub fn parse<R: Reader>(byte_stream: &mut ByteStream<R>) -> Ast {
    let mut ast = Vec::new();
    loop {
        match byte_stream.next_byte() {
            Some(byte) => match byte as char {
                '+' => ast.push(Ir::Add(1u8)),
                '-' => ast.push(Ir::Sub(1u8)),
                '<' => ast.push(Ir::MoveLeft(1us)),
                '>' => ast.push(Ir::MoveRight(1us)),
                ',' => ast.push(Ir::Read),
                '.' => ast.push(Ir::Write),
                '[' => ast.push(Ir::Open),
                ']' => ast.push(Ir::Close),
                _  => {} // ignore all other characters
            },
            None => break, // eof
        }
    }
    ast
}
