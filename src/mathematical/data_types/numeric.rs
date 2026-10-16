// Check XDoc> references> reference> rust> data_types

pub const U8_MINIMUM_VALUE: u8 = u8::MIN;
pub const U8_MAXIMUM_VALUE: u8 = u8::MAX;
pub const U16_MINIMUM_VALUE: u16 = u16::MIN;
pub const U16_MAXIMUM_VALUE: u16 = u16::MAX;
pub const U32_MINIMUM_VALUE: u32 = u32::MIN;
pub const U32_MAXIMUM_VALUE: u32 = u32::MAX;
pub const U64_MINIMUM_VALUE: u64 = u64::MIN;
pub const U64_MAXIMUM_VALUE: u64 = u64::MAX;
pub const U128_MINIMUM_VALUE: u128 = u128::MIN;
pub const U128_MAXIMUM_VALUE: u128 = u128::MAX;

pub const USIZE_MINIMUM_VALUE: usize = usize::MIN;
pub const USIZE_MAXIMUM_VALUE: usize = usize::MAX;

pub const I8_MINIMUM_VALUE: i8 = i8::MIN;
pub const I8_MAXIMUM_VALUE: i8 = i8::MAX;
pub const I16_MINIMUM_VALUE: i16 = i16::MIN;
pub const I16_MAXIMUM_VALUE: i16 = i16::MAX;
pub const I32_MINIMUM_VALUE: i32 = i32::MIN;
pub const I32_MAXIMUM_VALUE: i32 = i32::MAX;
pub const I64_MINIMUM_VALUE: i64 = i64::MIN;
pub const I64_MAXIMUM_VALUE: i64 = i64::MAX;
pub const I128_MINIMUM_VALUE: i128 = i128::MIN;
pub const I128_MAXIMUM_VALUE: i128 = i128::MAX;

pub const ISIZE_MINIMUM_VALUE: isize = isize::MIN;
pub const ISIZE_MAXIMUM_VALUE: isize = isize::MAX;

pub const F32_MINIMUM_VALUE: f32 = f32::MIN;
pub const F32_MAXIMUM_VALUE: f32 = f32::MAX;
pub const F64_MINIMUM_VALUE: f64 = f64::MIN;
pub const F64_MAXIMUM_VALUE: f64 = f64::MAX;
pub const F32_MINIMUM_POSITIVE_NORMAL_VALUE: f32 = f32::MIN_POSITIVE;
pub const F64_MAXIMUM_POSITIVE_NORMAL_VALUE: f64 = f64::MIN_POSITIVE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericType {
    U8,
    U16,
    U32, 
    U64,
    U128,
    Usize,
    I8,
    I16,
    I32, 
    I64,
    I128,
    Isize,
    F32,
    F64,
}

impl NumericType {
    pub fn is_integer(self) -> bool {
        !matches!(self, Self::F32 | Self::F64)
    }

    pub fn is_float(self) -> bool {
        matches!(self, Self::F32 | Self::F64)
    }

    pub fn bit_width(self) -> u32 {
        match self {
            Self::U8 | Self::I8 => 8, 
            Self::U16 | Self::I16 => 16,
            Self::U32 | Self::I32 | Self::F32 => 32,
            Self::U64 | Self::I64 | Self::F64 => 64,
            Self::U128 | Self::I128 => 128, 
            Self::Usize => usize::BITS,
            Self::Isize => isize::BITS,
        }
    }

    pub fn minimum_value(&self) -> NumericValue {
        match self {
            Self::U8 => NumericValue::U8(u8::MIN),
            Self::U16 => NumericValue::U16(u16::MIN),
            Self::U32 => NumericValue::U32(u32::MIN),
            Self::U64 => NumericValue::U64(u64::MIN),
            Self::U128 => NumericValue::U128(u128::MIN),
            Self::Usize => NumericValue::Usize(usize::MIN),
            Self::I8 => NumericValue::I8(i8::MIN),
            Self::I16 => NumericValue::I16(i16::MIN),
            Self::I32 => NumericValue::I32(i32::MIN),
            Self::I64 => NumericValue::I64(i64::MIN),
            Self::I128 => NumericValue::I128(i128::MIN),
            Self::Isize => NumericValue::Isize(isize::MIN),
            Self::F32 => NumericValue::F32(f32::MIN),
            Self::F64 => NumericValue::F64(f64::MIN),
        }
    }

