use crate::compound::Compound;
use crate::format::FormatRatio;
use num::{BigInt, BigRational, One, ToPrimitive};
use std::fmt;
use thiserror::Error;

#[cfg(test)]
mod parse_tests;

/// A arbitrary precision numerical value with a unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Numeric {
    value: BigRational,
    unit: Compound,
}

impl Numeric {
    /// Construct a new numerical value.
    pub fn new(value: BigRational, unit: Compound) -> Self {
        Self { value, unit }
    }

    /// Construct from a 64-bit float.
    pub fn from_f64(value: f64, unit: Compound) -> Self {
        let value =
            BigRational::from_float(value).unwrap_or_else(|| BigRational::new(1.into(), 1.into()));

        Self { value, unit }
    }

    /// Convert into its underlying value.
    pub fn into_value(self) -> BigRational {
        self.value
    }

    /// Interior method to split the numeric value into its components.
    pub fn split(self) -> (BigRational, Compound) {
        (self.value, self.unit)
    }

    /// Access the underlying rational.
    pub fn value(&self) -> &BigRational {
        &self.value
    }

    /// Get the unit of the numerical value.
    pub fn unit(&self) -> &Compound {
        &self.unit
    }
}

impl ToPrimitive for Numeric {
    fn to_i8(&self) -> Option<i8> {
        self.value.to_i8()
    }

    fn to_i16(&self) -> Option<i16> {
        self.value.to_i16()
    }

    fn to_i128(&self) -> Option<i128> {
        self.value.to_i128()
    }

    fn to_usize(&self) -> Option<usize> {
        self.value.to_usize()
    }

    fn to_u8(&self) -> Option<u8> {
        self.value.to_u8()
    }

    fn to_isize(&self) -> Option<isize> {
        self.value.to_isize()
    }

    fn to_u16(&self) -> Option<u16> {
        self.value.to_u16()
    }

    fn to_u32(&self) -> Option<u32> {
        self.value.to_u32()
    }

    fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.value.to_u128()
    }

    fn to_f32(&self) -> Option<f32> {
        self.value.to_f32()
    }

    fn to_f64(&self) -> Option<f64> {
        self.value.to_f64()
    }

    fn to_i32(&self) -> Option<i32> {
        self.value.to_i32()
    }

    fn to_i64(&self) -> Option<i64> {
        self.value.to_i64()
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_integer() {
            write!(f, "{}", self.value.numer())?;
        } else {
            write!(f, "{}", FormatRatio::new(&self.value, 8, -6))?;
        }

        if self.unit.has_numerator() {
            write!(f, " ")?;
        }

        self.unit.display(!self.value.is_one()).fmt(f)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("illegal numeric value")]
pub struct ParseNumericError(());

/// Parse the given number as a big rational.
pub(crate) fn parse_decimal_big_rational(number: &str) -> Result<BigRational, ParseNumericError> {
    let mut dot = false;
    let mut init = false;
    let mut dots = 0u32;

    let mut out = BigRational::new(0u32.into(), 1u32.into());
    let ten = BigInt::from(10u32);

    let mut it = number.bytes().peekable();

    let neg = if let Some(b'-' | b'+') = it.peek() {
        matches!(it.next(), Some(b'-'))
    } else {
        false
    };

    while let Some(b) = it.next() {
        match b {
            // Ignore leading zeros.
            b'0' if !init => {
                continue;
            }
            b'0'..=b'9' => {
                init = true;

                let c = (b - b'0') as u32;
                out *= &ten;
                out += BigInt::from(c);

                if dot {
                    dots = dots.checked_add(1).ok_or(ParseNumericError(()))?;
                }
            }
            b'.' if !dot => {
                init = true;
                dot = true;
            }
            b'e' | b'E' => {
                let neg = if let Some(b'-' | b'+') = it.peek() {
                    matches!(it.next(), Some(b'-'))
                } else {
                    false
                };

                let mut exp = 0u32;
                let mut init = false;

                for b in it {
                    match b {
                        // Ignore leading zeros.
                        b'0' if !init => {
                            continue;
                        }
                        b'0'..=b'9' => {
                            init = true;
                            let n = (b - b'0') as u32;

                            exp = match exp.checked_mul(10).and_then(|exp| exp.checked_add(n)) {
                                Some(exp) => exp,
                                None => return Err(ParseNumericError(())),
                            };
                        }
                        _ => {
                            return Err(ParseNumericError(()));
                        }
                    }
                }

                if neg {
                    out /= ten.pow(exp);
                } else {
                    out *= ten.pow(exp);
                }

                break;
            }
            _ => {
                return Err(ParseNumericError(()));
            }
        }
    }

    out /= ten.pow(dots);
    let out = if neg { -out } else { out };
    Ok(out)
}
