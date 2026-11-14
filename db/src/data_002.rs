pub enum D {
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    F32,
    F64,
    Str,
}

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

pub struct Data<'a> {
    id: &'a str,
    dt: Dt,
}

impl<'a> Data<'a> {
    pub fn new(id: &'a str, dt: Dt) -> Self {
        Self { 
            id, 
            dt,
        }
    }
}

pub struct Col<'a> {
    id: &'a str,
    data: Data<'a>,
}

pub struct Row<'a> {
    id: usize, 
    col: Vec<Col<'a>>, 
}

pub struct DataTable<'a> {
    id: &'a str,
    rows: Vec<Row<'a>>,
}