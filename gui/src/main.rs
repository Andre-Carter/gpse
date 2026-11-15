
use std::{env, fs, path::Path};

fn main() {
    let current = env::current_dir().unwrap();

    println!("Working directory: {:?}", current);

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("notes.md");

    println!("Reading file: {:?}", path);

    let contents = fs::read_to_string(&path);

    match contents {
        Ok(text) => println!("{text}"),
        Err(error) => println!("File error: {error}"),
    }
}
