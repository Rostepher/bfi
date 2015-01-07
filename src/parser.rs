use lexer::{Token, TokenStream};

/// Possible operations.
#[derive(Clone, Eq, PartialEq, Show)]
pub enum Op {
    Left(uint),     // move the pointer left by steps
    Right(uint),    // move the pointer right by steps
    Incr(u8),       // increment the cell by value
    Decr(u8),       // decrement the cell by value
    Read,           // read byte from user
    Write,          // write byte as char to stdout
    Loop(Box<Ast>), // loop while current cell is 0
}

/// Abstract Syntax Tree or `Ast`.
pub type Ast = Vec<Op>;

/// Helper function that builds `Some(Op::Left)`, `Some(Op::Right)` or `None`
/// if contiguous right and left ops balance out.
fn build_move(token_stream: &mut TokenStream) -> Option<Op> {
    let mut left_steps = 0u;
    let mut right_steps = 0u;

    // take into account the previously consumed token
    match token_stream.prev_token() {
        Some(token) => {
            if token == Token::Left {
                left_steps += 1;
            } else {
                right_steps += 1;
            }
        },
        None => panic!("error: previous token should exist!"),
    }

    loop {
        match token_stream.peek_token() {
            Some(token) => match token {
                Token::Left  => {
                    token_stream.consume_token();
                    left_steps += 1;
                },
                Token::Right => {
                    token_stream.consume_token();
                    right_steps += 1;
                },
                _ => break,
            },
            None => break,
        }
    }

    // determine which op to create or None if they balance out
    if left_steps > right_steps {
        Some(Op::Left(left_steps - right_steps))
    } else if left_steps < right_steps {
        Some(Op::Right(right_steps - left_steps))
    } else {
        None
    }
}

/// Helper function that builds `Some(Op::Incr)`, `Some(Op::Decr)` or `None`
/// if contiguous incr and decr ops balance out.
fn build_incr_decr(token_stream: &mut TokenStream) -> Option<Op> {
    let mut incr_val = 0u8;
    let mut decr_val = 0u8;

    // take into account the previously consumed token
    match token_stream.prev_token() {
        Some(token) => {
            if token == Token::Incr {
                incr_val += 1;
            } else {
                decr_val += 1;
            }
        },
        None => panic!("error: previous token should exist!"),
    }

    loop {
        match token_stream.peek_token() {
            Some(token) => match token {
                Token::Incr  => {
                    token_stream.consume_token();
                    incr_val += 1;
                },
                Token::Decr => {
                    token_stream.consume_token();
                    decr_val += 1;
                },
                _ => break,
            },
            None => break,
        }
    }

    // determine which op to create or None if they balance out
    if incr_val > decr_val {
        Some(Op::Incr(incr_val - decr_val))
    } else if incr_val < decr_val {
        Some(Op::Decr(decr_val - incr_val))
    } else {
        None
    }
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
    let ast = parse_token_stream(&mut TokenStream::new(box tokens));
    Op::Loop(box ast)
}

/// Parses a `TokenStream` and returns a vaid `Ast`, panics if there is a
/// syntax error.
fn parse_token_stream(token_stream: &mut TokenStream) -> Ast {
    let mut ast = Vec::new();
    loop {
        match token_stream.next_token() {
            Some(token) => match token {
                Token::Left  |
                Token::Right => {
                    match build_move(token_stream) {
                        Some(op) => ast.push(op),
                        None     => {}, // do nothing
                    }
                },
                Token::Incr  |
                Token::Decr  => {
                    match build_incr_decr(token_stream) {
                        Some(op) => ast.push(op),
                        None     => {}, // do nothing
                    }
                },
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

/// Parses a `TokenStream` and returns a vlaid, optimized `Ast`, panics if
/// there is a syntax error.
pub fn parse(token_stream: &mut TokenStream) -> Ast {
    let mut ast = parse_token_stream(token_stream);

    // remove 'comment loop'(s) at the beginning of the tree, as they will
    // never be executed.
    while !ast.is_empty() {
        match ast[0] {
            Op::Loop(_) => { ast.remove(0); },
            _           => break, // all comment loops removed
        }
    }

    println!("{}", ast);

    // return optimized ast
    ast
}