    pub fn minimum_value_string(self) -> String {
        match self {
            Self::U8 => U8_MINIMUM_VALUE.to_string(),
            Self::U16 => U16_MINIMUM_VALUE.to_string(),
            Self::U32 => U32_MINIMUM_VALUE.to_string(),
            Self::U64 => U64_MINIMUM_VALUE.to_string(),
            Self::U128 => U128_MINIMUM_VALUE.to_string(),
            Self::Usize => USIZE_MINIMUM_VALUE.to_string(),
            Self::I8 => I8_MINIMUM_VALUE.to_string(),
            Self::I16 => I16_MINIMUM_VALUE.to_string(),
            Self::I32 => I32_MINIMUM_VALUE.to_string(),
            Self::I64 => I64_MINIMUM_VALUE.to_string(),
            Self::I128 => I128_MINIMUM_VALUE.to_string(),
            Self::Isize => ISIZE_MINIMUM_VALUE.to_string(),
            Self::F32 => F32_MINIMUM_VALUE.to_string(),
            Self::F64 => F64_MINIMUM_VALUE.to_string(),
        }
    }

    pub fn maximum_value(&self) -> NumericValue {
        match self {
            Self::U8 => NumericValue::U8(u8::MAX),
            Self::U16 => NumericValue::U16(u16::MAX),
            Self::U32 => NumericValue::U32(u32::MAX),
            Self::U64 => NumericValue::U64(u64::MAX),
            Self::U128 => NumericValue::U128(u128::MAX),
            Self::Usize => NumericValue::Usize(usize::MAX),
            Self::I8 => NumericValue::I8(i8::MAX),
            Self::I16 => NumericValue::I16(i16::MAX),
            Self::I32 => NumericValue::I32(i32::MAX),
            Self::I64 => NumericValue::I64(i64::MAX),
            Self::I128 => NumericValue::I128(i128::MAX),
            Self::Isize => NumericValue::Isize(isize::MAX),
            Self::F32 => NumericValue::F32(f32::MAX),
            Self::F64 => NumericValue::F64(f64::MAX),
        }
    }

    pub fn maximum_value_string(self) -> String {
        match self {
            Self::U8 => U8_MAXIMUM_VALUE.to_string(),
            Self::U16 => U16_MAXIMUM_VALUE.to_string(),
            Self::U32 => U32_MAXIMUM_VALUE.to_string(),
            Self::U64 => U64_MAXIMUM_VALUE.to_string(),
            Self::U128 => U128_MAXIMUM_VALUE.to_string(),
            Self::Usize => USIZE_MAXIMUM_VALUE.to_string(),
            Self::I8 => I8_MAXIMUM_VALUE.to_string(),
            Self::I16 => I16_MAXIMUM_VALUE.to_string(),
            Self::I32 => I32_MAXIMUM_VALUE.to_string(),
            Self::I64 => I64_MAXIMUM_VALUE.to_string(),
            Self::I128 => I128_MAXIMUM_VALUE.to_string(),
            Self::Isize => ISIZE_MAXIMUM_VALUE.to_string(),
            Self::F32 => F32_MAXIMUM_VALUE.to_string(),
            Self::F64 => F64_MAXIMUM_VALUE.to_string(),
        }
    }

    //pub fn range? u8::MIN..=u8::MAX
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericValue {
    U8(u8),
    U16(u16),
    U32(u32), 
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32), 
    I64(i64),
    I128(i128),
    Isize(isize),
    F32(f32),
    F64(f64),
}

