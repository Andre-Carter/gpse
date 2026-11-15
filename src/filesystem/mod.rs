use std::{env, fs, path::Path};

pub fn get_current_directory() {
    env::current_dir().unwrap();
}

pub fn print_current_directory() {
    let print = get_current_directory();
    println!("Working directory: {:?}", print);
}