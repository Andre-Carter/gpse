pub enum CellType {
    storage,
    connector,
    converter, 
}

pub struct Cell {
    memory: f64,
    //connections: u64,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            memory: f64,
        }
    }
}

pub struct SuperCell {
    id: f64,
    memory: DataType,
    connections: u64,
}

