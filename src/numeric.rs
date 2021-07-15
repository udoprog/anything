use crate::compound::Compound;
use bigdecimal::{BigDecimal, ParseBigDecimalError};
use num::{BigRational, One, ToPrimitive};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseNumericError {
    #[error("failed to parse big decimal: {error}")]
    ParseBigDecimalError {
        #[source]
        #[from]
        error: ParseBigDecimalError,
    },
}

/// Parse the given number as a big rational.
pub fn parse(number: &str) -> Result<BigRational, ParseNumericError> {
    Ok(big_decimal_to_big_rational(str::parse::<BigDecimal>(
        number,
    )?))
}

fn big_decimal_to_big_rational(decimal: BigDecimal) -> BigRational {
    let (big, exp) = decimal.as_bigint_and_exponent();
    let ten = BigRational::new(10u32.into(), 1u32.into()).pow(-exp as i32);
    BigRational::new(big, 1.into()) * ten
}

/// A arbitrary precision numerical value with a unit.
#[derive(Debug)]
pub struct Numeric {
    value: BigRational,
    unit: Compound,
}

impl Numeric {
    /// Construct a numeric from a big decimal.
    pub(crate) fn from_big_decimal(value: BigDecimal, unit: Compound) -> Self {
        let value = big_decimal_to_big_rational(value);
        Self { value, unit }
    }

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
            if let Some(v) = self.value.to_f64() {
                write!(f, "{}", v)?;
            } else {
                write!(f, "{{{}}}", self.value)?;
            }
        }

        self.unit.format(f, !self.value.is_one())?;
        Ok(())
    }
}
