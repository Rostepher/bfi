// re-export Left and Right
pub use self::Dir::{Left, Right};

/// Directions Left or Right.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Dir {
    Left,
    Right,
}

/// Intermediate Representation of Brainfuck operations.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Ir {
    /// Adds the value to the current cell.
    Add(u8),
    /// Subtracts the value from the current cell.
    Sub(u8),
    /// Shifts the pointer in memory left or right by a number of steps.
    Shift(Dir, usize),
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
    /// Copies the value at the current cell to the cell left or right by a
    /// number of steps.
    Copy(Dir, usize),
    /// Multiplies the value at the current cell by a specified factor and
    /// then stores the product in the cell left or right by a number of steps.
    Mul(Dir, usize, i8),
    /// Scans left or right in memory until the value in the current cell is 0.
    Scan(Dir),
}

/// Abstract Syntax Tree or `Ast`.
pub type Ast = Vec<Ir>;
