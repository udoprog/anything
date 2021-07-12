use crate::unit::Unit;
use std::fmt;

pub struct Numeric {
    pub value: f64,
    pub unit: Unit,
}

impl Numeric {
    /// Construct a new numerical value.
    pub fn new(value: f64, unit: Unit) -> Self {
        Self { value, unit }
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}
