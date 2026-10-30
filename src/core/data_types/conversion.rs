// NUMERIC DATA TYPE CONVERSION (32-bit -> 64-bit)

#[derive(Debug)]
pub enum ConversionError {
    InvalidInteger,
    InvalidFloat,
    InvalidInput,
}

pub fn convert_to_u32(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.trim().parse::<u32>()
}



// Unsigned
pub fn u32_to_u64(data_type: u32) -> u64 {
    data_type as u64
}

// Integer
pub fn i32_to_i64(data_type: i32) -> i64 {
    data_type.into()
}

// Floating-point
pub fn f32_to_f64(data_type: f32) -> f64 {
    data_type.into()
}