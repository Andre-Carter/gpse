use std::io::{Write, stdin, stdout};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

pub fn arithmetic_cli() {
    let mut input = String::new();

    read(&mut input);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticError {
    Overflow,
    DivisionByZero,
}

pub fn add_i32(left: i32, right: i32) -> i32 {
    left + right
} 

pub fn checked_add(

) {

}

pub fn checked_add_i32(
    left: i32,
    right: i32,
) -> Result<i32, ArithmeticError> {
    left.checked_add(right)
        .ok_or(ArithmeticError::Overflow)
}
