//use crate::mathematical::data_types::numeric::Numeric;

use std::io::{Write, stdin, stdout};

fn read(cli_input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(cli_input).expect("failed to read");
}

pub fn arithmetic_cli() {
    let mut cli_input = String::new();

    read(&mut cli_input);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithmeticError {
    InvalidInput,
    Underflow,
    Overflow,
    DivisionByZero,
    UnsupportedOperation,
}

// Addition

// Platform-sized unsigned interger
pub fn add_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if left + right < usize::MIN {
        return Err(ArithmeticError::Underflow);
    }

    if left + right > usize::MAX {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left + right)
}

// Unsigned integers
pub fn add_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    Ok(left + right)
}

pub fn add_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    Ok(left + right)
}

// Signed integers
pub fn add_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    Ok(left + right)
}

pub fn add_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    Ok(left + right)
}

// Floating-point
pub fn add_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    Ok(left + right)
}

pub fn add_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    Ok(left + right)
}

// Subtraction

// Platform-sized unsigned interger
pub fn sub_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    Ok(left - right)
}

// Unsigned integers
pub fn sub_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    Ok(left - right)
}

pub fn sub_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    Ok(left - right)
}

// Signed integers
pub fn sub_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    Ok(left - right)
}

pub fn sub_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    Ok(left - right)
}

// Floating-point
pub fn sub_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    Ok(left - right)
}

pub fn sub_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    Ok(left - right)
}

// Multiplication

// Platform-sized unsigned interger
pub fn mul_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    Ok(left * right)
}

// Unsigned integers
pub fn mul_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    Ok(left * right)
}

pub fn mul_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    Ok(left * right)
}

// Signed integers
pub fn mul_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    Ok(left * right)
}

pub fn mul_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    Ok(left * right)
}

// Floating-point
pub fn mul_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    Ok(left * right)
}

pub fn mul_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    Ok(left * right)
}

// Division

// Platform-sized unsigned interger
pub fn div_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

// Unsigned integers
pub fn div_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

pub fn div_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

// Signed integers
pub fn div_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

pub fn div_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

// Floating-point
pub fn div_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

pub fn div_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

// Remainder (Modulo)

// Platform-sized unsigned interger
pub fn rem_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    Ok(left % right)
}

// Unsigned integers
pub fn rem_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    Ok(left % right)
}

pub fn rem_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    Ok(left % right)
}

// Signed integers
pub fn rem_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    Ok(left % right)
}

pub fn rem_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    Ok(left % right)
}

// Floating-point
pub fn rem_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    Ok(left % right)
}

pub fn rem_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    Ok(left % right)
}

// Power (exponent)

// Platform-sized unsigned interger
pub fn pow_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    Ok(left ^ right)
}

// Unsigned integers
pub fn pow_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    Ok(left ^ right)
}

pub fn pow_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    Ok(left ^ right)
}

// Signed integers
pub fn pow_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    Ok(left ^ right)
}

pub fn pow_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    Ok(left ^ right)
}

// No rust-implentation for (bitwise XOR) floating-point ^ (f32 ^ f32 & f64 ^ f64)
// Check XDoc: references> reference> rust> arithmetic

pub fn pow_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    Ok(left.powf(right))
}

pub fn pow_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    Ok(left.powf(right))
}

// test

#[cfg(test)]
mod tests {
    use super::*;

    // Test addition
    #[test]
    fn test_add_usize() {
        asserteq!(add_usize(10, 11), 21)
    }

    #[test]
    fn test_add_u32() {
        asserteq!(add_u32(10, 11), 21)
    }

    #[test]
    fn test_add_u64() {
        asserteq!(add_u64(10, 11), 21)
    }

    #[test]
    fn test_add_i32() {
        asserteq!(add_i32(10, 11), 21)
    }

    #[test]
    fn test_add_i64() {
        asserteq!(add_i64(10, 11), 21)
    }

    #[test]
    fn test_add_f32() {
        asserteq!(add_f32(10, 11), 21)
    }

    #[test]
    fn test_add_f64() {
        asserteq!(add_f64(10, 11), 21)
    }

    // Test subtraction
    #[test]
    fn test_sub_usize() {
        asserteq!(sub_usize(10, 11), 21)
    }

    #[test]
    fn test_sub_u32() {
        asserteq!(sub_u32(10, 11), 21)
    }

    #[test]
    fn test_sub_u64() {
        asserteq!(sub_u64(10, 11), 21)
    }

    #[test]
    fn test_sub_i32() {
        asserteq!(sub_i32(10, 11), 21)
    }

    #[test]
    fn test_sub_i64() {
        asserteq!(sub_i64(10, 11), 21)
    }

    #[test]
    fn test_sub_f32() {
        asserteq!(sub_f32(10, 11), 21)
    }

    #[test]
    fn test_sub_f64() {
        asserteq!(sub_f64(10, 11), 21)
    }

