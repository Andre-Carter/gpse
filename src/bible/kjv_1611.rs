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

pub fn lookup(book: &str, chapter: u8, verse: u8) -> Option<Verse> {
    match (book, chapter, verse) {
        ("genesis", 1, 1) => Some(Verse {
            content: "In the beginning God created the heaven and the earth.",
        }),

        _ => None,
    }
}
