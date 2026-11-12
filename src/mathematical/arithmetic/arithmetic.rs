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
    if right > 0 && left > i8::MAX - right {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0 && left < i8::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

pub fn add_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right > 0 && left > i16::MAX - right {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0 && left < i16::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

pub fn add_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right > 0 && left > i32::MAX - right {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0 && left < i32::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

pub fn add_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right > 0 && left > i64::MAX - right {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0 && left < i64::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

pub fn add_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right > 0 && left > i128::MAX - right {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0 && left < i128::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

pub fn add_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right > 0 && left > isize::MAX - right {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0 && left < isize::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

pub fn add_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right > f32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0.0 && left < f32::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

pub fn add_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right > f64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0.0 && left < f64::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left + right)
}

// SUBTRACTION

pub fn sub_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right > left {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right > left {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right > left {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right > left {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right > left {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right > left {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right < 0 && left > i8::MAX + right {
        return Err(ArithmeticError::Overflow);
    }
    if right > 0 && left < i8::MIN + right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right < 0 && left > i16::MAX + right {
        return Err(ArithmeticError::Overflow);
    }
    if right > 0 && left < i16::MIN + right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right < 0 && left > i32::MAX + right {
        return Err(ArithmeticError::Overflow);
    }

    if right > 0 && left < i32::MIN + right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right < 0 && left > i64::MAX + right {
        return Err(ArithmeticError::Overflow);
    }
    if right > 0 && left < i64::MIN + right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right < 0 && left > i128::MAX + right {
        return Err(ArithmeticError::Overflow);
    }
    if right > 0 && left < i128::MIN + right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right < 0 && left > isize::MAX + right {
        return Err(ArithmeticError::Overflow);
    }

    if right > 0 && left < isize::MIN + right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right > f32::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0.0 && left < f32::MIN - right {
        return Err(ArithmeticError::Underflow);
    }

    Ok(left - right)
}

pub fn sub_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right > f64::MAX - left {
        return Err(ArithmeticError::Overflow);
    }

    if right < 0.0 && left < f64::MIN - right {
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

    if right < i64::MIN / left {
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

    Ok(left / right)
}

pub fn div_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

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

pub fn div_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

pub fn div_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left / right)
}

pub fn div_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_div(right).ok_or(ArithmeticError::Overflow)
}

pub fn div_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_div(right).ok_or(ArithmeticError::Overflow)
}

pub fn div_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_div(right).ok_or(ArithmeticError::Overflow)
}

pub fn div_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_div(right).ok_or(ArithmeticError::Overflow)
}

pub fn div_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_div(right).ok_or(ArithmeticError::Overflow)
}

pub fn div_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_div(right).ok_or(ArithmeticError::Overflow)
}

pub fn div_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    let result = left / right;

    if result.is_infinite() {
        return Err(if result > 0.0 {
            ArithmeticError::Overflow
        } else {
            ArithmeticError::Underflow
        });
    }

    Ok(result)
}

pub fn div_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    let result = left / right;

    if result.is_infinite() {
        return Err(if result > 0.0 {
            ArithmeticError::Overflow
        } else {
            ArithmeticError::Underflow
        });
    }

    Ok(result)
}

// Remainder

pub fn rem_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

pub fn rem_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

pub fn rem_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

pub fn rem_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

pub fn rem_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

pub fn rem_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

pub fn rem_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_rem(right).ok_or(ArithmeticError::Overflow)
}

pub fn rem_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_rem(right).ok_or(ArithmeticError::Overflow)
}

pub fn rem_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_rem(right).ok_or(ArithmeticError::Overflow)
}

pub fn rem_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_rem(right).ok_or(ArithmeticError::Overflow)
}

pub fn rem_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_rem(right).ok_or(ArithmeticError::Overflow)
}

pub fn rem_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    if right == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    left.checked_rem(right).ok_or(ArithmeticError::Overflow)
}

pub fn rem_f32(left: f32, right: f32) -> Result<f32, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

pub fn rem_f64(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    if right == 0.0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    Ok(left % right)
}

