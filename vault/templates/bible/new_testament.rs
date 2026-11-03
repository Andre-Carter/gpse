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

pub fn lookup_kjv1611(book: &str, chapter: u8, verse: u8) -> Option<Verse> {
    match (book, chapter, verse) {

        // THE BOOK OF MATTHEW

        ("matthew", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 47) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 5, 48) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 6, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 7, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 8, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 9, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 10, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 47) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 48) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 49) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 12, 50) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 47) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 48) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 49) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 50) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 51) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 52) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 53) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 54) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 55) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 56) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 57) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 13, 58) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 14, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 15, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 16, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 17, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 18, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 19, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 20, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 21, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 22, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 23, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 47) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 48) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 49) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 50) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 24, 51) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 25, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 47) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 48) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 49) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 50) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 51) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 52) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 53) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 54) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 55) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 56) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 57) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 58) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 59) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 60) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 61) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 62) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 63) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 64) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 65) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 66) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 67) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 68) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 69) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 70) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 71) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 72) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 73) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 74) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 26, 75) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 20) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 21) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 22) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 23) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 24) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 25) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 26) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 27) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 28) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 29) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 30) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 31) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 32) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 33) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 34) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 35) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 36) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 37) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 38) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 39) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 40) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 41) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 42) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 43) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 44) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 45) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 46) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 47) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 48) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 49) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 50) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 51) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 52) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 53) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 54) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 55) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 56) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 57) => Some(Verse { 
            content: "",
        }),

        ("matthew", 27, 58) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 59) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 60) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 61) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 62) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 63) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 64) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 65) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 27, 66) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 1) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 2) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 3) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 4) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 5) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 6) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 7) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 8) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 9) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 10) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 11) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 12) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 13) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 14) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 15) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 16) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 17) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 18) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 19) => Some(Verse { 
            content: "", 
        }),

        ("matthew", 28, 20) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF MARK

        ("mark", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 44) => Some(Verse { 
            content: "", 
        }),

        ("mark", 1, 45) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 2, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 3, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 4, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 5, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 44) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 45) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 46) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 47) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 48) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 49) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 50) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 51) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 52) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 53) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 54) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 55) => Some(Verse { 
            content: "", 
        }),

        ("mark", 6, 56) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 7, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 8, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 44) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 45) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 46) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 47) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 48) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 49) => Some(Verse { 
            content: "", 
        }),

        ("mark", 9, 50) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 44) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 45) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 46) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 47) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 48) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 49) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 50) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 51) => Some(Verse { 
            content: "", 
        }),

        ("mark", 10, 52) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 11, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 12, 44) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 13, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 44) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 45) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 46) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 47) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 48) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 49) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 50) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 51) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 52) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 53) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 54) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 55) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 56) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 57) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 58) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 59) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 60) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 61) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 62) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 63) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 64) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 65) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 66) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 67) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 68) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 69) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 70) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 71) => Some(Verse { 
            content: "", 
        }),

        ("mark", 14, 72) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 20) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 21) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 22) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 23) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 24) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 25) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 26) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 27) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 28) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 29) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 30) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 31) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 32) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 33) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 34) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 35) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 36) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 37) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 38) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 39) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 40) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 41) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 42) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 43) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 44) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 45) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 46) => Some(Verse { 
            content: "", 
        }),

        ("mark", 15, 47) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("mark", 16, 20) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF LUKE

        ("luke", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 53) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 54) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 55) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 56) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 57) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 58) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 59) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 60) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 61) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 62) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 63) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 64) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 65) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 66) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 67) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 68) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 69) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 70) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 71) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 72) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 73) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 74) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 75) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 76) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 77) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 78) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 79) => Some(Verse { 
            content: "", 
        }),

        ("luke", 1, 80) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 2, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 3, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 4, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 5, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 6, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 8) => Some(Verse { 
            content:"", 
        }),

        ("luke", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 7, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 53) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 54) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 55) => Some(Verse { 
            content: "", 
        }),

        ("luke", 8, 56) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 53) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 54) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 55) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 56) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 57) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 58) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 59) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 60) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 61) => Some(Verse { 
            content: "", 
        }),

        ("luke", 9, 62) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 10, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 53) => Some(Verse { 
            content: "", 
        }),

        ("luke", 11, 54) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 53) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 54) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 55) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 56) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 57) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 58) => Some(Verse { 
            content: "", 
        }),

        ("luke", 12, 59) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 13, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 14, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 15, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 16, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 17, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 18, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 19, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 20, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 21, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 53) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 54) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 55) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 56) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 57) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 58) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 59) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 60) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 61) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 62) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 63) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 64) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 65) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 66) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 67) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 68) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 69) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 70) => Some(Verse { 
            content: "", 
        }),

        ("luke", 22, 71) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 53) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 54) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 55) => Some(Verse { 
            content: "", 
        }),

        ("luke", 23, 56) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 1) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 2) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 3) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 4) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 5) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 6) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 7) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 8) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 9) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 10) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 11) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 12) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 13) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 14) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 15) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 16) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 17) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 18) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 19) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 20) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 21) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 22) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 23) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 24) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 25) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 26) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 27) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 28) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 29) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 30) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 31) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 32) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 33) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 34) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 35) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 36) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 37) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 38) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 39) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 40) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 41) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 42) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 43) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 44) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 45) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 46) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 47) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 48) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 49) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 50) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 51) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 52) => Some(Verse { 
            content: "", 
        }),

        ("luke", 24, 53) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF JOHN

        ("john", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 48) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 49) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 50) => Some(Verse { 
            content: "", 
        }),

        ("john", 1, 51) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 3, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 48) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 49) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 50) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 51) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 52) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 53) => Some(Verse { 
            content: "", 
        }),

        ("john", 4, 54) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 5, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 48) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 49) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 50) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 51) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 52) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 53) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 54) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 55) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 56) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 57) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 58) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 59) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 60) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 61) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 62) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 63) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 64) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 65) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 66) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 67) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 68) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 69) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 70) => Some(Verse { 
            content: "", 
        }),

        ("john", 6, 71) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 48) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 49) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 50) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 51) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 52) => Some(Verse { 
            content: "", 
        }),

        ("john", 7, 53) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 48) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 49) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 50) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 51) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 52) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 53) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 54) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 55) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 56) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 57) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 58) => Some(Verse { 
            content: "", 
        }),

        ("john", 8, 59) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 9, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 10, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 48) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 49) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 50) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 51) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 52) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 53) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 54) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 55) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 56) => Some(Verse { 
            content: "", 
        }),

        ("john", 11, 57) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 43) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 44) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 45) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 46) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 47) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 48) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 49) => Some(Verse { 
            content: "", 
        }),

        ("john", 12, 50) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 13, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 14, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 15, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 16, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 17, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 18, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 32) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 33) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 34) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 35) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 36) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 37) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 38) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 39) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 40) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 41) => Some(Verse { 
            content: "", 
        }),

        ("john", 19, 42) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 25) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 26) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 27) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 28) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 29) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 30) => Some(Verse { 
            content: "", 
        }),

        ("john", 20, 31) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 1) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 2) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 3) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 4) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 5) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 6) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 7) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 8) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 9) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 10) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 11) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 12) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 13) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 14) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 15) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 16) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 17) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 18) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 19) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 20) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 21) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 22) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 23) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 24) => Some(Verse { 
            content: "", 
        }),

        ("john", 21, 25) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF ACTS

        ("acts", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 42) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 43) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 44) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 45) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 46) => Some(Verse { 
            content: "", 
        }),

        ("acts", 2, 47) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 3, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 4, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 5, 42) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 42) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 43) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 44) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 45) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 46) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 47) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 48) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 49) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 50) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 51) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 52) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 53) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 54) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 55) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 56) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 57) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 58) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 59) => Some(Verse { 
            content: "", 
        }),

        ("acts", 7, 60) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 8, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 42) => Some(Verse { 
            content: "", 
        }),

        ("acts", 9, 43) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 42) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 43) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 44) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 45) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 46) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 47) => Some(Verse { 
            content: "", 
        }),

        ("acts", 10, 48) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 12, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 42) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 43) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 44) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 45) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 46) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 47) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 48) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 49) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 50) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 51) => Some(Verse { 
            content: "", 
        }),

        ("acts", 13, 52) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 14, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 15, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 16, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 17, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 18, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 19, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 20, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 21, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 22, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 23, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 24, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 25, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 26, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 31) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 32) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 33) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 34) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 35) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 36) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 37) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 38) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 39) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 40) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 41) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 42) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 43) => Some(Verse { 
            content: "", 
        }),

        ("acts", 27, 44) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 1) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 2) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 3) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 4) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 5) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 6) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 7) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 8) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 9) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 10) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 11) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 12) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 13) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 14) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 15) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 16) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 17) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 18) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 19) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 20) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 21) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 22) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 23) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 24) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 25) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 26) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 27) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 28) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 29) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 30) => Some(Verse { 
            content: "", 
        }),

        ("acts", 28, 31) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF ROMANS

        ("romans", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 28) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 29) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 30) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 31) => Some(Verse { 
            content: "", 
        }),

        ("romans", 1, 32) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 27) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 28) => Some(Verse { 
            content: "", 
        }),

        ("romans", 2, 29) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 1) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 4) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 7) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 10) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 13) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 16) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 19) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 22) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 25) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 27) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 28) => Some(Verse { 
            content: "", 
        }),
        
        ("romans", 3, 29) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 30) => Some(Verse { 
            content: "", 
        }),

        ("romans", 3, 31) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 6, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 27) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 28) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 29) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 30) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 31) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 32) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 33) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 34) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 35) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 36) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 37) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 38) => Some(Verse { 
            content: "", 
        }),

        ("romans", 8, 39) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 28) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 29) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 30) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 31) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 32) => Some(Verse { 
            content: "", 
        }),

        ("romans", 9, 33) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 31) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 32) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 33) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 34) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 35) => Some(Verse { 
            content: "", 
        }),

        ("romans", 11, 36) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 14, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 27) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 28) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 29) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 30) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 31) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 32) => Some(Verse { 
            content: "", 
        }),

        ("romans", 15, 33) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 20) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 21) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 22) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 23) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 24) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 25) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 26) => Some(Verse { 
            content: "", 
        }),

        ("romans", 16, 27) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF FIRST CORINTHIANS

        ("1 corinthians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 30) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 1, 31) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 30) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 31) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 32) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 33) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 34) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 35) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 36) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 37) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 38) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 39) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 7, 40) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 30) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 31) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 32) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 10, 33) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 31) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 32) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 33) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 11, 34) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 30) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 12, 31) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 30) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 31) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 32) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 33) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 34) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 35) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 36) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 37) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 38) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 39) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 14, 40) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 30) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 31) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 32) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 33) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 34) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 35) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 36) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 37) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 38) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 39) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 40) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 41) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 42) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 43) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 44) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 45) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 46) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 47) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 48) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 49) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 50) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 51) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 52) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 53) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 54) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 55) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 56) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 57) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 15, 58) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 corinthians", 16, 24) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF SECOND CORINTHIANS

        ("2 corinthians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 7) => Some(Verse { 
            content: "",
        }),

        ("2 corinthians", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 22) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 23) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 8, 24) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 31) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 32) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 11, 33) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 corinthians", 13, 14) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF GALATIANS

        ("galatians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 25) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 26) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 27) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 28) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 3, 29) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 26) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 27) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 28) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 29) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 30) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 4, 31) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("galatians", 6, 18) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF EPHESIANS

        ("ephesians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 23) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 24) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 25) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 26) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 27) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 28) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 29) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 30) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 31) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 4, 32) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 27) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 28) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 29) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 30) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 31) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 32) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 5, 33) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 21) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 22) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 23) => Some(Verse { 
            content: "", 
        }),

        ("ephesians", 6, 24) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF PHILIPPIANS

        ("philippians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 28) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 29) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 1, 30) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 27) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 28) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 29) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 2, 30) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 22) => Some(Verse { 
            content: "", 
        }),

        ("philippians", 4, 23) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF COLOSSIANS

        ("colossians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 28) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 1, 29) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 3, 25) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("colossians", 4, 18) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF FIRST THESSALONIANS

        ("1 thessalonians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 thessalonians", 5, 28) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF SECOND THESSALONIANS

        ("2 thessalonians", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 thessalonians", 3, 18) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF FIRST TIMOTHY

        ("1 timothy", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 5, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 timothy", 6, 21) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF SECOND TIMOTHY

        ("2 timothy", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 timothy", 4, 22) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF TITUS

        ("titus", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("titus", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("titus", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("titus", 3, 15) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF PHILEMON

        ("philemon", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("philemon", 1, 25) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF HEBREWS

        ("hebrews", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 19) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 6, 20) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 19) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 20) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 21) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 22) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 23) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 24) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 25) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 26) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 27) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 7, 28) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 22) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 23) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 24) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 25) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 26) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 27) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 9, 28) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 19) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 20) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 21) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 22) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 23) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 24) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 25) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 26) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 27) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 28) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 29) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 30) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 31) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 32) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 33) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 34) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 35) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 36) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 37) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 38) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 10, 39) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 20) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 21) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 22) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 23) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 24) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 25) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 26) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 27) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 28) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 29) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 30) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 31) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 32) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 33) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 34) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 35) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 36) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 37) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 38) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 39) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 11, 40) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 4) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 7) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 10) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 13) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 16) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 19) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 20) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 21) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 22) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 23) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 24) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 25) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 26) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 27) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 12, 28) => Some(Verse { 
            content: "", 
        }),
        
        ("hebrews", 12, 29) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 15) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 16) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 17) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 18) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 19) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 20) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 21) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 22) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 23) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 24) => Some(Verse { 
            content: "", 
        }),

        ("hebrews", 13, 25) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF JAMES
       
        ("james", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 26) => Some(Verse { 
            content: "", 
        }),

        ("james", 1, 27) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("james", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("james", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("james", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("james", 5, 20) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF FIRST PETER

        ("1 peter", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 1, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 peter", 5, 14) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF SECOND PETER

        ("2 peter", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("2 peter", 3, 18) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF FIRST JOHN

        ("1 john", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 27) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 28) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 2, 29) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 23) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 3, 24) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 4, 21) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 15) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 16) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 17) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 18) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 19) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 20) => Some(Verse { 
            content: "", 
        }),

        ("1 john", 5, 21) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF SECOND JOHN

        ("2 john", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("2 john", 1, 13) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF THIRD JOHN

        ("3 john", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("3 john", 1, 14) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF JUDE

        ("jude", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 21) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 22) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 23) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 24) => Some(Verse { 
            content: "", 
        }),

        ("jude", 1, 25) => Some(Verse { 
            content: "", 
        }),

        // THE BOOK OF REVELATION

        ("revelation", 1, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 1, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 21) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 22) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 23) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 24) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 25) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 26) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 27) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 28) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 2, 29) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 21) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 3, 22) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 4, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 5, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 6, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 7, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 8, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 9, 21) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 10, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 11, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 6) => Some(Verse { 
            content: "",
        }),

        ("revelation", 12, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 12, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 13, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 14, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 15, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 16, 21) => Some(Verse { 
            content: "", 
        }),
        
        ("revelation", 17, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 17, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 21) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 22) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 23) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 18, 24) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 19, 21) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 20, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 21) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 22) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 23) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 24) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 25) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 26) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 21, 27) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 1) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 2) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 3) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 4) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 5) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 6) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 7) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 8) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 9) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 10) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 11) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 12) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 13) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 14) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 15) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 16) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 17) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 18) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 19) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 20) => Some(Verse { 
            content: "", 
        }),

        ("revelation", 22, 21) => Some(Verse { 
            content: "", 
        }),

        _ => None,
    }
}