use byte_stream::ByteStream;

#[derive(Clone, Copy, Eq, PartialEq, Show)]
pub enum Token {
    Left,   // '<'
    Right,  // '>'
    Add,    // '+'
    Sub,    // '-'
    Read,   // ','
    Write,  // '.'
    Open,   // '['
    Close,  // ']'
}

pub struct TokenStream {
    tokens: Box<Vec<Token>>,
    cursor: usize
}

impl TokenStream {
    #[inline]
    pub fn new(tokens: Box<Vec<Token>>) -> TokenStream {
        TokenStream {
            tokens: tokens,
            cursor: 0
        }
    }

    /// Consumes the current token.
    #[inline]
    pub fn consume_token(&mut self) {
        self.cursor += 1;
    }

    /// Returns `Some(token)` or `None` if the token stream is empty.
    pub fn peek_token(&self) -> Option<Token> {
        if self.cursor < self.tokens.len() {
            Some(self.tokens[self.cursor])
        } else {
            None
        }
    }

    /// Returns `Some(token)` or `None` if the token stream is empty and
    /// consumes the returned token.
    pub fn next_token(&mut self) -> Option<Token> {
        let token = self.peek_token();
        self.consume_token();
        token
    }

    /// Return the previous token.
    pub fn prev_token(&self) -> Option<Token> {
        if self.cursor == 0 {
            None
        } else {
            Some(self.tokens[self.cursor - 1])
        }
    }
}

/// Tokenizes a `ByteStream` and returns a `TokenStream`.
pub fn tokenize<R: Reader>(byte_stream: &mut ByteStream<R>) -> TokenStream {
    let mut tokens = Vec::new();
    loop {
        match byte_stream.next_byte() {
            Some(byte) => match byte as char {
                '<' => tokens.push(Token::Left),
                '>' => tokens.push(Token::Right),
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Sub),
                ',' => tokens.push(Token::Read),
                '.' => tokens.push(Token::Write),
                '[' => tokens.push(Token::Open),
                ']' => tokens.push(Token::Close),
                _   => {}, // ignore all other chars
            },
            None => break, // eof
        };

    }
    TokenStream::new(box tokens)
}
