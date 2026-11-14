fn main() {
    println!("I love Rust!");
}

use std::collections::HashMap;

pub struct DataBase<'a, K, V> {
    tag: &'a str,
    id: i8,
    clc: HashMap<K, V>,
}

impl<'a, K, V> DataBase<'a, K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new(tag: &'a str, id: i8) -> Self {
        Self {
            tag,
            id,
            clc: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: K, value: V) {
        self.clc.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.clc.get(key)
    }

    pub fn delete(&mut self, key: &K) -> Option<V> {
        self.clc.remove(key)
    }

    pub fn len(&self) -> usize {
        self.clc.len()
    }

    pub fn is_empty(&self) -> bool {
        self.clc.is_empty()
    }
}

pub fn hangar_one() {
    let mut gpse = DataBase::new("GPSE", 1);

    gpse.add("one", 1);
    gpse.add("two", 2);

    println!("{}: {}", gpse.tag, gpse.id);

    if let Some(value) = gpse.get(&"one") {
        println!("Value: {value}");
    }
}

// SECTION DATA TABLE
// col: name
// row: number u64

// more data type primitives

pub enum X {
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