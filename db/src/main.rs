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
