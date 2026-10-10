// Check XDoc> references> reference> rust> data_types
pub const USIZE_MINIMUM_VALUE: usize = usize::MIN;
pub const USIZE_MAXIMUM_VALUE: usize = usize::MAX;

pub const U32_MINIMUM_VALUE: u32 = u32::MIN;
pub const U32_MAXIMUM_VALUE: u32 = u32::MAX;

pub const U64_MINIMUM_VALUE: u64 = u64::MIN;
pub const U64_MAXIMUM_VALUE: u64 = u64::MAX;

pub const I32_MINIMUM_VALUE: i32 = i32::MIN;
pub const I32_MAXIMUM_VALUE: i32 = i32::MAX;

pub const I64_MINIMUM_VALUE: i64 = i64::MIN;
pub const I64_MAXIMUM_VALUE: i64 = i64::MAX;

pub const F32_MINIMUM_VALUE: f32 = f32::MIN;
pub const F32_MAXIMUM_VALUE: f32 = f32::MAX;

pub const F64_MINIMUM_VALUE: f64 = f64::MIN;
pub const F64_MAXIMUM_VALUE: f64 = f64::MAX;

pub const F32_MINIMUM_POSITIVE_NORMAL_VALUE: f32 = f32::MIN_POSITIVE;
pub const F64_MAXIMUM_POSITIVE_NORMAL_VALUE: f64 = f64::MIN_POSITIVE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericType {
    Usize,
    U32, 
    U64,
    I32, 
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumericValue {
    Usize(usize),
    U32(u32), 
    U64(u64),
    I32(i32), 
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Numeric {
    pub value: NumericValue,
}

impl NumericValue {
    pub fn numeric_type(&self) -> NumericType {
        match self {
            Self::Usize(_) => NumericType::Usize,
            Self::U32(_) => NumericType::U32,
            Self::U64(_) => NumericType::U64,
            Self::I32(_) => NumericType::I32,
            Self::I64(_) => NumericType::I64,
            Self::F32(_) => NumericType::F32,
            Self::F64(_) => NumericType::F64,
        }
    }
}

impl Numeric {
    pub fn numeric_type(&self) -> NumericType {
        self.value.numeric_type()
    }
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