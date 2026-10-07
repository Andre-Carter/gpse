use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub struct Title {
    pub name: String,
    pub description: String,
}

pub struct Titles<T> {
    pub titles: Vec<T>,
}

impl Title {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

impl<T> Titles<T> {
    pub fn new() -> Self {
        Self { titles: Vec::new() }
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name);
        write!(f, "{}", self.description);

        Ok(())
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Titles<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Titles:")?;

        for (index, title) in self.titles.iter().enumerate() {
            writeln!(f, "  [{index}] {title}")?;
        }

        Ok(())
    }
}
