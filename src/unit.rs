use bigdecimal::BigDecimal;

use crate::parser::Parser;
use std::collections::{btree_map, BTreeMap};
use std::fmt;

/// The data for a base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimpleState {
    /// The current power.
    pub power: i32,
}

impl SimpleState {
    /// Multiply the current power.
    fn mul_power(self, n: i32) -> Self {
        Self {
            power: self.power * n,
            ..self
        }
    }
}

/// The data for a base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    /// The current power.
    pub power: i32,
    /// The current prefix.
    pub prefix: i32,
}

impl State {
    /// Convert into simple state.
    fn simple(&self) -> SimpleState {
        SimpleState { power: self.power }
    }
}

const PREFIXES: [(i32, Prefix); 19] = [
    (Prefix::YOCTO, Prefix::Yocto),
    (Prefix::ZEPTO, Prefix::Zepto),
    (Prefix::ATTO, Prefix::Atto),
    (Prefix::FEMTO, Prefix::Femto),
    (Prefix::PICO, Prefix::Pico),
    (Prefix::NANO, Prefix::Nano),
    (Prefix::MICRO, Prefix::Micro),
    (Prefix::MILLI, Prefix::Milli),
    (Prefix::CENTI, Prefix::Centi),
    (Prefix::DECI, Prefix::Deci),
    (Prefix::NONE, Prefix::None),
    (Prefix::KILO, Prefix::Kilo),
    (Prefix::MEGA, Prefix::Mega),
    (Prefix::GIGA, Prefix::Giga),
    (Prefix::TERA, Prefix::Tera),
    (Prefix::PETA, Prefix::Peta),
    (Prefix::EXA, Prefix::Exa),
    (Prefix::ZETTA, Prefix::Zetta),
    (Prefix::YOTTA, Prefix::Yotta),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Prefix {
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    /// Empty prefix.
    None,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
}

impl Prefix {
    pub const YOTTA: i32 = 24;
    pub const ZETTA: i32 = 21;
    pub const EXA: i32 = 18;
    pub const PETA: i32 = 15;
    pub const TERA: i32 = 12;
    pub const GIGA: i32 = 9;
    pub const MEGA: i32 = 6;
    pub const KILO: i32 = 3;
    pub const NONE: i32 = 0;
    pub const DECI: i32 = -1;
    pub const CENTI: i32 = -2;
    pub const MILLI: i32 = -3;
    pub const MICRO: i32 = -6;
    pub const NANO: i32 = -9;
    pub const PICO: i32 = -12;
    pub const FEMTO: i32 = -15;
    pub const ATTO: i32 = -18;
    pub const ZEPTO: i32 = -21;
    pub const YOCTO: i32 = -24;

    /// Find the prefix matching the given power and return any extra that comes
    /// along.
    pub fn find(pow: i32) -> (Self, i32) {
        let (p, prefix) = match PREFIXES.binary_search_by(|e| e.0.cmp(&pow)) {
            Ok(n) => PREFIXES[n],
            Err(n) => PREFIXES[n.saturating_sub(1)],
        };

        (prefix, p - pow)
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
            Prefix::Yotta => 'Y'.fmt(f),
            Prefix::Zetta => 'Z'.fmt(f),
            Prefix::Exa => 'E'.fmt(f),
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
            Prefix::Pico => 'p'.fmt(f),
            Prefix::Femto => 'f'.fmt(f),
            Prefix::Atto => 'a'.fmt(f),
            Prefix::Zepto => 'z'.fmt(f),
            Prefix::Yocto => 'y'.fmt(f),
        }
    }
}

/// A base unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Name {
    /// Second base unit.
    /// Designated as `s`.
    Second,
    /// Gram base unit.
    /// Designated by default as `kg`.
    KiloGram,
    /// Meter base unit.
    /// Designated as `m`.
    Meter,
    /// A byte.
    /// Designated as `B`.
    Byte,

    // Derived units
    /// A Joule `kg*m^2*s^-2`.
    Joule,
    /// `Y` or `(3600 * 24 * 365)s`.
    Year,
    /// `M`.
    Month,
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
    /// An astronomical unit.
    Au,
    /// The speed of light.
    LightSpeed,
}

