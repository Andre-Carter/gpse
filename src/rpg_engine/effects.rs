pub struct Effect {
    pub name: String,
    pub duration: Option<u32>,
}

pub struct Effects<T> {
    pub effects: Vec<T>,
}

impl Effect {
    pub fn new(name: &str, duration: Option<u32>) -> Self {
        Self {
            name: name.to_string(),
            duration,
        }
    }
}

impl<T> Effects<T> {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Effects<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Effects:")?;

        for (index, effect) in self.effects.iter().enumerate() {
            writeln!(f, "  [{index}] {effect}")?;
        }

        Ok(())
    }
}