impl NumericValue {
    pub fn numeric_type(&self) -> NumericType {
        match self {
            Self::U8(_) => NumericType::U8,
            Self::U16(_) => NumericType::U16,
            Self::U32(_) => NumericType::U32,
            Self::U64(_) => NumericType::U64,
            Self::U128(_) => NumericType::U128,
            Self::Usize(_) => NumericType::Usize,
            Self::I8(_) => NumericType::I8,
            Self::I16(_) => NumericType::I16,
            Self::I32(_) => NumericType::I32,
            Self::I64(_) => NumericType::I64,
            Self::I128(_) => NumericType::I128,
            Self::Isize(_) => NumericType::Isize,
            Self::F32(_) => NumericType::F32,
            Self::F64(_) => NumericType::F64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Numeric {
    pub value: NumericValue,
}

impl Numeric {
    pub fn numeric_type(&self) -> NumericType {
        self.value.numeric_type()
    }

    pub fn minimum_values() {
        println!("u8::MIN = {:?}", NumericType::U8.minimum_value());
        println!("u16::MIN = {:?}", NumericType::U16.minimum_value());
        println!("u32::MIN = {:?}", NumericType::U32.minimum_value());
        println!("u64::MIN = {:?}", NumericType::U64.minimum_value());
        println!("u128::MIN = {:?}", NumericType::U128.minimum_value());
        println!("usize::MIN = {:?}", NumericType::Usize.minimum_value());
        println!("i8::MIN = {:?}", NumericType::I8.minimum_value());
        println!("i16::MIN = {:?}", NumericType::I16.minimum_value());
        println!("i32::MIN = {:?}", NumericType::I32.minimum_value());
        println!("i64::MIN = {:?}", NumericType::I64.minimum_value());
        println!("i128::MIN = {:?}", NumericType::I128.minimum_value());
        println!("isize::MIN = {:?}", NumericType::Isize.minimum_value());
        println!("f32::MIN = {:?}", NumericType::F32.minimum_value());
        println!("f64::MIN = {:?}", NumericType::F64.minimum_value());
    }

    pub fn maximum_values() {
        println!("u8::MAX = {:?}", NumericType::U8.maximum_value());
        println!("u16::MAX = {:?}", NumericType::U16.maximum_value());
        println!("u32::MAX = {:?}", NumericType::U32.maximum_value());
        println!("u64::MAX = {:?}", NumericType::U64.maximum_value());
        println!("u128::MAX = {:?}", NumericType::U128.maximum_value());
        println!("usize::MAX = {:?}", NumericType::Usize.maximum_value());
        println!("i8::MAX = {:?}", NumericType::I8.maximum_value());
        println!("i16::MAX = {:?}", NumericType::I16.maximum_value());
        println!("i32::MAX = {:?}", NumericType::I32.maximum_value());
        println!("i64::MAX = {:?}", NumericType::I64.maximum_value());
        println!("i128::MAX = {:?}", NumericType::I128.maximum_value());
        println!("isize::MAX = {:?}", NumericType::Isize.maximum_value());
        println!("f32::MAX = {:?}", NumericType::F32.maximum_value());
        println!("f64::MAX = {:?}", NumericType::F64.maximum_value());
    }
}



pub enum NumberError {
    InvalidInput,
    InvalidParsedInput,
}

// test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32_value_preserves_type() {
        let number = Numeric {
            value: NumericValue::I32(42),
        };

        assert_eq!(number.numeric_type(), NumericType::I32);

        match number.value {
            NumericValue::I32(value) => assert_eq!(value, 42),
            _ => panic!("Expected I32"),
        }
    }

    #[test]
    fn f64_value_preserves_type() {
        let number = Numeric {
            value: NumericValue::F64(3.5),
        };

        assert_eq!(number.numeric_type(), NumericType::F64);

        match number.value {
            NumericValue::F64(value) => assert_eq!(value, 3.5),
            _ => panic!("Expected F64"),
        }
    }
}

// NUMERIC DATA TYPE CONVERSION (32-bit -> 64-bit)

// Unsigned
pub fn u32_to_u64(data_type: u32) -> u64 {
    data_type.into()
}

// Integer
pub fn i32_to_i64(data_type: i32) -> i64 {
    data_type.into()
}

// Floating-point
pub fn f32_to_f64(data_type: f32) -> f64 {
    data_type.into()
}

// configure test
#[cfg(test)]
mod tests {
use super::*;

#[test]
pub fn u32_to_u64(data_type: u32) -> u64 {
data_type.into()
}

#[test]
pub fn i32_to_i64(data_type: i32) -> i64 {
    data_type.into()
}    

#[test]
pub fn f32_to_f64(data_type: f32) -> f64 {
    data_type.into()
}
}