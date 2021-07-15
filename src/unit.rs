use bigdecimal::BigDecimal;
use std::cmp;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::fmt;
use std::hash;

/// A base unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Unit {
    /// A custom derived unit.
    ///
    /// See [units][crate::units] for definitions.
    Derived(Derived),

    /// Second base unit.
    /// Designated as `s`.
    Second,
    /// Gram base unit.
    /// Designated by default as `kg`.
    KiloGram,
    /// Meter base unit.
    /// Designated as `m`.
    Meter,
    /// Ampere.
    /// Designated as `A`.
    Ampere,
    /// Kelvin.
    /// Designated as `K`.
    Kelvin,
    /// Mole.
    /// Designated as `mol`.
    Mole,
    /// Candela.
    /// Designated as `cd`.
    Candela,
    /// A byte.
    /// Designated as `B`.
    Byte,
}

impl Unit {
    pub fn powers(self, powers: &mut Powers, power: i32) -> bool {
        match self {
            Unit::Derived(derived) => {
                (derived.vtable.powers)(powers, power);
                true
            }
            unit => {
                powers.add(unit, power);
                false
            }
        }
    }

    /// The multiplication factor for this component.
    pub fn multiple(&self) -> Option<BigDecimal> {
        match self {
            Unit::Derived(derived) => Some((derived.vtable.multiple?)()),
            _ => None,
        }
    }

    pub(crate) fn prefix_bias(&self) -> i32 {
        match self {
            Unit::KiloGram => 3,
            _ => 0,
        }
    }

    pub(crate) fn format(&self, f: &mut fmt::Formatter<'_>, pluralize: bool) -> fmt::Result {
        use std::fmt::Display as _;

        match self {
            Unit::Second => 's'.fmt(f),
            Unit::KiloGram => 'g'.fmt(f),
            Unit::Meter => 'm'.fmt(f),
            Unit::Ampere => 'A'.fmt(f),
            Unit::Kelvin => 'K'.fmt(f),
            Unit::Mole => "mol".fmt(f),
            Unit::Candela => "cd".fmt(f),
            Unit::Byte => 'B'.fmt(f),
            Unit::Derived(derived) => (derived.vtable.format)(f, pluralize),
        }
    }
}

/// Helpers struct to build bases.
#[derive(Default)]
pub struct Powers {
    powers: BTreeMap<Unit, i32>,
}

impl Powers {
    /// Iterate over all populated powers.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a Unit, &'a i32)> {
        self.powers.iter()
    }

    /// Clear the current collection of powers.
    pub fn clear(&mut self) {
        self.powers.clear();
    }

    /// Add the given unit to the collection of powers.
    ///
    /// This will accumulate the specified unit onto the existing powers.
    pub fn add(&mut self, unit: Unit, power: i32) {
        match self.powers.entry(unit) {
            btree_map::Entry::Vacant(e) => {
                e.insert(power);
            }
            btree_map::Entry::Occupied(mut e) => {
                *e.get_mut() += power;
            }
        }
    }

    pub(crate) fn into_inner(self) -> BTreeMap<Unit, i32> {
        self.powers
    }
}

/// The vtable for a custom unit.
pub struct UnitVtable {
    /// Populate base powers.
    pub powers: fn(&mut Powers, i32),
    /// Format the unit.
    pub format: fn(&mut fmt::Formatter<'_>, bool) -> fmt::Result,
    /// Access multiplier.
    pub multiple: Option<fn() -> BigDecimal>,
}

/// Wrapper arounda derived unit.
#[derive(Clone, Copy)]
pub struct Derived {
    pub(crate) id: u32,
    pub(crate) vtable: &'static UnitVtable,
}

impl fmt::Debug for Derived {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Derived")
            .field("id", &format_args!("{:#x}", self.id))
            .field("vtable", &(self.vtable as *const _))
            .finish()
    }
}

impl cmp::PartialEq for Derived {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl cmp::PartialOrd for Derived {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl cmp::Ord for Derived {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl cmp::Eq for Derived {}

impl hash::Hash for Derived {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
    }
}
