use crate::powers::Powers;
use bigdecimal::BigDecimal;
use num::BigRational;
use std::cmp;
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
    /// KiloGram base unit as `kg`.
    KiloGram,
    /// Candela base unit as `cd`.
    Candela,
    /// Meter base unit as `m`.
    Meter,
    /// Second base unit as `s`.
    Second,
    /// Ampere base unit as `A`.
    Ampere,
    /// Kelvin base unit as `K`.
    Kelvin,
    /// Mole base unit as `mol`.
    Mole,
    /// A byte base unit as `B`.
    Byte,
}

impl Unit {
    /// Populate powers for the current unit.
    ///
    /// ```
    /// use facts::Unit;
    ///
    /// let second = Unit::Second;
    /// let mut powers = Default::default();
    ///
    /// second.powers(&mut powers, -1);
    ///
    /// assert_eq!(powers.get(Unit::Second), Some(-1));
    /// ```
    pub fn powers(self, powers: &mut Powers, power: i32) -> bool {
        match self {
            Unit::Derived(derived) => {
                (derived.vtable.powers)(powers, power);
                true
            }
            unit => {
                powers.insert(unit, power);
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

    /// Get the multiple of the type as a ratio.
    pub fn multiple_ratio(&self) -> Option<BigRational> {
        match self {
            Unit::Derived(derived) => Some((derived.vtable.multiple_ratio?)()),
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

/// The vtable for a derived unit.
pub struct DerivedVtable {
    /// Populate base powers.
    pub powers: fn(&mut Powers, i32),
    /// Format the unit.
    pub format: fn(&mut fmt::Formatter<'_>, bool) -> fmt::Result,
    /// Access multiplier.
    pub multiple: Option<fn() -> BigDecimal>,
    /// Access multiplier as a ratio.
    pub multiple_ratio: Option<fn() -> BigRational>,
}

/// Wrapper arounda derived unit.
#[derive(Clone, Copy)]
pub struct Derived {
    /// The unique identifier for this derived unit.
    pub id: u32,
    /// The implementation of the unit.
    pub vtable: &'static DerivedVtable,
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
