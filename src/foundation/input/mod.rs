pub mod text;

use std::io::{self, Write};

const _GREEN: &str = "\x1b[32m";
const _RESET: &str = "\x1b[0m";

pub fn read_text() -> String {
    let mut input = String::new();

    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
