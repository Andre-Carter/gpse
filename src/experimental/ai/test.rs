pub enum CellType {
    storage,
    connector,
    converter, 
}

// SuperCell: id, connections, etc.

pub struct SuperCell {
    memory: DataType,
}

impl SuperCell {
    pub fn new() -> Self {
        Self { 
            memory: DataType, 
        }
    }
}