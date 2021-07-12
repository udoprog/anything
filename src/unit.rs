use crate::parser::Parser;
use std::collections::BTreeMap;
use std::fmt;

/// The data for a base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseData {
    /// The current power.
    pub power: i32,
    /// The current prefix.
    pub prefix: Prefix,
    /// The multiple of the value contained.
    pub multiple: u32,
}

impl BaseData {
    pub fn big_factor(&self) -> bigdecimal::BigDecimal {
        let prefix = self.prefix.big_factor();

        if self.multiple == 1 {
            return prefix;
        }

        prefix * bigdecimal::BigDecimal::from(self.multiple)
    }

    pub fn max(self, other: Self) -> Self {
        if self.big_factor() > other.big_factor() {
            self
        } else {
            other
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Prefix {
    /// `P`.
    Peta,
    /// `T`.
    Tera,
    /// `G`.
    Giga,
    /// `M`.
    Mega,
    /// `k`.
    Kilo,
    /// Empty prefix.
    None,
    /// `m`.
    Milli,
    /// `μ`.
    Micro,
    /// `n`.
    Nano,
}

impl Prefix {
    /// Get the factor for a given prefix.
    pub fn factor(&self) -> i32 {
        match self {
            Prefix::Peta => 15,
            Prefix::Tera => 12,
            Prefix::Giga => 9,
            Prefix::Mega => 6,
            Prefix::Kilo => 3,
            Prefix::None => 0,
            Prefix::Milli => -3,
            Prefix::Micro => -6,
            Prefix::Nano => -9,
        }
    }

    /// Get the factor as a bigdecimal.
    pub fn big_factor(&self) -> bigdecimal::BigDecimal {
        let mut pow = self.factor();

        if pow == 0 {
            return bigdecimal::BigDecimal::from(1);
        }

        let mut base = bigdecimal::BigDecimal::from(10);

        while pow > 0 {
            base = base * bigdecimal::BigDecimal::from(10);
            pow -= 1;
        }

        while pow < 0 {
            base = base / bigdecimal::BigDecimal::from(10);
            pow += 1;
        }

        base
    }

    /// Parse a character as a prefix.
    pub(crate) fn parse(c: char) -> Option<Prefix> {
        Some(match c {
            'P' => Prefix::Peta,
            'T' => Prefix::Tera,
            'G' => Prefix::Giga,
            'M' => Prefix::Mega,
            'k' => Prefix::Kilo,
            'm' => Prefix::Milli,
            'μ' => Prefix::Micro,
            'n' => Prefix::Nano,
            _ => return None,
        })
    }
}

impl Default for Prefix {
    fn default() -> Self {
        Prefix::None
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Peta => 'P'.fmt(f),
            Prefix::Tera => 'T'.fmt(f),
            Prefix::Giga => 'G'.fmt(f),
            Prefix::Mega => 'M'.fmt(f),
            Prefix::Kilo => 'k'.fmt(f),
            Prefix::None => Ok(()),
            Prefix::Milli => 'm'.fmt(f),
            Prefix::Micro => 'μ'.fmt(f),
            Prefix::Nano => 'n'.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Base {
    /// Second base unit.
    /// Designated as `s`.
    Second,
    /// Gram base unit.
    /// Designated by default as `g`.
    Gram,
    /// Meter base unit.
    /// Designated as `m`.
    Meter,
}

impl Base {
    /// Parse the given character as a base unit.
    pub fn parse(c: char) -> Option<Self> {
        Some(match c {
            's' => Base::Second,
            'g' => Base::Gram,
            'm' => Base::Meter,
            _ => return None,
        })
    }
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base::Second => 's'.fmt(f),
            Base::Gram => 'g'.fmt(f),
            Base::Meter => 'm'.fmt(f),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    bases: BTreeMap<Base, BaseData>,
}

impl Unit {
    pub fn empty() -> Self {
        Self {
            bases: BTreeMap::new(),
        }
    }

    /// Construct a new unit.
    pub(crate) fn new(bases: BTreeMap<Base, BaseData>) -> Self {
        Self { bases }
    }

    /// Test if the unit is the special empty unit.
    pub fn is_empty(&self) -> bool {
        self.bases.is_empty()
    }

    /// Get the value factor.
    pub fn value_factor(&self) -> bigdecimal::BigDecimal {
        let mut factor = bigdecimal::BigDecimal::from(1);

        for data in self.bases.values() {
            if data.power > 0 {
                factor *= bigdecimal::BigDecimal::from(data.multiple);
            } else {
                factor = factor / bigdecimal::BigDecimal::from(data.multiple);
            }
        }

        factor
    }

    /// Calculate the factor for coercing one unit to another.
    pub fn factor(
        &mut self,
        other: &Self,
    ) -> Option<(bigdecimal::BigDecimal, bigdecimal::BigDecimal)> {
        let mut lhs_factor = bigdecimal::BigDecimal::from(1);
        let mut rhs_factor = bigdecimal::BigDecimal::from(1);

        if self.is_empty() || other.is_empty() {
            return Some((lhs_factor, rhs_factor));
        }

        for (unit, rhs) in &other.bases {
            let lhs = self.bases.get_mut(unit)?;

            if lhs.power != rhs.power {
                return None;
            }

            let from = BaseData::max(*lhs, *rhs);
            lhs_factor *= lhs.big_factor() / from.big_factor();
            rhs_factor *= rhs.big_factor() / from.big_factor();
            lhs.prefix = from.prefix;
            lhs.multiple = from.multiple;
        }

        Some((lhs_factor, rhs_factor))
    }

    pub fn mul(&mut self, other: Self, n: i32) -> (bigdecimal::BigDecimal, bigdecimal::BigDecimal) {
        let mut lhs_factor = bigdecimal::BigDecimal::from(1);
        let mut rhs_factor = bigdecimal::BigDecimal::from(1);

        for (unit, rhs) in other.bases {
            let lhs = self.bases.entry(unit).or_insert_with(|| BaseData {
                prefix: rhs.prefix,
                power: 0,
                multiple: 1,
            });
            let rhs_power = rhs.power * n;

            let from = BaseData::max(*lhs, rhs);
            lhs_factor *= lhs.big_factor() / from.big_factor();
            rhs_factor *= rhs.big_factor() / from.big_factor();

            lhs.power += rhs_power;

            if lhs.power == 0 {
                self.bases.remove(&unit);
            } else {
                lhs.prefix = from.prefix;
                lhs.multiple = from.multiple;
            }
        }

        (lhs_factor, rhs_factor)
    }
}

impl std::str::FromStr for Unit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node = Parser::new(s).parse_unit();
        crate::eval::unit(s, node)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let without_den = self.bases.iter().all(|c| c.1.power >= 0);

        if self.bases.iter().all(|c| c.1.power == 0) {
            if without_den {
                return Ok(());
            }

            write!(f, "1")?;
        } else {
            for (base, data) in self.bases.iter().filter(|e| e.1.power >= 0) {
                fmt_base(base, f, data, 1)?;
            }
        }

        if without_den {
            return Ok(());
        }

        write!(f, "/")?;

        for (base, data) in self.bases.iter().filter(|e| e.1.power < 0) {
            fmt_base(base, f, data, -1)?;
        }

        return Ok(());

        fn fmt_base(
            base: &Base,
            f: &mut fmt::Formatter<'_>,
            data: &BaseData,
            n: i32,
        ) -> fmt::Result {
            write!(f, "{}", data.prefix)?;

            match (base, data.multiple) {
                (Base::Second, 60) => {
                    write!(f, "{{minutes}}")?;
                }
                (base, _) => {
                    write!(f, "{}", base)?;
                }
            }

            let mut power = (data.power * n) as u32;

            if power != 1 {
                if power < 10 {
                    pow_into_char(power).fmt(f)?;
                } else {
                    let mut chars = Vec::new();

                    while power != 0 {
                        chars.push(pow_into_char(power % 10));
                        power /= 10;
                    }

                    for c in chars.into_iter().rev() {
                        c.fmt(f)?;
                    }
                }
            }

            Ok(())
        }
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
