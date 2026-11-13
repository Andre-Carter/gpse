pub fn alpha_index(characters: char) -> i32 {
    match characters {
        'a' => 1,
        '\0'..='`' | 'b'..='\u{d7ff}' | '\u{e000}'..='\u{10ffff}' => todo!(),
    }
}

pub fn alphaenumerate(part: &str) {
    let _characters: Vec<_> = part.chars().collect();
}

pub fn alphabetically_order<T>(_vec: Vec<T>) {}

pub fn split_by() {}

pub fn vector<T>() {
    let _vector: Vec<T> = Vec::new();
}

pub fn test_vector<T>() {
    vector::<T>();
}
