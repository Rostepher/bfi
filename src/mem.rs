// size of allocated memory in bytes
const MEM_SIZE: uint = 65_536; // 64kB!

#[derive(Show)]
pub struct Mem {
    cells: Box<[u8]>,   // address space
    ptr: uint           // pointer in address space
}

impl Mem {
    /// Create a new `Mem` stuct.
    #[inline]
    pub fn new() -> Mem {
        Mem {
            cells: box [0u8; MEM_SIZE],
            ptr: 0u
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

    /// Shift the current pointer to the left.
    #[inline]
    pub fn move_left(&mut self, steps: uint) {
        self.ptr -= steps;
    }

    /// Shift the current pointer to the right.
    #[inline]
    pub fn move_right(&mut self, steps: uint) {
        self.ptr += steps;
    }

    /// Decrement the value at the current pointer.
    #[inline]
    pub fn decrement(&mut self, value: u8) {
        self.cells[self.ptr] -= value;
    }

    /// Increment the value at the current pointer.
    #[inline]
    pub fn increment(&mut self, value: u8) {
        self.cells[self.ptr] += value;
    }
}