// Power (exponent)

pub fn pow_u8(left: u8, right: u8) -> Result<u8, ArithmeticError> {
    left.checked_pow(u32::from(right))
        .ok_or(ArithmeticError::Overflow)
}

pub fn pow_u16(left: u16, right: u16) -> Result<u16, ArithmeticError> {
    left.checked_pow(u32::from(right))
        .ok_or(ArithmeticError::Overflow)
}

pub fn pow_u32(left: u32, right: u32) -> Result<u32, ArithmeticError> {
    left.checked_pow(u32::from(right))
        .ok_or(ArithmeticError::Overflow)
}

pub fn pow_u64(left: u64, right: u64) -> Result<u64, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(ArithmeticError::Overflow)
}

pub fn pow_u128(left: u128, right: u128) -> Result<u128, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(ArithmeticError::Overflow)
}

pub fn pow_usize(left: usize, right: usize) -> Result<usize, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(ArithmeticError::Overflow)
}

pub fn pow_i8(left: i8, right: i8) -> Result<i8, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(if left < 0 && exp % 2 == 1 {
        ArithmeticError::Underflow
    } else {
        ArithmeticError::Overflow
    })
}

pub fn pow_i16(left: i16, right: i16) -> Result<i16, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(if left < 0 && exp % 2 == 1 {
        ArithmeticError::Underflow
    } else {
        ArithmeticError::Overflow
    })
}

pub fn pow_i32(left: i32, right: i32) -> Result<i32, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(if left < 0 && exp % 2 == 1 {
        ArithmeticError::Underflow
    } else {
        ArithmeticError::Overflow
    })
}

pub fn pow_i64(left: i64, right: i64) -> Result<i64, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(if left < 0 && exp % 2 == 1 {
        ArithmeticError::Underflow
    } else {
        ArithmeticError::Overflow
    })
}

pub fn pow_i128(left: i128, right: i128) -> Result<i128, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(if left < 0 && exp % 2 == 1 {
        ArithmeticError::Underflow
    } else {
        ArithmeticError::Overflow
    })
}

