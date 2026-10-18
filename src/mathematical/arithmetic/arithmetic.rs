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
    Overflow,
    Underflow,
    DivisionByZero,
    UnsupportedOperation,
    InvalidExponent,
}

// NOTE
// add(numeric_data_type: u8, left: 5, right: 6);
//1. identify numeric data type

// END NOTE

// Addition, Subtraction, Multiplication, Division, Remainder, Power
// u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64

// Addition

pub fn add_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right > u8::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left + right)
}

pub fn add_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right > u16::MAX - left {
        return Err(ArithmeticError::Overflow);
    }
 
    Ok(left + right)
}

pub fn add_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right > u32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left + right)
}

pub fn add_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right > u64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left + right)
}

pub fn add_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right > u128::MAX - left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left + right)
}

pub fn add_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right > usize::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left + right)
}

pub fn add_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right > i8::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < i8::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

pub fn add_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right > i16::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < i16::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

pub fn add_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right > i32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < i32::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

pub fn add_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right > i64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < i64::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

pub fn add_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right > i128::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < i128::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

pub fn add_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right > isize::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < isize::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

pub fn add_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right > f32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < f32::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

pub fn add_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right > f64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < f64::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left + right)
}

// Subtraction 

pub fn sub_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right > u8::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right > left {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right > u16::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right > left {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right > u32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right > left {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right > u64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right > left {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right > u128::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right > left {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right > usize::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right > left {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right > i8::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i8::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right > i16::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i16::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right > i32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i32::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right > i64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i64::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right > i128::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i128::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right > isize::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < isize::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right > f32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < f32::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

pub fn sub_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right > f64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if left < f64::MIN + right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left - right)
}

// Multiplication

pub fn mul_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right > u8::MAX / left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left * right)
}

pub fn mul_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right > u16::MAX / left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left * right)
}

pub fn mul_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right > u32::MAX / left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left * right)
}

pub fn mul_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right > u64::MAX / left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left * right)
}

pub fn mul_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right > u128::MAX / left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left * right)
}

pub fn mul_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right > usize::MAX / left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left * right)
}

pub fn mul_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right > i8::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i8::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

pub fn mul_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right > i16::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i16::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

pub fn mul_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right > i32::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i32::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

pub fn mul_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right > i64::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if right < i64::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

pub fn mul_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right > i128::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i128::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

pub fn mul_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right > isize::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < isize::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

pub fn mul_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right > f32::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < f32::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

pub fn mul_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right > f64::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < f64::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left * right)
}

// Division

pub fn div_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / u8::MAX {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left / right)
}

pub fn div_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / u16::MAX {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left / right)
}

pub fn div_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / u32::MAX {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left / right)
}

pub fn div_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / u64::MAX {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left / right)
}

pub fn div_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / u128::MAX {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left / right)
}

pub fn div_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / usize::MAX {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left / right)
}

pub fn div_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / i8::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / i8::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

pub fn div_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / i16::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / i16::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

pub fn div_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / i32::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / i32::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

pub fn div_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / i64::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / i64::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

pub fn div_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / i128::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / i128::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

pub fn div_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / isize::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / isize::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

pub fn div_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / f32::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / f32::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

pub fn div_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    if right > left / f64::MAX {
        return Err(ArithmeticError::Overflow);
    }

    if left < right / f64::MIN {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left / right)
}

// Remainder

pub fn rem_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > u8::MAX % left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left % right)
}

pub fn rem_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > u16::MAX % left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left % right)
}

pub fn rem_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > u32::MAX % left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left % right)
}

pub fn rem_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > u64::MAX % left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left % right)
}

pub fn rem_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > u128::MAX % left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left % right)
}

pub fn rem_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > usize::MAX % left {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left % right)
}

pub fn rem_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > i8::MAX % left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i8::MIN % right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left % right)
}

pub fn rem_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > i16::MAX % left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i16::MIN % right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left % right)
}

pub fn rem_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > i32::MAX % left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i32::MIN % right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left % right)
}

pub fn rem_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > i64::MAX % left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i64::MIN % right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left % right)
}

pub fn rem_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > i128::MAX % left {
        return Err(ArithmeticError::Overflow);
    }

    if left < i128::MIN % right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left % right)
}

pub fn rem_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > isize::MAX % left {
        return Err(ArithmeticError::Overflow);
    }

    if left < isize::MIN % right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left % right)
}

pub fn rem_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > f32::MAX / left {
        return Err(ArithmeticError::Overflow);
    }

    if left < f32::MIN / right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left % right)
}

