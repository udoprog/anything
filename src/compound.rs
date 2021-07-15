use crate::powers::Powers;
use crate::prefix::Prefix;
use crate::syntax::parser::Parser;
use crate::unit::Unit;
use num::BigRational;
use std::collections::{btree_map, BTreeMap};
use std::fmt;

/// A calculated factor.
pub struct Factor {
    prefix: i32,
    ratio: BigRational,
}

impl Factor {
    fn new() -> Self {
        Self {
            prefix: 0,
            ratio: BigRational::new(1.into(), 1.into()),
        }
    }

    /// Convert into a big rational
    pub fn into_big_rational(self) -> BigRational {
        let ten = BigRational::new(10u32.into(), 1u32.into()).pow(self.prefix);
        self.ratio * ten
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

/// A complex unit which supports powers and prefixes.
///
/// It uses a sparse internal representation where each unit is mapped to a
/// 32-bit signed power (which can be negative to indicate reciprocals) and
/// their corresponding SI prefix as the power of 10 it corresponds to.
///
/// ```
/// use facts::{Compound, Unit};
///
/// let b = Compound::from_iter([(Unit::Meter, 1, -2), (Unit::Second, -2, 0)]);
/// assert_eq!(b.to_string(), "cm/s²");
/// ```
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Compound {
    names: BTreeMap<Unit, State>,
}

impl Compound {
    /// Construct the empty unit.
    ///
    /// ```
    /// let unit = facts::Compound::empty();
    /// assert!(unit.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self {
            names: BTreeMap::new(),
        }
    }