pub fn pow_isize(left: isize, right: isize) -> Result<isize, ArithmeticError> {
    let exp = u32::try_from(right).map_err(|_| ArithmeticError::InvalidExponent)?;
    left.checked_pow(exp).ok_or(if left < 0 && exp % 2 == 1 {
        ArithmeticError::Underflow
    } else {
        ArithmeticError::Overflow
    })
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

// TEST

#[cfg(test)]
mod tests {
    use super::*;

    // TEST ADDITION: ADD, UPPER, OVERFLOW

    #[test]
    fn test_add_u8() {
        assert_eq!(add_u8(1, 2), Ok(3))
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
    fn test_add_u16_overflow() {
        assert_eq!(add_u16(1, u16::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_u32() {
        assert_eq!(add_u32(1, 2), Ok(3))
    }

    #[test]
    fn test_add_u32_overflow() {
        assert_eq!(add_u32(1, u32::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_u64() {
        assert_eq!(add_u64(1, 2), Ok(3))
    }

    #[test]
    fn test_add_u64_overflow() {
        assert_eq!(add_u64(1, u64::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_u128() {
        assert_eq!(add_u128(1, 2), Ok(3))
    }

    #[test]
    fn test_add_u128_overflow() {
        assert_eq!(add_u128(1, u128::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_usize() {
        assert_eq!(add_usize(1, 2), Ok(3))
    }

    #[test]
    fn test_add_usize_overflow() {
        assert_eq!(add_usize(1, usize::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_i8() {
        assert_eq!(add_i8(1, 2), Ok(3))
    }

    #[test]
    fn test_add_i8_overflow() {
        assert_eq!(add_i8(1, i8::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_i16() {
        assert_eq!(add_i16(1, 2), Ok(3))
    }

    #[test]
    fn test_add_i16_overflow() {
        assert_eq!(add_i16(1, i16::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_i32() {
        assert_eq!(add_i32(1, 2), Ok(3))
    }

    #[test]
    fn test_add_i32_overflow() {
        assert_eq!(add_i32(1, i32::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_i64() {
        assert_eq!(add_i64(1, 2), Ok(3))
    }

    #[test]
    fn test_add_i64_overflow() {
        assert_eq!(add_i64(1, i64::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_i128() {
        assert_eq!(add_i128(1, 2), Ok(3))
    }

    #[test]
    fn test_add_i128_overflow() {
        assert_eq!(add_i128(1, i128::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_isize() {
        assert_eq!(add_isize(1, 2), Ok(3))
    }

    #[test]
    fn test_add_isize_overflow() {
        assert_eq!(add_isize(1, isize::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_f32() {
        assert_eq!(add_f32(1.0, 2.0), Ok(3.0))
    }

    #[test]
    fn test_add_f32_overflow() {
        assert_eq!(add_f32(f32::MAX, f32::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(add_f64(1.0, 2.0), Ok(3.0))
    }

    #[test]
    fn test_add_f64_overflow() {
        assert_eq!(add_f64(f64::MAX, f64::MAX), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_sub_u8() {
        assert_eq!(sub_u8(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_u8_underflow() {
        assert_eq!(sub_u8(0, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_u16() {
        assert_eq!(sub_u16(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_u16_underflow() {
        assert_eq!(sub_u16(0, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_u32() {
        assert_eq!(sub_u32(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_u32_underflow() {
        assert_eq!(sub_u32(0, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_u64() {
        assert_eq!(sub_u64(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_u64_underflow() {
        assert_eq!(sub_u64(0, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_u128() {
        assert_eq!(sub_u128(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_u128_underflow() {
        assert_eq!(sub_u128(0, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_usize() {
        assert_eq!(sub_usize(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_usize_underflow() {
        assert_eq!(sub_usize(0, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_i8() {
        assert_eq!(sub_i8(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_i8_underflow() {
        assert_eq!(sub_i8(i8::MIN, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_i16() {
        assert_eq!(sub_i16(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_i16_underflow() {
        assert_eq!(sub_i16(i16::MIN, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_i32() {
        assert_eq!(sub_i32(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_i32_underflow() {
        assert_eq!(sub_i32(i32::MIN, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_i64() {
        assert_eq!(sub_i64(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_i64_underflow() {
        assert_eq!(sub_i64(i64::MIN, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_i128() {
        assert_eq!(sub_i128(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_i128_underflow() {
        assert_eq!(sub_i128(i128::MIN, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_isize() {
        assert_eq!(sub_isize(3, 2), Ok(1))
    }

    #[test]
    fn test_sub_isize_underflow() {
        assert_eq!(sub_isize(isize::MIN, 1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_sub_f32() {
        assert_eq!(sub_f32(3.0, 2.0), Ok(1.0))
    }

    #[test]
    fn test_sub_f32_overflow() {
        assert_eq!(sub_f32(f32::MIN, 1.0), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_sub_f32_max() {
        assert_eq!(sub_f32(f32::MAX, f32::MAX), Ok(0.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(sub_f64(3.0, 2.0), Ok(1.0))
    }

    #[test]
    fn test_sub_f64_overflow() {
        assert_eq!(sub_f64(f64::MIN, 1.0), Err(ArithmeticError::Overflow))
    }

    // TEST MULTIPICATION

    #[test]
    fn test_mul_u8() {
        assert_eq!(mul_u8(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_u8_overflow() {
        assert_eq!(mul_u8(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_u16() {
        assert_eq!(mul_u16(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_u16_overflow() {
        assert_eq!(mul_u16(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_u32() {
        assert_eq!(mul_u32(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_u32_overflow() {
        assert_eq!(mul_u32(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_u64() {
        assert_eq!(mul_u64(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_u64_overflow() {
        assert_eq!(mul_u64(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_u128() {
        assert_eq!(mul_u128(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_u128_overflow() {
        assert_eq!(mul_u128(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_usize() {
        assert_eq!(mul_usize(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_usize_overflow() {
        assert_eq!(mul_usize(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_i8() {
        assert_eq!(mul_i8(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_i8_overflow() {
        assert_eq!(mul_i8(i8::MIN, -1), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_i8_underflow() {
        assert_eq!(mul_i8(10, 10), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_i8_negative_times_positive() {
        assert_eq!(mul_i8(-1, 5), Ok(-5))
    }

    #[test]
    fn test_mul_i8_min_times_negative_one() {
        assert_eq!(mul_i8(i8::MIN, -1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_i16() {
        assert_eq!(mul_i16(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_i16_overflow() {
        assert_eq!(mul_i16(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_i16_underflow() {
        assert_eq!(mul_i16(10, 10), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_i32() {
        assert_eq!(mul_i32(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_i32_overflow() {
        assert_eq!(mul_i32(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_i32_underflow() {
        assert_eq!(mul_i32(10, 10), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_i64() {
        assert_eq!(mul_i64(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_i64_overflow() {
        assert_eq!(mul_i64(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_i64_underflow() {
        assert_eq!(mul_i64(10, 10), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_i128() {
        assert_eq!(mul_i128(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_i128_overflow() {
        assert_eq!(mul_i128(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_i128_underflow() {
        assert_eq!(mul_i128(10, 10), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_isize() {
        assert_eq!(mul_isize(10, 10), Ok(100))
    }

    #[test]
    fn test_mul_isize_overflow() {
        assert_eq!(mul_isize(10, 10), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_isize_underflow() {
        assert_eq!(mul_isize(10, 10), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_f32() {
        assert_eq!(mul_f32(10.0, 10.0), Ok(100.0))
    }

    #[test]
    fn test_mul_f32_overflow() {
        assert_eq!(mul_f32(10.0, 10.0), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_f32_underflow() {
        assert_eq!(mul_f32(10.0, 10.0), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(mul_f64(10.0, 10.0), Ok(100.0))
    }

    #[test]
    fn test_mul_f64_overflow() {
        assert_eq!(mul_f64(10.0, 10.0), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_mul_f64_underflow() {
        assert_eq!(mul_f64(10.0, 10.0), Err(ArithmeticError::Underflow))
    }

    // TEST DIVISION

    #[test]
    fn test_div_u8() {
        assert_eq!(div_u8(100, 10), Ok(10))
    }

    #[test]
    fn test_div_u8_division_by_zero() {
        assert_eq!(div_u8(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_u16() {
        assert_eq!(div_u16(100, 10), Ok(10))
    }

    #[test]
    fn test_div_u16_division_by_zero() {
        assert_eq!(div_u16(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_u32() {
        assert_eq!(div_u32(100, 10), Ok(10))
    }

    #[test]
    fn test_div_u32_division_by_zero() {
        assert_eq!(div_u32(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_u64() {
        assert_eq!(div_u64(100, 10), Ok(10))
    }

    #[test]
    fn test_div_u64_division_by_zero() {
        assert_eq!(div_u64(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_u128() {
        assert_eq!(div_u128(100, 10), Ok(10))
    }

    #[test]
    fn test_div_u128_division_by_zero() {
        assert_eq!(div_u128(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_usize() {
        assert_eq!(div_usize(100, 10), Ok(10))
    }

    #[test]
    fn test_div_usize_division_by_zero() {
        assert_eq!(div_usize(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_i8() {
        assert_eq!(div_i8(100, 10), Ok(10))
    }

    #[test]
    fn test_div_i8_division_by_zero() {
        assert_eq!(div_i8(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_i16() {
        assert_eq!(div_i16(100, 10), Ok(10))
    }

    #[test]
    fn test_div_i16_division_by_zero() {
        assert_eq!(div_i16(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_i32() {
        assert_eq!(div_i32(100, 10), Ok(10))
    }

    #[test]
    fn test_div_i32_division_by_zero() {
        assert_eq!(div_i32(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_i64() {
        assert_eq!(div_i64(100, 10), Ok(10))
    }

    #[test]
    fn test_div_i64_division_by_zero() {
        assert_eq!(div_i64(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_i128() {
        assert_eq!(div_i128(100, 10), Ok(10))
    }

    #[test]
    fn test_div_i128_division_by_zero() {
        assert_eq!(div_i128(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_isize() {
        assert_eq!(div_isize(100, 10), Ok(10))
    }

    #[test]
    fn test_div_isize_division_by_zero() {
        assert_eq!(div_isize(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_f32() {
        assert_eq!(div_f32(100.0, 10.0), Ok(10.0))
    }

    #[test]
    fn test_div_f32_division_by_zero() {
        assert_eq!(div_f32(100.0, 0.0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_div_f64() {
        assert_eq!(div_f64(100.0, 10.0), Ok(10.0))
    }

    #[test]
    fn test_div_f64_division_by_zero() {
        assert_eq!(div_f64(100.0, 0.0), Err(ArithmeticError::DivisionByZero))
    }

    // TEST REMAINDER

    #[test]
    fn test_rem_u8() {
        assert_eq!(rem_u8(u8::MAX, 10), Ok(5))
    }

    #[test]
    fn test_rem_u8_division_by_zero() {
        assert_eq!(rem_u8(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_u16() {
        assert_eq!(rem_u16(u16::MAX, 10), Ok(5))
    }

    #[test]
    fn test_rem_u16_division_by_zero() {
        assert_eq!(rem_u16(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_u32() {
        assert_eq!(rem_u32(u32::MAX, 10), Ok(5))
    }

    #[test]
    fn test_rem_u32_division_by_zero() {
        assert_eq!(rem_u32(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_u64() {
        assert_eq!(rem_u64(u64::MAX, 10), Ok(5))
    }

    #[test]
    fn test_rem_u64_division_by_zero() {
        assert_eq!(rem_u64(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_u128() {
        assert_eq!(rem_u128(u128::MAX, 10), Ok(5))
    }

    #[test]
    fn test_rem_u128_division_by_zero() {
        assert_eq!(rem_u128(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_usize() {
        assert_eq!(rem_usize(usize::MAX, 10), Ok(5))
    }

    #[test]
    fn test_rem_usize_division_by_zero() {
        assert_eq!(rem_usize(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_i8() {
        assert_eq!(rem_i8(i8::MAX, 10), Ok(7))
    }

    #[test]
    fn test_rem_i8_division_by_zero() {
        assert_eq!(rem_i8(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_i16() {
        assert_eq!(rem_i16(i16::MAX, 10), Ok(7))
    }

    #[test]
    fn test_rem_i16_division_by_zero() {
        assert_eq!(rem_i16(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_i32() {
        assert_eq!(rem_i32(i32::MAX, 10), Ok(7))
    }

    #[test]
    fn test_rem_i32_division_by_zero() {
        assert_eq!(rem_i32(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_i32_min_by_negative_one() {
        assert_eq!(rem_i32(i32::MIN, -1), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_rem_i64() {
        assert_eq!(rem_i64(i64::MAX, 10), Ok(7))
    }

    #[test]
    fn test_rem_i64_division_by_zero() {
        assert_eq!(rem_i64(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_i128() {
        assert_eq!(rem_i128(i128::MAX, 10), Ok(7))
    }

    #[test]
    fn test_rem_i128_division_by_zero() {
        assert_eq!(rem_i128(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_isize() {
        assert_eq!(rem_isize(isize::MAX, 10), Ok(7))
    }

    #[test]
    fn test_rem_isize_division_by_zero() {
        assert_eq!(rem_isize(100, 0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_f32() {
        assert_eq!(rem_f32(10.0, 10.0), Ok(0.0))
    }

    #[test]
    fn test_rem_f32_division_by_zero() {
        assert_eq!(rem_f32(100.0, 0.0), Err(ArithmeticError::DivisionByZero))
    }

    #[test]
    fn test_rem_f64() {
        assert_eq!(rem_f64(f64::MAX, 10.0), Ok(8.0))
    }

    #[test]
    fn test_rem_f64_division_by_zero() {
        assert_eq!(rem_f64(100.0, 0.0), Err(ArithmeticError::DivisionByZero))
    }

    // TEST POWER

    #[test]
    fn test_pow_u8() {
        assert_eq!(pow_u8(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_u8_overflow() {
        assert_eq!(pow_u8(2, 8_u8), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_u16() {
        assert_eq!(pow_u16(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_u16_overflow() {
        assert_eq!(pow_u16(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_u32() {
        assert_eq!(pow_u32(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_u32_overflow() {
        assert_eq!(pow_u32(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_u64() {
        assert_eq!(pow_u64(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_u64_invalid_exponent() {
        assert_eq!(pow_u64(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_u64_overflow() {
        assert_eq!(pow_u64(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_u128() {
        assert_eq!(pow_u128(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_u128_invalid_exponent() {
        assert_eq!(pow_u128(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_u128_overflow() {
        assert_eq!(pow_u128(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_usize() {
        assert_eq!(pow_usize(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_usize_invalid_exponent() {
        assert_eq!(pow_usize(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_usize_overflow() {
        assert_eq!(pow_usize(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_i8() {
        assert_eq!(pow_i8(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_i8_overflow() {
        assert_eq!(pow_i8(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_i8_underflow() {
        assert_eq!(pow_i8(10, 2), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_i8_invalid_exponent() {
        assert_eq!(pow_i8(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_i16() {
        assert_eq!(pow_i16(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_i16_overflow() {
        assert_eq!(pow_i16(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_i16_underflow() {
        assert_eq!(pow_i16(10, 2), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_i32() {
        assert_eq!(pow_i32(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_i32_overflow() {
        assert_eq!(pow_i32(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_i32_underflow() {
        assert_eq!(pow_i32(10, 2), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_i32_invalid_exponent() {
        assert_eq!(pow_i32(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_i64() {
        assert_eq!(pow_i64(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_i64_overflow() {
        assert_eq!(pow_i64(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_i64_underflow() {
        assert_eq!(pow_i64(10, 2), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_i64_invalid_exponent() {
        assert_eq!(pow_i64(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_i128() {
        assert_eq!(pow_i128(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_i128_overflow() {
        assert_eq!(pow_i128(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_i128_underflow() {
        assert_eq!(pow_i128(10, 2), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_i128_invalid_exponent() {
        assert_eq!(pow_i128(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_isize() {
        assert_eq!(pow_isize(10, 2), Ok(100))
    }

    #[test]
    fn test_pow_isize_overflow() {
        assert_eq!(pow_isize(10, 2), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_isize_underflow() {
        assert_eq!(pow_isize(10, 2), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_isize_invalid_exponent() {
        assert_eq!(pow_isize(10, 2), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_f32() {
        assert_eq!(pow_f32(10.0, 2.0), Ok(100.0))
    }

    #[test]
    fn test_pow_f32_overflow() {
        assert_eq!(pow_f32(10.0, 2.0), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_f32_underflow() {
        assert_eq!(pow_f32(10.0, 2.0), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_f32_invalid_exponent() {
        assert_eq!(pow_f32(10.0, 2.0), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_f32_large_exponent_ok() {
        assert_eq!(pow_f32(10.0, 2.0), Ok(100.0))
    }

    #[test]
    fn test_pow_f64() {
        assert_eq!(pow_f64(10.0, 2.0), Ok(100.0))
    }

    #[test]
    fn test_pow_f64_overflow() {
        assert_eq!(pow_f64(10.0, 2.0), Err(ArithmeticError::Overflow))
    }

    #[test]
    fn test_pow_f64_underflow() {
        assert_eq!(pow_f64(10.0, 2.0), Err(ArithmeticError::Underflow))
    }

    #[test]
    fn test_pow_f64_invalid_exponent() {
        assert_eq!(pow_f64(10.0, 2.0), Err(ArithmeticError::InvalidExponent))
    }

    #[test]
    fn test_pow_f64_negative_base_ok() {
        assert_eq!(pow_f64(10.0, -2.0), Ok(-100.0))
    }

    #[test]
    fn test_pow_f64_fractional_of_negative() {
        assert_eq!(pow_f64(10.0, 0.2), Ok(0.5))
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
