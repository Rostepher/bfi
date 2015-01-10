use lexer::{Token, TokenStream};
use syntax::{Ast, Ir};

/// Helper function that builds `Ir::Loop` from a `TokenStream`.
fn build_loop(token_stream: &mut TokenStream) -> Ir {
    let mut tokens = Vec::new();    // tokens inside the loop
    let mut unmatched = 1u32;       // number of unmatched open tokens
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
    Ir::Loop(box ast)
}

/// Parses a `TokenStream` and returns a vaid `Ast`, panics if there is a
/// syntax error.
pub fn parse(token_stream: &mut TokenStream) -> Ast {
    let mut ast = Vec::new();
    loop {
        match token_stream.next_token() {
            Some(token) => match token {
                Token::Add   => ast.push(Ir::Add(1u8)),
                Token::Sub   => ast.push(Ir::Sub(1u8)),
                Token::Left  => ast.push(Ir::MoveLeft(1us)),
                Token::Right => ast.push(Ir::MoveRight(1us)),
                Token::Read  => ast.push(Ir::Read),
                Token::Write => ast.push(Ir::Write),
                Token::Open  => ast.push(build_loop(token_stream)),
                Token::Close => panic!("error: build_loop did not consume Token::Close"),
            },
            None => break, // end of tokens
        }
    }
    ast
}
