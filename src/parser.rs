use byte_stream::ByteStream;
use syntax::{Ast, Ir, Left, Right};

/// Parses a `TokenStream` and returns a vaid `Ast`, panics if there is a
/// syntax error.
pub fn parse<R: Reader>(byte_stream: &mut ByteStream<R>) -> Ast {
    let mut ast = Vec::new();
    let mut open_count = 0u32;
    let mut close_count = 0u32;
    loop {
        match byte_stream.next_byte() {
            Some(byte) => match byte as char {
                '+' => ast.push(Ir::Add(1u8)),
                '-' => ast.push(Ir::Sub(1u8)),
                '<' => ast.push(Ir::Move(Left, 1us)),
                '>' => ast.push(Ir::Move(Right, 1us)),
                ',' => ast.push(Ir::Read),
                '.' => ast.push(Ir::Write),
                '[' => {
                    open_count += 1;
                    ast.push(Ir::Open);
                },
                ']' => {
                    close_count += 1;
                    ast.push(Ir::Close)
                },
                _  => {} // ignore all other characters
            },
            None => break, // eof
        }
    }

    // assert that there is a matching number of '[' and ']'
    if open_count > close_count {
        panic!("syntax error: unmatched '['");
    } else if open_count < close_count {
        panic!("syntax error: unmatched ']'");
    } else {
        ast
    }
}