impl Name {
    pub fn populate_bases(
        &self,
        bases: &mut BTreeMap<Self, SimpleState>,
        state: SimpleState,
    ) -> bool {
        fn merge(bases: &mut BTreeMap<Name, SimpleState>, name: Name, state: SimpleState) {
            match bases.entry(name) {
                btree_map::Entry::Vacant(e) => {
                    e.insert(state);
                }
                btree_map::Entry::Occupied(mut e) => {
                    e.get_mut().power += state.power;
                }
            }
        }

        match self {
            Name::Second => {
                merge(bases, Name::Second, state);
                false
            }
            Name::KiloGram => {
                merge(bases, Name::KiloGram, state);
                false
            }
            Name::Meter => {
                merge(bases, Name::Meter, state);
                false
            }
            Name::Byte => {
                merge(bases, Name::Byte, state);
                false
            }
            Name::Year => {
                merge(bases, Name::Second, state);
                true
            }
            Name::Month => {
                merge(bases, Name::Second, state);
                true
            }
            Name::Week => {
                merge(bases, Name::Second, state);
                true
            }
            Name::Day => {
                merge(bases, Name::Second, state);
                true
            }
            Name::Hour => {
                merge(bases, Name::Second, state);
                true
            }
            Name::Minute => {
                merge(bases, Name::Second, state);
                true
            }
            Name::Btu | Name::Joule => {
                // kg⋅m2⋅s−2
                merge(bases, Name::KiloGram, state);
                merge(bases, Name::Meter, state.mul_power(2));
                merge(bases, Name::Second, state.mul_power(-2));
                true
            }
            Name::Au => {
                merge(bases, Name::Meter, state);
                true
            }
            Name::LightSpeed => {
                merge(bases, Name::Meter, state);
                merge(bases, Name::Second, state.mul_power(-1));
                true
            }
        }
    }

    /// The multiplication factor for this component.
    pub fn multiple(&self) -> Option<BigDecimal> {
        let m: u64 = match self {
            Name::Year => {
                return Some(BigDecimal::new(3147113076u32.into(), 2));
            }
            Name::Month => {
                return Some(BigDecimal::new(262259423u32.into(), 2));
            }
            Name::Week => 604800,
            Name::Day => 86400,
            Name::Hour => 3600,
            Name::Minute => 60,
            Name::Btu => 1055,
            Name::Au => 149597870700,
            Name::LightSpeed => 299792458,
            _ => return None,
        };

        Some(BigDecimal::from(m))
    }

