pub struct Skill {
    pub name: String,
    //pub level: u32,
    //pub experience: u64,
}

pub struct Skills<T> {
    pub skills: Vec<T>,
}

impl Skill {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl<T> Skills<T> {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}", self.name);

        Ok(())
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Skills<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Skills:")?;

        for (index, skill) in self.skills.iter().enumerate() {
            writeln!(f, "  [{index}] {skill}")?;
        }

        Ok(())
    }
}