pub fn rem_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }
    
    if right > f64::MAX % left {
        return Err(ArithmeticError::Overflow);
    }

    if left < f64::MIN % right {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left % right)
}

// Power (exponent)

pub fn pow_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right > 8_u8 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right > 16_u16 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right > 32_u32 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right > 64_u64 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right > 128_u128 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right > usize::MAX {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right > 8_i8 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right > 16_i16 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right > 32_i32 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if left ^ right > 64_i64 {
        return Err(ArithmeticError::Overflow);
    }

    Ok(left ^ right)
}

pub fn pow_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right > 128_i128 {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

pub fn pow_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right > isize::MAX {
        return Err(ArithmeticError::Overflow);
    }
    
    Ok(left ^ right)
}

// No rust-implentation for (bitwise XOR) floating-point ^ (f32 ^ f32 & f64 ^ f64)
// Check XDoc: references> reference> rust> arithmetic

pub fn pow_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right > 32.0_f32 {
        return Err(ArithmeticError::Overflow);
    }

    if left < -32.0_f32 {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left.powf(right))
}

pub fn pow_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right > 64_f64 {
        return Err(ArithmeticError::Overflow);
    }

    if left < f64::MIN {
        return Err(ArithmeticError::Underflow);
    }
    
    Ok(left.powf(right))
}

// test

#[cfg(test)]
mod tests {
    use super::*;

    // test addition

    #[test]
    fn test_add_u8() {
        assert_eq!(add_u8(1, 2), Ok(3))
    }

    #[test]
    fn test_add_u8_max() {
        assert_eq!(add_u8(0, u8::MAX), Ok(u8::MAX))
    }

