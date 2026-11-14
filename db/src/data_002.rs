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
    records: Vec<Data>,
}

impl DataRow {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            records: Vec::new(),
        }
    }

    pub fn add(&mut self, records: Data) {
        self.records.push(records);
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

pub struct DataTable {
    id: String,
    rows: Vec<DataRow>,
}

impl DataTable {
    pub fn new(id: String) -> Self {
        Self {
            id,
            rows: Vec::new(),
        }
    }

    pub fn add(&mut self, row: DataRow) {
        self.rows.push(row);
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

pub fn main() {
    let mut table = DataTable::new("GPSE".into());

    let mut row = DataRow::new(1);

    row.add(Data::new(
        "temperature".into(),
        Dt::F64(273.15),
    ));

    row.add(Data::new(
        "name".into(),
        Dt::Str("Earth".into()),
    ));

    table.add(row);

    println!("Rows: {}", table.len());
}
