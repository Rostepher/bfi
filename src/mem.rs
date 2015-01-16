use std::num::SignedInt;

use syntax::{Dir, Left, Right};

// size of allocated memory in bytes
const MEM_SIZE: usize = 65_536; // 64kB!

pub struct Mem {
    cells: Box<[u8]>,   // address space
    ptr: usize           // pointer in address space
}

impl Mem {
    /// Create a new `Mem` stuct.
    #[inline]
    pub fn new() -> Mem {
        Mem {
            cells: box [0u8; MEM_SIZE],
            ptr: 0
        }
    }

    /// Return the value of cell at the current pointer.
    #[inline]
    pub fn get(&self) -> u8 {
        self.cells[self.ptr]
    }

    /// Set the value at the current pointer.
    #[inline]
    pub fn set(&mut self, value: u8) {
        self.cells[self.ptr] = value;
    }

    /// Adds `value` to the current cell.
    #[inline]
    pub fn add(&mut self, value: u8) {
        self.cells[self.ptr] += value;
    }

    /// Subtracts `value` from the current cell.
    #[inline]
    pub fn subtract(&mut self, value: u8) {
        self.cells[self.ptr] -= value;
    }

    /// Shifts the current pointer to the left or right by a number of steps.
    #[inline]
    pub fn shift(&mut self, dir: Dir, steps: usize) {
        match dir {
            Left  => self.ptr -= steps,
            Right => self.ptr += steps,
        }
    }

    // optimizations

    /// Clears the current cell.
    #[inline]
    pub fn clear(&mut self) {
        self.cells[self.ptr] = 0;
    }

    /// Scans left or right for a zero cell. This fuction will panic! if there
    /// is no zero cell before it scans past the beginning of the address space.
    #[inline]
    pub fn scan(&mut self, dir: Dir) {
        while self.cells[self.ptr] != 0 {
            self.shift(dir, 1);
        }
    }

    /// Copys the value of the current cell into the cell left or right a
    /// number of steps.
    #[inline]
    pub fn copy(&mut self, dir: Dir, steps: usize) {
        let index = match dir {
            Left  => self.ptr - steps,
            Right => self.ptr + steps,
        };
        self.cells[index] += self.cells[self.ptr];
    }

    /// Multiplys the value of the current cell by a factor and inserts the
    /// product into the cell left or right a number of steps.
    pub fn multiply(&mut self, dir: Dir, steps: usize, factor: i8) {
        let index = match dir {
            Left  => self.ptr - steps,
            Right => self.ptr + steps,
        };

        // safely cast factor to u8
        let u8_factor = SignedInt::abs(factor) as u8;

        // when factor is 1 it acts like a copy
        if factor == 1 {
            self.cells[index] += self.cells[self.ptr];
        }

        // when factor is -1 it acts like the inverse of copy
        else if factor == -1 {
            self.cells[index] -= self.cells[self.ptr];
        }

        // when factor is >= 2 it adds the product of the current cell and the
        // absolute value of factor to the cell at index
        else if factor >= 2 {
            self.cells[index] += self.cells[self.ptr] * u8_factor;
        }

        // when factor is <= 2 it subtracts the product of the current cell and the
        // absolute value of factor to the cell at index
        else if factor <= 2 {
            self.cells[index] -= self.cells[self.ptr] * u8_factor;
        }

        // when factor is 0 it is ignored, as it would do nothing
        else {}
    }
}
