use bigdecimal::{BigDecimal, ToPrimitive};

use crate::compound_unit::CompoundUnit;
use std::fmt;

pub struct Numeric {
    value: BigDecimal,
    unit: CompoundUnit,
}

impl Numeric {
    /// Construct a new numerical value.
    pub(crate) fn new(value: BigDecimal, unit: CompoundUnit) -> Self {
        Self { value, unit }
    }

    /// Convert into its underlying value.
    pub(crate) fn into_value(self) -> BigDecimal {
        self.value
    }

    /// Interior method to split the numeric value into its components.
    pub(crate) fn split(self) -> (BigDecimal, CompoundUnit) {
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
    pub fn unit(&self) -> &CompoundUnit {
        &self.unit
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use bigdecimal::One as _;

        if self.value.is_integer() {
            write!(f, "{}", self.value)?;
        } else {
            write!(f, "{}", self.value.round(16))?;
        }

        self.unit.format(f, !self.value.is_one())?;
        Ok(())
    }
}