    fn prefix_bias(&self) -> i32 {
        match self {
            Name::KiloGram => 3,
            _ => 0,
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Name::Second => 's'.fmt(f),
            Name::KiloGram => 'g'.fmt(f),
            Name::Meter => 'm'.fmt(f),
            Name::Byte => 'B'.fmt(f),
            Name::Joule => 'J'.fmt(f),
            Name::Year => "Y".fmt(f),
            Name::Month => "M".fmt(f),
            Name::Week => "W".fmt(f),
            Name::Day => "d".fmt(f),
            Name::Hour => "H".fmt(f),
            Name::Minute => "m".fmt(f),
            Name::Btu => "btu".fmt(f),
            Name::Au => "au".fmt(f),
            Name::LightSpeed => "c".fmt(f),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    names: BTreeMap<Name, State>,
}

impl Unit {
    pub fn empty() -> Self {
        Self {
            names: BTreeMap::new(),
        }
    }

    /// Construct a new unit.
    pub(crate) fn new(names: BTreeMap<Name, State>) -> Self {
        Self { names }
    }

    /// Test if the unit is the special empty unit.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Calculate the factor for coercing one unit to another.
    pub fn factor(&self, other: &Self) -> Option<BigDecimal> {
        let mut factor = BigDecimal::from(1);

        if self.is_empty() || other.is_empty() {
            return Some(factor);
        }

        let (_, lhs_bases) = self.base_units();
        let (_, rhs_bases) = other.base_units();

        for (name, rhs) in &rhs_bases {
            let lhs = lhs_bases.get(name)?;

            if lhs.power != rhs.power {
                return None;
            }
        }

        let mut prefix = 0;

        for (name, state) in &self.names {
            prefix += state.prefix;

            if let Some(multiple) = name.multiple() {
                multiply(state.power, &mut factor, multiple);
            }
        }

        for (name, state) in &other.names {
            prefix -= state.prefix;

            if let Some(multiple) = name.multiple() {
                multiply(state.power * -1, &mut factor, multiple);
            }
        }

        factor = factor * pow10(prefix);
        Some(factor)
    }

    pub fn mul(&self, other: Self, n: i32) -> (BigDecimal, BigDecimal, Self) {
        if self.is_empty() || other.is_empty() {
            let unit = if self.is_empty() {
                other.clone()
            } else {
                self.clone()
            };

            return (BigDecimal::from(1), BigDecimal::from(1), unit);
        }

        let (lhs_der, lhs_bases) = self.base_units();
        let (_, rhs_bases) = other.base_units();

        let mut names = BTreeMap::new();

        for (name, state) in lhs_bases {
            names.insert(
                name,
                State {
                    power: state.power,
                    prefix: 0,
                },
            );
        }

        for (name, rhs) in rhs_bases {
            match names.entry(name) {
                btree_map::Entry::Vacant(e) => {
                    e.insert(State {
                        power: rhs.power * n,
                        prefix: 0,
                    });
                }
                btree_map::Entry::Occupied(mut e) => {
                    e.get_mut().power += rhs.power * n;

                    if e.get().power == 0 {
                        e.remove_entry();
                    }
                }
            }
        }

        let mut lhs_fac = BigDecimal::from(1);
        let mut rhs_fac = BigDecimal::from(1);

        for (name, state) in &self.names {
            lhs_fac = lhs_fac * pow10(state.prefix);

            if let Some(multiple) = name.multiple() {
                multiply(state.power, &mut lhs_fac, multiple);
            }
        }

        for (name, state) in &other.names {
            rhs_fac = rhs_fac * pow10(state.prefix);

            if let Some(multiple) = name.multiple() {
                multiply(state.power, &mut rhs_fac, multiple);
            }
        }

        // Step where we try to reconstruct some of the deconstructed names.
        // We use the left-hand side to guide us on possible alternatives.
        for name in lhs_der {
            let mut bases = BTreeMap::new();

            if !name.populate_bases(&mut bases, SimpleState { power: 1 }) {
                continue;
            }

            while bases.iter().all(|e| base_match(*e.0, *e.1, &names)) {
                for (n, s) in &bases {
                    if let btree_map::Entry::Occupied(mut e) = names.entry(*n) {
                        e.get_mut().power -= s.power;

                        if e.get().power == 0 {
                            e.remove_entry();
                        }
                    }
                }

                let entry = names.entry(name).or_insert_with(|| State {
                    power: 0,
                    prefix: 0,
                });
                entry.power += 1;

                if let Some(multiple) = name.multiple() {
                    lhs_fac = lhs_fac / multiple;
                }
            }
        }

        return (lhs_fac, rhs_fac, Unit::new(names));

        fn base_match(name: Name, state: SimpleState, bases: &BTreeMap<Name, State>) -> bool {
            let base = match bases.get(&name) {
                Some(base) => base,
                None => return false,
            };

            if state.power < 0 {
                base.power < 0 && base.power <= state.power
            } else {
                base.power >= 0 && state.power <= base.power
            }
        }
    }

    /// Get all base units out of the current unit.
    fn base_units(&self) -> (Vec<Name>, BTreeMap<Name, SimpleState>) {
        let mut bases = BTreeMap::new();
        let mut derived = Vec::new();

        for (name, state) in &self.names {
            if name.populate_bases(&mut bases, state.simple()) {
                derived.push(*name);
            }
        }

        (derived, bases)
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
        let without_num = self.names.iter().all(|c| c.1.power == 0);
        let without_den = self.names.iter().all(|c| c.1.power >= 0);

        if without_num {
            if without_den {
                return Ok(());
            }

            write!(f, "1")?;
        } else {
            for (base, data) in self.names.iter().filter(|e| e.1.power >= 0) {
                fmt_help(base, f, data, 1)?;
            }
        }

        if without_den {
            return Ok(());
        }

        write!(f, "/")?;

        for (base, data) in self.names.iter().filter(|e| e.1.power < 0) {
            fmt_help(base, f, data, -1)?;
        }

        return Ok(());

        fn fmt_help(name: &Name, f: &mut fmt::Formatter<'_>, data: &State, n: i32) -> fmt::Result {
            let (prefix, extra) = Prefix::find(data.prefix + name.prefix_bias());

            if extra == 0 {
                write!(f, "{}{}", prefix, name)?;
            } else {
                let extra = pow10(extra);
                write!(f, "{}{}{}", extra, prefix, name)?;
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

/// Get the factor as a bigdecimal.
#[inline]
fn pow10(pow: i32) -> BigDecimal {
    BigDecimal::new(1.into(), -pow as i64)
}

fn multiply(pow: i32, factor: &mut BigDecimal, multiple: BigDecimal) {
    for _ in pow..0 {
        let f = std::mem::take(factor);
        *factor = f / multiple.clone();
    }

    for _ in 0..pow {
        let f = std::mem::take(factor);
        *factor = f * multiple.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::pow10;
    use bigdecimal::BigDecimal;

    #[test]
    fn test_pow10() {
        assert_eq!(pow10(-1), BigDecimal::new(1.into(), 1));
        assert_eq!(pow10(1), BigDecimal::new(1.into(), -1));
        assert_eq!(pow10(0), BigDecimal::new(1.into(), 0));
    }
}
