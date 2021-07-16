use crate::powers::Powers;
use crate::syntax::parser::Parser;
use crate::unit::Unit;
use num::BigRational;
use std::collections::{btree_map, BTreeMap};
use std::fmt;
use std::iter::FromIterator;

/// The data for a base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    /// The current power.
    pub power: i32,
    /// The current prefix.
    pub prefix: i32,
}

impl From<(i32, i32)> for State {
    fn from((power, prefix): (i32, i32)) -> Self {
        Self { power, prefix }
    }
}

/// A complex unit which supports powers and prefixes.
///
/// It uses a sparse internal representation where each unit is mapped to a
/// 32-bit signed power (which can be negative to indicate reciprocals) and
/// their corresponding SI prefix as the power of 10 it corresponds to.
///
/// ```
/// use facts::{Compound, Unit};
/// use std::iter::FromIterator;
///
/// let b = Compound::from_iter([(Unit::Meter, (1, -2)), (Unit::Second, (-2, 0))]);
/// assert_eq!(b.to_string(), "cm/s²");
/// ```
#[derive(Default, Clone, PartialEq, Eq)]
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

    /// Test if this unit has a numerator.
    pub fn has_numerator(&self) -> bool {
        self.names.values().any(|s| s.power > 0)
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
    pub fn factor(&self, other: &Self, value: &mut BigRational) -> bool {
        if self.is_empty() || other.is_empty() {
            return true;
        }

        let (_, lhs_bases) = self.base_units();
        let (_, rhs_bases) = other.base_units();

        if lhs_bases.len() != rhs_bases.len() {
            return false;
        }

        for (name, rhs) in &rhs_bases {
            let lhs = match lhs_bases.get(name) {
                Some(lhs) => lhs,
                None => return false,
            };

            if lhs != rhs {
                return false;
            }
        }

        for (name, state) in &other.names {
            *value *= BigRational::new(10u32.into(), 1u32.into()).pow(state.prefix * state.power);

            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power, value, multiple);
            }
        }

        for (name, state) in &self.names {
            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power * -1, value, multiple);
            }

            *value /= BigRational::new(10u32.into(), 1u32.into()).pow(state.prefix * state.power);
        }

        true
    }

    /// Calculate multiplication factors for the given multiplication.
    pub fn mul(&self, other: &Self, n: i32, lhs: &mut BigRational, rhs: &mut BigRational) -> Self {
        if self.is_empty() || other.is_empty() {
            let unit = if self.is_empty() {
                other
                    .names
                    .iter()
                    .map(|(unit, state)| (*unit, (state.power * n, state.prefix)))
                    .collect()
            } else {
                self.clone()
            };

            return unit;
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

        for (name, state) in &self.names {
            *lhs *= BigRational::new(10u32.into(), 1u32.into()).pow(state.prefix * state.power);

            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power, lhs, multiple);
            }
        }

        for (name, state) in &other.names {
            *rhs *= BigRational::new(10u32.into(), 1u32.into()).pow(state.prefix * state.power);

            if let Some(multiple) = name.multiple_ratio() {
                multiply_factor(state.power, rhs, multiple);
            }
        }

        // NB: reconstruct units if possible.
        let der = lhs_der
            .into_iter()
            .map(|(u, p)| (u, p, 1))
            .chain(rhs_der.into_iter().map(|(u, p)| (u, p, n)));

        reconstruct(der, lhs, &mut names);
        return Compound::new(names);

        /// Reconstruct names.
        fn reconstruct(
            der: impl IntoIterator<Item = (Unit, i32, i32)>,
            out: &mut BigRational,
            names: &mut BTreeMap<Unit, State>,
        ) {
            let mut powers = Powers::default();

            // Step where we try to reconstruct some of the deconstructed names.
            // We use the left-hand side to guide us on possible alternatives.
            for (unit, power, n) in der {
                powers.clear();

                if !unit.powers(&mut powers, 1) {
                    continue;
                }

                let mod_power = match bases_match(power * n, &powers, names) {
                    Some(power) => power,
                    None => continue,
                };

                for (u, s) in &powers {
                    if let btree_map::Entry::Occupied(mut e) = names.entry(u) {
                        e.get_mut().power -= s * mod_power;

                        if e.get().power == 0 {
                            e.remove_entry();
                        }
                    }
                }

                match names.entry(unit) {
                    btree_map::Entry::Vacant(e) => {
                        e.insert(State {
                            power: mod_power,
                            prefix: 0,
                        });
                    }
                    btree_map::Entry::Occupied(mut e) => {
                        e.get_mut().power += mod_power;
                    }
                };

                if let Some(multiple) = unit.multiple_ratio() {
                    // So this is kinda complicated, so bear with me. `n` is the
                    // original factor modifier, which we apply to mod_power to
                    // get the original power back. Then we multiply by `-1`
                    // because we want to shed the multiples here.
                    multiply_factor(mod_power * -1, out, multiple);
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
            let s = match names.get(&unit) {
                Some(state) => state.power,
                None => return false,
            };

            while *cur != 0 {
                let p = base * *cur;

                if p.signum() == s.signum() && p * p.signum() <= s * s.signum() {
                    return true;
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
    ///
    /// ```
    /// let unit = str::parse::<facts::Compound>("decade/s").unwrap();
    /// assert_eq!(unit.display(true).to_string(), "decades/s");
    /// ```
    pub fn display(&self, pluralize: bool) -> Display<'_> {
        Display {
            this: self,
            pluralize,
        }
    }
}

/// Construct a unit from an iterator of its constituent names and powers.
///
/// ```
/// use facts::{Unit, Compound};
/// use std::iter::FromIterator;
///
/// let a = str::parse::<facts::Compound>("cm/s^2").unwrap();
/// let b = facts::Compound::from_iter([(Unit::Meter, (1, -2)), (Unit::Second, (-2, 0))]);
///
/// assert_eq!(a, b);
/// assert_eq!(a.to_string(), "cm/s²");
/// ```
impl<S> FromIterator<(Unit, S)> for Compound
where
    State: From<S>,
{
    fn from_iter<T: IntoIterator<Item = (Unit, S)>>(iter: T) -> Self {
        let mut names = BTreeMap::new();

        for (unit, state) in iter {
            let state = State::from(state);

            if state.power != 0 {
                names.insert(unit, state);
            }
        }

        Self { names }
    }
}

/// Display helper for [Compound].
///
/// Constructed through [Compound::display].
pub struct Display<'a> {
    this: &'a Compound,
    pluralize: bool,
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;

        let mut pluralize = if self.this.names.iter().filter(|e| e.1.power >= 0).count() == 1 {
            self.pluralize
        } else {
            false
        };

        let mut it = self.this.names.iter().filter(|e| e.1.power >= 0).peekable();

        while let Some((base, data)) = it.next() {
            base.display(data, std::mem::take(&mut pluralize), 1)
                .fmt(f)?;

            if it.peek().is_some() {
                f.write_char('⋅')?;
            }
        }

        if self.this.names.iter().any(|c| c.1.power < 0) {
            write!(f, "/")?;

            let mut it = self.this.names.iter().filter(|e| e.1.power < 0).peekable();

            while let Some((base, data)) = it.next() {
                base.display(data, false, -1).fmt(f)?;

                if it.peek().is_some() {
                    f.write_char('⋅')?;
                }
            }
        }

        Ok(())
    }
}

impl std::str::FromStr for Compound {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let node = Parser::new(s).parse_unit();
        crate::eval::unit(s, node, Default::default())
    }
}

impl fmt::Debug for Compound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Compound")
            .field(&self.display(false).to_string())
            .finish()
    }
}

impl fmt::Display for Compound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(false).fmt(f)
    }
}

fn multiply_factor(pow: i32, ratio: &mut BigRational, multiple: BigRational) {
    for _ in pow..0 {
        *ratio /= multiple.clone();
    }

    for _ in 0..pow {
        *ratio *= multiple.clone();
    }
}