    // Test multiplication
    #[test]
    fn test_mul_usize() {
        asserteq!(mul_usize(10, 11), 21)
    }

    #[test]
    fn test_mul_u32() {
        asserteq!(mul_u32(10, 11), 21)
    }

    #[test]
    fn test_mul_u64() {
        asserteq!(mul_u64(10, 11), 21)
    }

    #[test]
    fn test_mul_i32() {
        asserteq!(mul_i32(10, 11), 21)
    }

    #[test]
    fn test_mul_i64() {
        asserteq!(mul_i64(10, 11), 21)
    }

    #[test]
    fn test_mul_f32() {
        asserteq!(mul_f32(10, 11), 21)
    }

    #[test]
    fn test_mul_f64() {
        asserteq!(mul_f64(10, 11), 21)
    }

    // Test division
    #[test]
    fn test_div_usize() {
        asserteq!(div_usize(10, 11), 21)
    }

    #[test]
    fn test_div_u32() {
        asserteq!(div_u32(10, 11), 21)
    }

    #[test]
    fn test_div_u64() {
        asserteq!(div_u64(10, 11), 21)
    }

    #[test]
    fn test_div_i32() {
        asserteq!(div_i32(10, 11), 21)
    }

    #[test]
    fn test_div_i64() {
        asserteq!(div_i64(10, 11), 21)
    }

    #[test]
    fn test_div_f32() {
        asserteq!(div_f32(10, 11), 21)
    }

    #[test]
    fn test_div_f64() {
        asserteq!(div_f64(10, 11), 21)
    }

    // Test remainder
    #[test]
    fn test_rem_usize() {
        asserteq!(rem_usize(10, 11), 21)
    }

    #[test]
    fn test_rem_u32() {
        asserteq!(rem_u32(10, 11), 21)
    }

    #[test]
    fn test_rem_u64() {
        asserteq!(rem_u64(10, 11), 21)
    }

    #[test]
    fn test_rem_i32() {
        asserteq!(rem_i32(10, 11), 21)
    }

    #[test]
    fn test_rem_i64() {
        asserteq!(rem_i64(10, 11), 21)
    }

    #[test]
    fn test_rem_f32() {
        asserteq!(rem_f32(10, 11), 21)
    }

    #[test]
    fn test_rem_f64() {
        asserteq!(rem_f64(10, 11), 21)
    }

    // Test power
    #[test]
    fn test_pow_usize() {
        asserteq!(pow_usize(10, 11), 21)
    }

    #[test]
    fn test_pow_u32() {
        asserteq!(pow_u32(10, 11), 21)
    }

    #[test]
    fn test_pow_u64() {
        asserteq!(pow_u64(10, 11), 21)
    }

    #[test]
    fn test_pow_i32() {
        asserteq!(pow_i32(10, 11), 21)
    }

    #[test]
    fn test_pow_i64() {
        asserteq!(pow_i64(10, 11), 21)
    }

    #[test]
    fn test_pow_f32() {
        asserteq!(pow_f32(10, 11), 21)
    }

    #[test]
    fn test_pow_f64() {
        asserteq!(pow_f64(10, 11), 21)
    }
}

// debug?

/*
pub fn arithmetic_debug() {
    add_usize();
    add_u32();
    add_u64();
    add_i32();
    add_i64();
    add_f32();
    add_f64();

    sub_usize();
    sub_u32();
    sub_u64();
    sub_i32();
    sub_i64();
    sub_f32();
    sub_f64();

    mul_usize();
    mul_u32();
    mul_u64();
    mul_i32();
    mul_i64();
    mul_f32();
    mul_f64();

    div_usize();
    div_u32();
    div_u64();
    div_i32();
    div_i64();
    div_f32();
    div_f64();

    rem_usize();
    rem_u32();
    rem_u64();
    rem_i32();
    rem_i64();
    rem_f32();
    rem_f64();

    pow_usize();
    pow_u32();
    pow_u64();
    pow_i32();
    pow_i64();
    pow_f32();
    pow_f64();
}
*/

// Checked add

/*

pub fn add(
    left: ,
    right: ,
) -> Result<, ArithmeticError> {

}

pub fn checked_add_i32(
    left: i32,
    right: i32,
) -> Result<i32, ArithmeticError> {
    left.checked_add(right)
        .ok_or(ArithmeticError::Overflow)
}

*/

//example code, temp., refactor
/*
pub fn add(
    left: NumericType,
    right: NumericType,
) -> Result<NumericType, ArithmeticError> {
    match (left, right) {
        (NumericType::Usize(left), NumericType::Usize(right)) => {
            left.checked_add(right)
                .map(NumericType::Usize)
                .ok_or(ArithmeticError::Overflow)
        }

        _ => Err(ArithmeticError::InvalidDataType),
    }
}
*/
