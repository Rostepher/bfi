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

    /// Moves the current pointer to the left.
    #[inline]
    pub fn move_left(&mut self, steps: usize) {
        self.ptr -= steps;
    }

    /// Moves the current pointer to the right.
    #[inline]
    pub fn move_right(&mut self, steps: usize) {
        self.ptr += steps;
    }

    // optimizations

    /// Clears the current cell.
    #[inline]
    pub fn clear(&mut self) {
        self.cells[self.ptr] = 0;
    }

    /// Scans left for a zero cell. This fuction will panic! if there is no
    /// zero cell before it scans past the beginning of the address space.
    #[inline]
    pub fn scan_left(&mut self) {
        while self.cells[self.ptr] != 0 {
            self.move_left(1);
        }
    }

    /// Scans right for a zero cell. This function will panic! if there is no
    /// zero cell before it scans past the end of the address space.
    #[inline]
    pub fn scan_right(&mut self) {
        while self.cells[self.ptr] != 0 {
            self.move_right(1);
        }
    }

    /// Copys the value of the current cell into the cell left a number of
    /// steps.
    #[inline]
    pub fn copy_left(&mut self, steps: usize) {
        self.cells[self.ptr - steps] = self.cells[self.ptr];
    }

    /// Copys the value of the current cell into the cell right a number of
    /// steps.
    #[inline]
    pub fn copy_right(&mut self, steps: usize) {
        self.cells[self.ptr + steps] = self.cells[self.ptr];
    }

    /// Multiplys the value of the current cell by a factor and inserts the
    /// product into the cell left a number of steps.
    #[inline]
    pub fn multiply_left(&mut self, steps: usize, factor: u8) {
        self.cells[self.ptr - steps] = self.cells[self.ptr] * factor;
    }

    /// Multiplys the value of the current cell by a factor and inserts the
    /// product into the cell right a number of steps.
    #[inline]
    pub fn multiply_right(&mut self, steps: usize, factor: u8) {
        self.cells[self.ptr + steps] = self.cells[self.ptr] * factor;
    }
}