    /// Construct a unit from an iterator of its constituent names and powers.
    ///
    /// ```
    /// use facts::{Unit, Compound};
    ///
    /// let a = str::parse::<facts::Compound>("cm/s^2").unwrap();
    /// let b = facts::Compound::from_iter([(Unit::Meter, 1, -2), (Unit::Second, -2, 0)]);
    ///
    /// assert_eq!(a, b);
    /// assert_eq!(a.to_string(), "cm/s²");
    /// ```
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Unit, i32, i32)>,
    {
        let mut names = BTreeMap::new();

        for (unit, power, prefix) in iter {
            if power != 0 {
                names.insert(unit, State { power, prefix });
            }
        }

        Self { names }
    }

    /// Internal only function to construct a new unit.
    ///
    /// Caller must ensure that no names with a power of 0 are specified during
    /// construction, otherwise certain internal invariants will not hold.
    pub(crate) fn new(names: BTreeMap<Unit, State>) -> Self {
        debug_assert!(
            names.values().all(|s| s.power != 0),
            "all powers of a constructed unit must be non-zero; {:?}",
            names
        );
        Self { names }
    }

    /// Test if the unit is the special empty unit.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Test if this unit is an acceleration unit.
    pub fn is_acceleration(&self) -> bool {
        let (_, bases) = self.base_units();

        if bases.len() != 2 {
            return false;
        }

        let meter = bases.get(Unit::Meter);
        let second = bases.get(Unit::Second);
        meter == Some(1) && second == Some(-2)
    }

    /// Calculate the factor for coercing one unit to another.
    pub fn factor(&self, other: &Self) -> Option<Factor> {
        let mut factor = Factor::new();

        if self.is_empty() || other.is_empty() {
            return Some(factor);
        }

        let (_, lhs_bases) = self.base_units();
        let (_, rhs_bases) = other.base_units();

        if lhs_bases.len() != rhs_bases.len() {
            return None;
        }

        for (name, rhs) in &rhs_bases {
            let lhs = lhs_bases.get(name)?;

            if lhs != rhs {
                return None;
            }
        }

        for (name, state) in &self.names {
            factor.prefix += state.prefix;

            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power, &mut factor, multiple);
            }
        }

        for (name, state) in &other.names {
            factor.prefix -= state.prefix;

            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power * -1, &mut factor, multiple);
            }
        }

        Some(factor)
    }

    /// Calculate multiplication factors for the given multiplication.
    pub fn mul(&self, other: &Self, n: i32) -> (Factor, Factor, Self) {
        if self.is_empty() || other.is_empty() {
            let unit = if self.is_empty() {
                Self::from_iter(
                    other
                        .names
                        .iter()
                        .map(|(unit, state)| (*unit, state.power * n, state.prefix)),
                )
            } else {
                self.clone()
            };

            return (Factor::new(), Factor::new(), unit);
        }

        let (lhs_der, lhs_bases) = self.base_units();
        let (rhs_der, rhs_bases) = other.base_units();

        let mut names = BTreeMap::new();

        for (name, power) in lhs_bases {
            names.insert(name, State { power, prefix: 0 });
        }

        for (name, power) in rhs_bases {
            match names.entry(name) {
                btree_map::Entry::Vacant(e) => {
                    e.insert(State {
                        power: power * n,
                        prefix: 0,
                    });
                }
                btree_map::Entry::Occupied(mut e) => {
                    e.get_mut().power += power * n;

                    if e.get().power == 0 {
                        e.remove_entry();
                    }
                }
            }
        }

        let mut lhs_fac = Factor::new();
        let mut rhs_fac = Factor::new();

        for (name, state) in &self.names {
            lhs_fac.prefix += state.prefix;

            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power, &mut lhs_fac, multiple);
            }
        }

        for (name, state) in &other.names {
            rhs_fac.prefix += state.prefix;

            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power, &mut rhs_fac, multiple);
            }
        }

        // NB: reconstruct units if possible.
        let der = lhs_der
            .into_iter()
            .chain(rhs_der.into_iter().map(|(u, p)| (u, p * n)));
        reconstruct(der, &mut lhs_fac, &mut names);

        return (lhs_fac, rhs_fac, Compound::new(names));

        /// Reconstruct names.
        fn reconstruct(
            der: impl IntoIterator<Item = (Unit, i32)>,
            fac: &mut Factor,
            names: &mut BTreeMap<Unit, State>,
        ) {
            let mut powers = Powers::default();

            // Step where we try to reconstruct some of the deconstructed names.
            // We use the left-hand side to guide us on possible alternatives.
            for (unit, power) in der {
                powers.clear();

                if !unit.powers(&mut powers, 1) {
                    continue;
                }

                let power = match bases_match(power, &powers, names) {
                    Some(power) => power,
                    None => continue,
                };

                for (u, s) in &powers {
                    if let btree_map::Entry::Occupied(mut e) = names.entry(u) {
                        e.get_mut().power -= s * power;

                        if e.get().power == 0 {
                            e.remove_entry();
                        }
                    }
                }

                match names.entry(unit) {
                    btree_map::Entry::Vacant(e) => {
                        e.insert(State { power, prefix: 0 });
                    }
                    btree_map::Entry::Occupied(mut e) => {
                        e.get_mut().power += power;
                    }
                };

                if let Some(multiple) = unit.multiple_ratio() {
                    fac.ratio /= multiple;
                }
            }
        }

        fn bases_match(
            mut power: i32,
            powers: &Powers,
            names: &BTreeMap<Unit, State>,
        ) -> Option<i32> {
            let dec = power.signum();

            let m = |(u, p)| inner_match(u, p, &mut power, dec, names);

            if powers.iter().all(m) {
                return Some(power);
            }

            None
        }

        fn inner_match(
            unit: Unit,
            base: i32,
            cur: &mut i32,
            dec: i32,
            names: &BTreeMap<Unit, State>,
        ) -> bool {
            let state = match names.get(&unit) {
                Some(state) => state,
                None => return false,
            };

            while *cur != 0 {
                let power = base * *cur;

                if power.signum() == state.power.signum() {
                    if power * power.signum() <= state.power * state.power.signum() {
                        return true;
                    }
                }

                *cur -= dec;
            }

            false
        }
    }

    /// Get all base units out of the current unit.
    fn base_units(&self) -> (Vec<(Unit, i32)>, Powers) {
        let mut powers = Powers::default();
        let mut derived = Vec::new();

        for (name, state) in &self.names {
            if name.powers(&mut powers, state.power) {
                derived.push((*name, state.power));
            }
        }

        (derived, powers)
    }

    /// Helper to format a compound unit. This allows for pluralization in the
    /// scenario that this compound unit is composed of a single unit.
    pub(crate) fn format(&self, f: &mut fmt::Formatter<'_>, pluralize: bool) -> fmt::Result {
        use std::fmt::Write;

        let mut pluralize = if self.names.iter().filter(|e| e.1.power >= 0).count() == 1 {
            pluralize
        } else {
            false
        };

        let mut it = self.names.iter().filter(|e| e.1.power >= 0).peekable();

        while let Some((base, data)) = it.next() {
            inner(base, f, data, std::mem::take(&mut pluralize), 1)?;

            if it.peek().is_some() {
                f.write_char('⋅')?;
            }
        }

        if self.names.iter().any(|c| c.1.power < 0) {
            write!(f, "/")?;

            let mut it = self.names.iter().filter(|e| e.1.power < 0).peekable();

            while let Some((base, data)) = it.next() {
                inner(base, f, data, false, -1)?;

                if it.peek().is_some() {
                    f.write_char('⋅')?;
                }
            }
        }

        return Ok(());

        fn inner(
            unit: &Unit,
            f: &mut fmt::Formatter<'_>,
            data: &State,
            pluralize: bool,
            n: i32,
        ) -> fmt::Result {
            use std::fmt::Display as _;

            let (prefix, extra) = Prefix::find(data.prefix + unit.prefix_bias());

            if extra == 0 {
                write!(f, "{}", prefix)?;
            } else {
                write!(f, "e{}{}", extra, prefix)?;
            }

            unit.format(f, pluralize)?;

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

impl std::str::FromStr for Compound {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node = Parser::new(s).parse_unit();
        crate::eval::unit(s, node, Default::default())
    }
}

impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Compound::format(self, f, false)
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

fn multiply_factor(pow: i32, factor: &mut Factor, multiple: BigRational) {
    for _ in pow..0 {
        factor.ratio /= multiple.clone();
    }

    for _ in 0..pow {
        factor.ratio *= multiple.clone();
    }
}
