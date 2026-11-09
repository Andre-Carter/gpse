use::Cell;
use::Cells;

pub enum Data {
    Float(f64),
    Unsigned(u64),
    Signed(i64),
    Text(String),
}

pub enum DataType {
    Float,
    Unsigned,
    Signed,
    Text,
}

pub struct Cell {
    //kind: DataType,
    memory: f64,
    //connections: u32,
}

impl Cell {
    pub fn new(memory: f64) -> Self {
        Self { memory }
    }

    pub fn memory(&self) -> f64 {
        self.memory
    }

    pub fn write(&mut self, value: f64) {
        self.memory = value;
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