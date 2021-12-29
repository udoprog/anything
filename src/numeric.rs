use crate::compound::Compound;
use rational::Rational;

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
