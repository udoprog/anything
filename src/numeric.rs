use crate::unit::Unit;
use std::fmt;

pub struct Numeric {
    pub value: bigdecimal::BigDecimal,
    pub unit: Unit,
}

impl Numeric {
    /// Construct a new numerical value.
    pub fn new(value: bigdecimal::BigDecimal, unit: Unit) -> Self {
        Self { value, unit }
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_integer() {
            write!(f, "{}{}", self.value, self.unit)
        } else {
            write!(f, "{}{}", self.value.round(8), self.unit)
        }
    }
}
