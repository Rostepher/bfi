use byte_stream::ByteStream;

pub use self::Cmd::*;

#[derive(Clone, Eq, PartialEq, Show)]
pub enum Cmd {
    LShift,     // '<'
    RShift,     // '>'
    Inc,        // '+'
    Dec,        // '-'
    Read,       // ','
    Write,      // '.'
    JmpF(Ast),  // '['
    JmpB,       // ']'
}

pub type Ast = Box<Vec<Cmd>>;

pub fn parse<R: Reader>(byte_stream: &mut ByteStream<R>) -> Ast {
    // abstract syntax tree
    let mut ast = box Vec::new();

    loop {
        // get the next char from the stream
        let c = match byte_stream.next_byte() {
            Some(byte) => byte as char,
            None       => break, // end of file
        };

        match c {
            '<' => ast.push(LShift),
            '>' => ast.push(RShift),
            '+' => ast.push(Inc),
            '-' => ast.push(Dec),
            ',' => ast.push(Read),
            '.' => ast.push(Write),
            '[' => ast.push(JmpF(parse(byte_stream))),
            ']' => {
                ast.push(JmpB);
                return ast;
            },
            _   => {} // ignore all other chars
        }
    }

    ast // return the abstract syntax tree
}
