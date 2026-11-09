use::Cell;
use::Cells;

pub struct Cell {
    memory: f64,
}

impl Cell {
    pub fn new(memory: f64) -> Self {
        Self { memory }
    }

    pub fn memory(&self) -> f64 {
        self.memory
    }
}

pub struct Cells {
    cells: Vec<Cell>,
}

impl Cells {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
        }
    }

    pub fn add(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }
}