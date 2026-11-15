use std::{env, fs};

fn main() {
    let current = env::current_dir().unwrap();

    println!("Working directory: {:?}", current);

    let contents = fs::read_to_string("notes.md");

    match contents {
        Ok(text) => println!("{text}"),
        Err(error) => println!("File error: {error}"),
    }
}