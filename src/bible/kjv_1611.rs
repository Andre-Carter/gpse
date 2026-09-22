use std::fmt;

#[derive(Debug)]
pub struct Verse {
    pub content: &'static str,
}

impl fmt::Display for Verse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

pub static GENESIS_001_001: Verse = Verse {
    content: "In the beginning God created the Heauen, and the Earth.",
};
