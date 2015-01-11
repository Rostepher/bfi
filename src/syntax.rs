/// Intermediate Representation of Brainfuck operations.
#[derive(Clone, Copy, Eq, PartialEq, Show, String)]
pub enum Ir {
    Add(u8),            // add value to the current cell
    Sub(u8),            // subtract value from the current cell
    MoveLeft(usize),    // move the pointer left by steps
    MoveRight(usize),   // move the pointer right by steps
    Read,               // read byte from user
    Write,              // write byte as char to stdout
    Open,               // opens a loop
    Close,              // closes a loop

    // optimizations
    Clear,              // clear the current cell
    Copy(isize),        //
    Mul(isize, u8),     // multiply the cell at pointer by the
    ScanLeft,           // scan left for a zero cell
    ScanRight,          // scan right for a zero cell
}

/// Abstract Syntax Tree or `Ast`.
pub type Ast = Vec<Ir>;

