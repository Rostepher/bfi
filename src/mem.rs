const MEM_SIZE: uint = 4_096;

#[derive(Show)]
pub struct Mem {
    cells: Box<[u8]>,   // address space
    pos: uint           // position in address space
}

// TODO: add safety checks for moving pos and changing cells
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
    pub fn move_left(&mut self, steps: uint) {
        self.pos -= steps;
    }

    /// Shift the current position to the right.
    #[inline]
    pub fn move_right(&mut self, steps: uint) {
        self.pos += steps;
    }

    /// Decrement the value at the current position.
    #[inline]
    pub fn decrement(&mut self, value: u8) {
        self.cells[self.pos] -= value;
    }

    /// Increment the value at the current position.
    #[inline]
    pub fn increment(&mut self, value: u8) {
        self.cells[self.pos] += value;
    }
}
