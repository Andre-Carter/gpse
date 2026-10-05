use std::f64::consts::PI as RUST_PI;

pub const PI: f64 = RUST_PI;

use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

const DEBUG: bool = true;

//math> solve>
//math> debug>

//variants of enum Token
#[derive(Debug)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LeftParen,
    RightParen,
    Number(String),
    Identifier(String),
} 

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Power => write!(f, "^"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Number(value) => write!(f, "{value}"),
            Token::Identifier(name) => write!(f, "{name}"),
        }
    }
}

pub fn math_cli() {
    print!("math> ");

    let mut raw_input: String = String::new();

    read(&mut raw_input);

    let raw_input_chars: Vec<char> = raw_input
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();

    let math_array: Vec<&str> = raw_input.split_whitespace().collect();

    let tokens = math_tokenization(&raw_input_chars);

    math_debug(&raw_input_chars, &math_array, &tokens);
} 

pub fn math_tokenization(raw_input_chars: &[char]) -> Vec<Token> {
    let mut position = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while position < raw_input_chars.len() {
        match raw_input_chars.get(position) {
            Some('+') => tokens.push(Token::Plus),
            Some('-') => tokens.push(Token::Minus),
            Some('*') => tokens.push(Token::Multiply),
            Some('/') => tokens.push(Token::Divide),
            Some('^') => tokens.push(Token::Power),
            Some('(') => tokens.push(Token::LeftParen),
            Some(')') => tokens.push(Token::RightParen),
            _ => {}
        } 

        position +=1;
    }

    tokens
}

// parser

pub fn math_parser(token: Token) {
    match token {
        Token::Number(value) => {
            println!("Number: {value}");
        }

        Token::Plus => {
            println!("Plus operator");
        }

        Token::Identifier(name) => {
            println!("Identifier: {name}");
        }

        _ => {}
    }
}

pub fn math_debug(
    raw_input_chars: &[char], 
    math_array: &[&str],
    tokens: &[Token],
) {
    if DEBUG {
        print_math_chars(&raw_input_chars);
        print_math_array(&math_array);
        print_tokens(tokens);
    }
}

fn print_math_chars(raw_input_chars: &[char]) {
    println!("--- MATH CHARS ---");

    for (index, character) in raw_input_chars.iter().enumerate() {
        println!("[{index}] {character}");
    }

    println!("------------------");
}

fn print_math_array(math_array: &[&str]) {
    println!("--- MATH ARRAY ---");

    for (index, token) in math_array.iter().enumerate() {
        println!("[{}] {}", index, token);
    }

    println!("------------------");
}

fn print_tokens(tokens: &[Token]) {
    println!("--- TOKENS ---");

    for (index, token) in tokens.iter().enumerate() {
        println!("[{index}] [{token:?}] [{token}]");
    }

    println!("--------------");
}

