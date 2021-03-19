use std::collections::BTreeMap;
use std::fmt;

/// A base unit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Base {
    Meter,
    Second,
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Meter => write!(f, "m"),
            Self::Second => write!(f, "s"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unit {
    /// The unitless base unit.
    One,
    /// Units multiplied together.
    Mul { units: Vec<Unit> },
    /// One unit divided by another.
    Div { num: Box<Unit>, den: Box<Unit> },
    /// The units and its powers.
    Pow { base: Base, pow: u32 },
}

impl Unit {
    /// Normalize the unit.
    pub fn normalize(self) -> Self {
        let mut numerators = BTreeMap::new();
        let mut denominators = BTreeMap::new();

        match self {
            Unit::Pow { base, pow } => {
                return Self::Pow { base, pow };
            }
            unit => {
                unit.normalize_inner(&mut numerators, &mut denominators, false);
            }
        }

        let num = match Self::normalized_into_unit(numerators) {
            Some(num) => num,
            None => Self::One,
        };

        if let Some(den) = Self::normalized_into_unit(denominators) {
            if den.is_one() {
                num
            } else {
                Self::Div {
                    num: Box::new(num),
                    den: Box::new(den),
                }
            }
        } else {
            num
        }
    }

    /// Convert a normalized map into a unit.
    fn normalized_into_unit(map: BTreeMap<Base, u32>) -> Option<Self> {
        let mut it = map.into_iter().peekable();
        let (base, pow) = it.next()?;

        let first = Self::pow(base, pow);

        if it.peek().is_none() {
            return Some(first);
        }

        let mut units = vec![first];

        for (base, pow) in it {
            units.push(Self::pow(base, pow));
        }

        Some(Self::Mul { units })
    }

    fn normalize_inner(
        self,
        numerators: &mut BTreeMap<Base, u32>,
        denominators: &mut BTreeMap<Base, u32>,
        denom: bool,
    ) {
        match self {
            Self::Mul { units } => {
                for unit in units {
                    unit.normalize_inner(numerators, denominators, denom);
                }
            }
            Self::Div { num, den } if denom => {
                num.normalize_inner(numerators, denominators, denom);
                den.normalize_inner(numerators, denominators, denom);
            }
            Self::Div { num, den } => {
                num.normalize_inner(numerators, denominators, denom);
                // We flip numerators and denominators for the first level of
                // denominator.
                // So that `a/b/c/d` -> `a/b * 1/c * 1/d` -> `a / bcd`.
                den.normalize_inner(denominators, numerators, true);
            }
            Self::One | Self::Pow { pow: 0, .. } => {}
            Self::Pow { base, pow } => {
                *numerators.entry(base).or_default() += pow;
            }
        }
    }

    /// Test if the unit is the special one unit.
    pub fn is_one(&self) -> bool {
        matches!(self, Self::One | Self::Pow { pow: 0, .. })
    }

    /// Create the meter base unit.
    pub fn meter() -> Self {
        Self::Pow {
            base: Base::Meter,
            pow: 1,
        }
    }

    /// Create a the second base unit.
    pub fn second() -> Self {
        Self::Pow {
            base: Base::Second,
            pow: 1,
        }
    }

    /// Create a divisor unit.
    pub fn div(num: Self, den: Self) -> Self {
        Self::Div {
            num: Box::new(num),
            den: Box::new(den),
        }
    }

    /// Create a power unit.
    pub fn pow(base: Base, pow: u32) -> Self {
        match pow {
            0 => Self::One,
            pow => Self::Pow { base, pow },
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One | Self::Pow { pow: 0, .. } => {
                write!(f, "1")?;
            }
            Self::Mul { units } => {
                for u in units {
                    u.fmt(f)?;
                }
            }
            Self::Div { num, den } => {
                write!(f, "{}/{}", num, den)?;
            }
            Self::Pow { base, pow } => {
                write!(f, "{}", base)?;
                let mut pow = *pow;

                if pow != 1 {
                    if pow < 10 {
                        pow_into_char(pow).fmt(f)?;
                    } else {
                        let mut chars = Vec::new();

                        while pow != 0 {
                            chars.push(pow_into_char(pow % 10));
                            pow /= 10;
                        }

                        for c in chars.into_iter().rev() {
                            c.fmt(f)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn pow_into_char(pow: u32) -> char {
    match pow {
        0 => '⁰',
        1 => '¹',
        2 => '²',
        3 => '³',
        4 => '⁴',
        5 => '⁵',
        6 => '⁶',
        7 => '⁷',
        8 => '⁸',
        _ => '⁹',
    }
}

#[cfg(test)]
mod tests {
    use super::{Base, Unit};

    #[test]
    fn display_unit() {
        let unit = Unit::div(Unit::meter(), Unit::pow(Base::Second, 2));
        assert_eq!("m/s²", unit.to_string());
    }

    #[test]
    fn test_normalize_div_div() {
        let unit = Unit::div(Unit::meter(), Unit::div(Unit::second(), Unit::second())).normalize();
        assert_eq!("m/s²", unit.to_string());

        let unit = Unit::div(
            Unit::meter(),
            Unit::div(Unit::second(), Unit::div(Unit::second(), Unit::second())),
        )
        .normalize();
        assert_eq!("m/s³", unit.to_string());

        let unit = Unit::div(
            Unit::meter(),
            Unit::div(Unit::div(Unit::second(), Unit::second()), Unit::second()),
        )
        .normalize();
        assert_eq!("m/s³", unit.to_string());
    }

    #[test]
    fn test_large_pow() {
        let unit = Unit::div(Unit::meter(), Unit::pow(Base::Second, 103));
        assert_eq!("m/s¹⁰³", unit.to_string());
    }

    #[test]
    fn test_normalize() {
        let unit = Unit::div(Unit::meter(), Unit::pow(Base::Second, 0)).normalize();
        assert_eq!(Unit::meter(), unit);
    }
}
