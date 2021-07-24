use crate::compound::Compound;
use num::One;
use rational::Rational;
use std::fmt;

/// A arbitrary precision numerical value with a unit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Numeric {
    value: Rational,
    unit: Compound,
}

impl Numeric {
    /// Construct a new numerical value.
    pub fn new(value: Rational, unit: Compound) -> Self {
        Self { value, unit }
    }

    /// Convert into its underlying value.
    pub fn into_value(self) -> Rational {
        self.value
    }

    /// Interior method to split the numeric value into its components.
    pub fn split(self) -> (Rational, Compound) {
        (self.value, self.unit)
    }

    /// Access the underlying rational.
    pub fn value(&self) -> &Rational {
        &self.value
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
            write!(f, "{}", self.value.display(8, -6, true))?;
        }

        if self.unit.has_numerator() {
            write!(f, " ")?;
        }

        self.unit.display(!self.value.is_one()).fmt(f)?;
        Ok(())
    }
}
