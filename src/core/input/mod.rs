pub mod text;

use std::io::{self, Write};

pub fn read_text() -> String {
    let mut input = String::new();

    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}