use crate::compound::Compound;
use num::One;
use rational::{DisplaySpec, Rational};
use std::fmt;

/// A arbitrary precision numerical value with a unit.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Numeric {
    /// The value of this numeric type.
    pub value: Rational,
    /// The compound unit of this numeric type.
    pub unit: Compound,
}

impl Numeric {
    /// Construct a new numerical value.
    pub fn new(value: Rational, unit: Compound) -> Self {
        Self { value, unit }
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut spec = DisplaySpec::default();

        spec.limit = 12;
        spec.exponent_limit = 12;
        spec.cap = true;

        write!(f, "{}", self.value.display(&spec))?;

        if self.unit.has_numerator() {
            write!(f, " ")?;
        }

        self.unit.display(!self.value.is_one()).fmt(f)?;
        Ok(())
    }
}