    #[test]
    fn test_add_u8_overflow() {
        assert_eq!(add_u8(1, u8::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_u16() {
        assert_eq!(add_u16(1, 2), Ok(3))
    }

    #[test]
    fn test_add_u16_max() {
        assert_eq!(add_u16(0, u16::MAX), Ok(u16::MAX))
    }

    #[test]
    fn test_add_u16_overflow() {
        assert_eq!(add_u16(1, u16::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_u32() {
        assert_eq!(add_u32(0, 11), Ok(21))
    }

    #[test]
    fn test_add_u64() {
        assert_eq!(add_u64(10, 11), Ok(21))
    }

    #[test]
    fn test_add_u128() {
        assert_eq!(add_u128(10, 11), Ok(21))
    }

    #[test]
    fn test_add_usize() {
        assert_eq!(add_usize(10, 11), Ok(21))
    }

    #[test]
    fn test_add_i8() {
        assert_eq!(add_i8(10, 11), Ok(21))
    }

    #[test]
    fn test_add_i16() {
        assert_eq!(add_i16(10, 11), Ok(21))
    }

    #[test]
    fn test_add_i32() {
        assert_eq!(add_i32(10, 11), Ok(21))
    }

    #[test]
    fn test_add_i64() {
        assert_eq!(add_i64(10, 11), Ok(21))
    }

    #[test]
    fn test_add_i128() {
        assert_eq!(add_i128(10, 11), Ok(21))
    }

    #[test]
    fn test_add_isize() {
        assert_eq!(add_isize(10, 11), Ok(21))
    }

    #[test]
    fn test_add_f32() {
        assert_eq!(add_f32(10.0, 11.0), Ok(21.0))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(add_f64(10.0, 11.0), Ok(21.0))
    }

    // Test subtraction

    #[test]
    fn test_sub_u8() {
        assert_eq!(sub_u8(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_u16() {
        assert_eq!(sub_u16(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_u32() {
        assert_eq!(sub_u32(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_u64() {
        assert_eq!(sub_u64(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_u128() {
        assert_eq!(sub_u128(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_usize() {
        assert_eq!(sub_usize(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_i8() {
        assert_eq!(sub_i8(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_i16() {
        assert_eq!(sub_i16(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_i32() {
        assert_eq!(sub_i32(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_i64() {
        assert_eq!(sub_i64(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_i128() {
        assert_eq!(sub_i128(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_isize() {
        assert_eq!(sub_isize(10, 11), Ok(21))
    }

    #[test]
    fn test_sub_f32() {
        assert_eq!(sub_f32(10.0, 11.0), Ok(21.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(sub_f64(10.0, 11.0), Ok(21.0))
    }

    // TEST MULTIPICATION

    #[test]
    fn test_mul_u8() {
        assert_eq!(mul_u8(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_u16() {
        assert_eq!(mul_u16(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_u32() {
        assert_eq!(mul_u32(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_u64() {
        assert_eq!(mul_u64(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_u128() {
        assert_eq!(mul_u128(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_usize() {
        assert_eq!(mul_usize(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_i8() {
        assert_eq!(mul_i8(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_i16() {
        assert_eq!(mul_i16(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_i32() {
        assert_eq!(mul_i32(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_i64() {
        assert_eq!(mul_i64(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_i128() {
        assert_eq!(mul_i128(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_isize() {
        assert_eq!(mul_isize(10, 11), Ok(21))
    }

    #[test]
    fn test_mul_f32() {
        assert_eq!(mul_f32(10.0, 11.0), Ok(21.0))
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(mul_f64(10.0, 11.0), Ok(21.0))
    }

    // TEST DIVISION

    #[test]
    fn test_div_u8() {
        assert_eq!(div_u8(10, 11), Ok(21))
    }

    #[test]
    fn test_div_u16() {
        assert_eq!(div_u16(10, 11), Ok(21))
    }

    #[test]
    fn test_div_u32() {
        assert_eq!(div_u32(10, 11), Ok(21))
    }

    #[test]
    fn test_div_u64() {
        assert_eq!(div_u64(10, 11), Ok(21))
    }

    #[test]
    fn test_div_u128() {
        assert_eq!(div_u128(10, 11), Ok(21))
    }

    #[test]
    fn test_div_usize() {
        assert_eq!(div_usize(10, 11), Ok(21))
    }

    #[test]
    fn test_div_i8() {
        assert_eq!(div_i8(10, 11), Ok(21))
    }

    #[test]
    fn test_div_i16() {
        assert_eq!(div_i16(10, 11), Ok(21))
    }

    #[test]
    fn test_div_i32() {
        assert_eq!(div_i32(10, 11), Ok(21))
    }

    #[test]
    fn test_div_i64() {
        assert_eq!(div_i64(10, 11), Ok(21))
    }

    #[test]
    fn test_div_i128() {
        assert_eq!(div_i128(10, 11), Ok(21))
    }

    #[test]
    fn test_div_isize() {
        assert_eq!(div_isize(10, 11), Ok(21))
    }

    #[test]
    fn test_div_f32() {
        assert_eq!(div_f32(10.0, 11.0), Ok(21.0))
    }

    #[test]
    fn test_div_f64() {
        assert_eq!(div_f64(10.0, 11.0), Ok(21.0))
    }

    // TEST REMAINDER

    #[test]
    fn test_rem_u8() {
        assert_eq!(rem_u8(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_u16() {
        assert_eq!(rem_u16(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_u32() {
        assert_eq!(rem_u32(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_u64() {
        assert_eq!(rem_u64(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_u128() {
        assert_eq!(rem_u128(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_usize() {
        assert_eq!(rem_usize(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_i8() {
        assert_eq!(rem_i8(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_i16() {
        assert_eq!(rem_i16(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_i32() {
        assert_eq!(rem_i32(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_i64() {
        assert_eq!(rem_i64(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_i128() {
        assert_eq!(rem_i128(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_isize() {
        assert_eq!(rem_isize(10, 11), Ok(21))
    }

    #[test]
    fn test_rem_f32() {
        assert_eq!(rem_f32(10.0, 11.0), Ok(21.0))
    }

    #[test]
    fn test_rem_f64() {
        assert_eq!(rem_f64(10.0, 11.0), Ok(21.0))
    }

    // TEST POWER

    #[test]
    fn test_pow_u8() {
        assert_eq!(pow_u8(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_u16() {
        assert_eq!(pow_u16(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_u32() {
        assert_eq!(pow_u32(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_u64() {
        assert_eq!(pow_u64(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_u128() {
        assert_eq!(pow_u128(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_usize() {
        assert_eq!(pow_usize(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_i8() {
        assert_eq!(pow_i8(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_i16() {
        assert_eq!(pow_i16(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_i32() {
        assert_eq!(pow_i32(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_i64() {
        assert_eq!(pow_i64(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_i128() {
        assert_eq!(pow_i128(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_isize() {
        assert_eq!(pow_isize(10, 11), Ok(21))
    }

    #[test]
    fn test_pow_f32() {
        assert_eq!(pow_f32(10.0, 11.0), Ok(21.0))
    }

    #[test]
    fn test_pow_f64() {
        assert_eq!(pow_f64(10.0, 11.0), Ok(21.0))
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
