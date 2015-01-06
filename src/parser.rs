use lexer::{Token, TokenStream};

#[derive(Clone, Copy, Eq, PartialEq, Show)]
pub enum Dir {
    Left,
    Right
}

#[derive(Clone, Eq, PartialEq, Show)]
pub enum Op {
    Move(Dir, uint),    // move left or right a number of steps
    Incr(u8),           // increment the cell by value
    Decr(u8),           // decrement the cell by value
    Read,               // read byte from user
    Write,              // write byte as char to stdout
    Loop(Box<Ast>),     // loop while current cell is 0
}

/// Abstract Syntax Tree or `Ast`.
pub type Ast = Vec<Op>;

/// Optimizes an `Ast` to remove the beginning 'loop comment' and combines
/// contiguous similar operations. Such as combining a series of `Move` ops
/// into a single `Move` or removes it altogether in the case that the
/// moves would balance out.
fn optimize_ast(ast: &mut Ast) {
    // TODO
}

/// Helper function that builds `Op::Loop` from a `TokenStream`.
fn build_loop(token_stream: &mut TokenStream) -> Op {
    let mut tokens = Vec::new();    // tokens inside the loop
    let mut unmatched = 1u;         // number of unmatched open tokens
    while unmatched > 0 {
        match token_stream.next_token() {
            Some(token) => {
                // adjust unmatched for new nested loops
                if token == Token::Open {
                    unmatched += 1;
                } else if token == Token::Close {
                    unmatched -= 1;
                }

                // add all but the matched close token to tokens
                if unmatched > 0 || token != Token::Close {
                    tokens.push(token);
                }
            },
            None => break, // token stream is empty
        }
    }

    // still unmatched open tokens, but the token stream is empty
    if unmatched > 0 {
        panic!("syntax error: malformed loop");
    }

    // parse the tokens inside the loop and return the loop
    let ast = parse(&mut TokenStream::new(box tokens));
    Op::Loop(box ast)
}

/// Parses a `TokenStream` and returns a vaid `Ast`, panics if there is a
/// syntax error.
pub fn parse(token_stream: &mut TokenStream) -> Ast {
    let mut ast = Vec::new();
    loop {
        match token_stream.next_token() {
            Some(token) => match token {
                Token::Left  => ast.push(Op::Move(Dir::Left, 1u)),
                Token::Right => ast.push(Op::Move(Dir::Right, 1u)),
                Token::Incr  => ast.push(Op::Incr(1u8)),
                Token::Decr  => ast.push(Op::Decr(1u8)),
                Token::Read  => ast.push(Op::Read),
                Token::Write => ast.push(Op::Write),
                Token::Open  => ast.push(build_loop(token_stream)),
                Token::Close => panic!("error: build_loop did not consume Token::Close"),
            },
            None => break, // end of tokens
        }
    }
    ast
}
