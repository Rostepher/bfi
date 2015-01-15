/// Intermediate Representation of Brainfuck operations.
#[derive(Clone, Copy, Eq, PartialEq, Show)]
pub enum Ir {
    /// Adds the value to the current cell.
    Add(u8),
    /// Subtracts the value from the current cell.
    Sub(u8),
    /// Moves the pointer in memory left by a number of steps.
    MoveLeft(usize),
    /// Moves the pointer in memory right by a number of steps.
    MoveRight(usize),
    /// Reads and stores a single byte into the current cell.
    Read,
    /// Writes the byte at the current cell to `stdout` as a `char`.
    Write,
    /// Opens a loop.
    Open,
    /// Closes a loop.
    Close,

    // optimizations

    /// Clears the current cell.
    Clear,
    /// Copies the value at the current cell to the cell at pointer + a number
    /// of steps.
    Copy(isize),
    /// Multiplies the value at the current cell by a specified factor and
    /// then stores the product in the cell at pointer + a number of steps.
    Mul(isize, u8),
    /// Scans left in memory until the value in the cell at pointer is 0.
    ScanLeft,
    /// Scans right in memory until the value in the cell at pointer is 0.
    ScanRight,
}

/// Abstract Syntax Tree or `Ast`.
pub type Ast = Vec<Ir>;
