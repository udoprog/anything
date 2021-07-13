use crate::parser::Parser;
use std::collections::{btree_map, BTreeMap};
use std::fmt;

/// The data for a base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseData {
    /// The current power.
    pub power: i32,
    /// The current prefix.
    pub prefix: Prefix,
    /// If the unit is a special multiple unit.
    pub multiple: Multiple,
}

impl BaseData {
    pub fn factor(&self) -> bigdecimal::BigDecimal {
        let prefix = self.prefix.factor();

        if let Multiple::None = self.multiple {
            return prefix;
        }

        prefix * self.multiple.factor()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Multiple {
    /// `Y` or `(3600 * 24 * 365)s`.
    Year,
    /// `d` or `(3600 * 24 * 7)s`.
    Week,
    /// `d` or `86400s`.
    Day,
    /// `H` or `3600s`.
    Hour,
    /// `m` or `60s`.
    Minute,
    /// A British Thermal Unit, or `1055J`.
    Btu,
    /// No specific multiple.
    None,
}

impl Multiple {
    /// Convert the multiple into a multiplication factor.
    pub fn factor(&self) -> bigdecimal::BigDecimal {
        let m: u32 = match self {
            Multiple::Year => 3600 * 24 * 265,
            Multiple::Week => 3600 * 24 * 7,
            Multiple::Day => 3600 * 24,
            Multiple::Hour => 3600,
            Multiple::Minute => 60,
            Multiple::Btu => 1055,
            Multiple::None => 1,
        };

        bigdecimal::BigDecimal::from(m)
    }
}

impl fmt::Display for Multiple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Multiple::Year => "Y".fmt(f),
            Multiple::Week => "W".fmt(f),
            Multiple::Day => "d".fmt(f),
            Multiple::Hour => "H".fmt(f),
            Multiple::Minute => "m".fmt(f),
            Multiple::Btu => "BTU".fmt(f),
            Multiple::None => "*".fmt(f),
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
    /// `d`.
    Deci,
    /// `c`.
    Centi,
    /// `m`.
    Milli,
    /// `μ`.
    Micro,
    /// `n`.
    Nano,
}

impl Prefix {
    /// Get the factor for a given prefix.
    pub fn pow(&self) -> i32 {
        match self {
            Prefix::Peta => 15,
            Prefix::Tera => 12,
            Prefix::Giga => 9,
            Prefix::Mega => 6,
            Prefix::Kilo => 3,
            Prefix::None => 0,
            Prefix::Deci => -1,
            Prefix::Centi => -2,
            Prefix::Milli => -3,
            Prefix::Micro => -6,
            Prefix::Nano => -9,
        }
    }

    /// Test if prefix is none.
    pub fn is_none(&self) -> bool {
        matches!(self, Prefix::None)
    }

    /// Get the factor as a bigdecimal.
    pub fn factor(&self) -> bigdecimal::BigDecimal {
        let mut pow = self.pow();

        if pow == 0 {
            return bigdecimal::BigDecimal::from(1);
        }

        let mut base = bigdecimal::BigDecimal::from(1);

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
            Prefix::Deci => 'd'.fmt(f),
            Prefix::Centi => 'c'.fmt(f),
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
    /// A joule.
    ///
    /// Designated as `J`.
    Joule,
    /// A byte.
    /// Designated as `B`.
    Byte,
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Base::Second => 's'.fmt(f),
            Base::Gram => 'g'.fmt(f),
            Base::Meter => 'm'.fmt(f),
            Base::Joule => 'J'.fmt(f),
            Base::Byte => 'B'.fmt(f),
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
                factor *= data.multiple.factor();
            } else {
                factor = factor / data.multiple.factor();
            }
        }

        factor
    }

    /// Calculate the factor for coercing one unit to another.
    pub fn factor(&self, other: &Self) -> Option<bigdecimal::BigDecimal> {
        let mut factor = bigdecimal::BigDecimal::from(1);

        if self.is_empty() || other.is_empty() {
            return Some(factor);
        }

        for (unit, rhs) in &other.bases {
            let lhs = self.bases.get(unit)?;

            if lhs.power != rhs.power {
                return None;
            }

            factor *= lhs.factor() / rhs.factor();
        }

        Some(factor)
    }

    pub fn mul(&mut self, other: Self, n: i32) -> bigdecimal::BigDecimal {
        let mut factor = bigdecimal::BigDecimal::from(1);

        for (unit, rhs) in other.bases {
            match self.bases.entry(unit) {
                btree_map::Entry::Vacant(e) => {
                    e.insert(BaseData {
                        prefix: rhs.prefix,
                        power: rhs.power * n,
                        multiple: rhs.multiple,
                    });
                }
                btree_map::Entry::Occupied(mut o) => {
                    let lhs = o.get_mut();
                    factor *= rhs.factor() / lhs.factor();
                    lhs.power += rhs.power * n;

                    if lhs.power == 0 {
                        o.remove_entry();
                    }
                }
            }
        }

        factor
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
                (base, Multiple::None) => {
                    write!(f, "{}", base)?;
                }
                (_, multiple) => {
                    write!(f, "{}", multiple)?;
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
