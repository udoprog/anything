use crate::compound::Compound;
use crate::format::FormatRatio;
use num::{BigInt, BigRational, One, ToPrimitive};
use std::fmt;
use thiserror::Error;

#[cfg(test)]
mod parse_tests;

/// A arbitrary precision numerical value with a unit.
#[derive(Debug)]
pub struct Numeric {
    value: BigRational,
    unit: Compound,
}

impl Numeric {
    /// Construct a new numerical value.
    pub(crate) fn new(value: BigRational, unit: Compound) -> Self {
        Self { value, unit }
    }

    /// Convert into its underlying value.
    pub(crate) fn into_value(self) -> BigRational {
        self.value
    }

    /// Interior method to split the numeric value into its components.
    pub(crate) fn split(self) -> (BigRational, Compound) {
        (self.value, self.unit)
    }

    /// Get the value as a 32-bit integer.
    pub fn to_u32(&self) -> Option<u32> {
        self.value.to_u32()
    }

    /// Get the value as a 32-bit float.
    pub fn to_f32(&self) -> Option<f32> {
        self.value.to_f32()
    }

    /// Get the value as a 64-bit float.
    pub fn to_f64(&self) -> Option<f64> {
        self.value.to_f64()
    }

    /// Get the unit of the numerical value.
    pub fn unit(&self) -> &Compound {
        &self.unit
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_integer() {
            write!(f, "{}", self.value.numer())?;
        } else {
            write!(f, "{}", FormatRatio::new(&self.value, 8, -6))?;
        }

        self.unit.format(f, !self.value.is_one())?;
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
    let mut dots = 0;

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
                out *= ten.clone();
                out += BigInt::from(c);

                if dot {
                    dots += 1;
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

                while let Some(b) = it.next() {
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
