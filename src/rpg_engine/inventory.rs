use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub struct Inventory<T> {
    items: Vec<T>,
}

pub struct Item {
    pub name: String,
}

impl<Item> Inventory<Item> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl Item {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Inventory<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Inventory:")?;

        for (index, item) in self.items.iter().enumerate() {
            writeln!(f, "  [{index}] {item}")?;
        }

        Ok(())
    }
}
