#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum U8Error {
    Overflow,
    Underflow,
    DivisionByZero,
}

pub struct U8;

impl U8 {
    pub const MIN: u8 = u8::MIN;
    pub const MAX: u8 = u8::MAX;

    pub fn range() -> std::ops::RangeInclusive<u8> {
        Self::MIN..=Self::MAX
    }

    pub fn add(left: u8, right: u8) -> Result<u8, U8Error> {
        if right > Self::MAX - left {
            return Err(U8Error::Overflow);
        }

        Ok(left + right)
    }

    pub fn subtract(left: u8, right: u8) -> Result<u8, U8Error> {
        if right > left {
            return Err(U8Error::Underflow);
        }

        Ok(left - right)
    }

    pub fn multiply(left: u8, right: u8) -> Result<u8, U8Error> {
        if right != 0 && left > Self::MAX / right {
            return Err(U8Error::Overflow);
        }

        Ok(left * right)
    }

    pub fn divide(left: u8, right: u8) -> Result<u8, U8Error> {
        if right == 0 {
            return Err(U8Error::DivisionByZero);
        }

        Ok(left / right)
    }
}