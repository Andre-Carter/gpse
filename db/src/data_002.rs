pub enum Dt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    F32(f32),
    F64(f64),
    Str(String),
}

pub struct Data {
    id: String,
    data: Dt,
}

impl Data {
    pub fn new(id: String, data: Dt) -> Self {
        Self { id, data }
    }
}

pub struct DataRow {
    id: usize,
    record: Vec<Data>,
}

impl DataRow {
    pub fn new(id: usize) -> Self { DataRow {
        id: 0,
        record: Vec::new(),
    } }

    pub fn add(&mut self, record: Data) {
        self.record.push(record);
    }

    pub fn len(&self) -> usize {
        self.record.len()
    }

    pub fn is_empty(&self) -> bool {
        self.record.is_empty()
    }
}

pub struct DataTable {
    id: String,
    rows: Vec<DataRow>,
}

