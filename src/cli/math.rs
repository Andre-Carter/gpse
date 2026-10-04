//use std::f32::consts::PI;
use std::f64::consts::PI as RUST_PI;

pub const PI: f64 = RUST_PI;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

//tokenizer .. what are there symbol?
// parser (expression tree) what do these symbols mean together?
//expression
// evaluator

const DEBUG: bool = true;

pub fn math_cli() {
    print!("math> ");

    let mut raw_input: String = String::new();

    read(&mut raw_input);

    let math_array: Vec<&str> = raw_input.split_whitespace().collect();

    let _first_token = math_array.first();

    let _last_token = math_array.last();

    let _number_of_tokens = math_array.len();

    if DEBUG {
        print_math_array(&math_array);
    };
}

fn print_math_array(math_array: &[&str]) {
    println!("--- MATH ARRAY ---");

    for (index, token) in math_array.iter().enumerate() {
        println!("[{}] {}", index, token);
    }

    println!("------------------");
}

