use::std::collections::HashMap;
use::std::process::id;

pub struct DataBase<'a, K, V> {
    tag: &'a str,
    id: i8,
    clc: HashMap<K, V>,
}

impl<K, V> DataBase<'_, K, V> {
    pub fn new(&self, tag: &str, id: i8) -> Self {
        Self { 
            tag: self.tag,
            id: self.id,
            clc: HashMap::new(), 
        }
    }

    pub fn add(&self, tag: self.tag, id: self.id) -> Self {
        Self::new(self.tag, self.id)
    }
}

pub fn hangar_one() {
    let mut gpse = HashMap::new();

    gpse.insert(String::from("one"), 1);
    gpse.insert(String::from("two"), 2);

    for (key, value) in &gpse {
        println!("{key}: {value}");
    }

    let system = gpse.get("one").copied().unwrap_or(0);

    dbg!(system);
}