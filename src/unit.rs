use crate::compound::State;
use crate::powers::Powers;
use crate::prefix::Prefix;
use rational::Rational;
use serde::de;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::fmt;
use std::hash;

/// A base unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

    /// Access conversion functions for the given unit.
    pub fn conversion(&self) -> Option<Conversion> {
        if let Unit::Derived(d) = self {
            d.vtable.conversion
        } else {
            None
        }
    }

    pub(crate) fn prefix_bias(&self) -> i32 {
        match self {
            Unit::KiloGram => 3,
            _ => 0,
        }
    }

    pub(crate) fn format_suffix(&self, f: &mut fmt::Formatter<'_>, pluralize: bool) -> fmt::Result {
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

    pub(crate) fn display<'a>(&'a self, data: &'a State, pluralize: bool, n: i32) -> Display<'a> {
        Display {
            unit: self,
            data,
            pluralize,
            n,
        }
    }
}

/// Display helper for unit.
///
/// Constructed through [Unit::display].
pub(crate) struct Display<'a> {
    unit: &'a Unit,
    data: &'a State,
    pluralize: bool,
    n: i32,
}

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (prefix, extra) = Prefix::find(self.data.prefix + self.unit.prefix_bias());

        if extra == 0 {
            write!(f, "{}", prefix)?;
        } else {
            write!(f, "e{}{}", extra, prefix)?;
        }

        self.unit.format_suffix(f, self.pluralize)?;

        let mut power = (self.data.power * self.n) as u32;

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

#[derive(Clone, Copy)]
pub struct Conversion {
    /// A conversion to kelvin from a given unit.
    pub to: fn(&mut Rational),
    /// Perform a conversion from kelvin to a given unit.
    pub from: fn(&mut Rational),
}

/// The vtable for a derived unit.
pub struct DerivedVtable {
    /// Populate base powers.
    pub powers: fn(&mut Powers, i32),
    /// Format the unit.
    pub format: fn(&mut fmt::Formatter<'_>, bool) -> fmt::Result,
    /// Access conversion functions.
    pub conversion: Option<Conversion>,
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
        struct DebugFn(fn(&mut fmt::Formatter<'_>, bool) -> fmt::Result);

        impl fmt::Debug for DebugFn {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "\"")?;
                (self.0)(f, false)?;
                write!(f, "\"")?;
                Ok(())
            }
        }

        f.debug_struct("Derived")
            .field("name", &DebugFn(self.vtable.format))
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

impl Serialize for Derived {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Derived {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let id = u32::deserialize(deserializer)?;

        match crate::generated::ids::id_to_derived(id) {
            Some(derived) => Ok(derived),
            None => Err(<D::Error as de::Error>::custom(format!(
                "{} is not the identifier for a derived unit",
                id
            ))),
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
