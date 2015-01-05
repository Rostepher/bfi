const MEM_SIZE: uint = 16;

#[derive(Show)]
pub struct Mem {
    cells: Box<[u8]>,   // address space
    pos: uint           // position in address space
}

impl Mem {
    /// Create a new `Mem` stuct.
    #[inline]
    pub fn new() -> Mem {
        Mem {
            cells: box [0u8; MEM_SIZE],
            pos: 0u
        }
    }

    /// Return the value of cell at the current position.
    #[inline]
    pub fn get(&self) -> u8 {
        self.cells[self.pos]
    }

    /// Set the value at the current position.
    #[inline]
    pub fn set(&mut self, value: u8) {
        self.cells[self.pos] = value;
    }

    /// Shift the current position to the left.
    #[inline]
    pub fn left_shift(&mut self) {
        self.pos -= 1;
    }

    /// Shift the current position to the right.
    #[inline]
    pub fn right_shift(&mut self) {
        self.pos += 1;
    }

    /// Decrement the value at the current position.
    #[inline]
    pub fn decrement(&mut self) {
        self.cells[self.pos] -= 1;
    }

    /// Increment the value at the current position.
    #[inline]
    pub fn increment(&mut self) {
        self.cells[self.pos] += 1;
    }
}
