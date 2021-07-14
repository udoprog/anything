use bigdecimal::BigDecimal;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::fmt;

/// A base unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Unit {
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

impl Unit {
    pub fn populate_bases(&self, bases: &mut BTreeMap<Self, i32>, power: i32) -> bool {
        fn merge(bases: &mut BTreeMap<Unit, i32>, name: Unit, power: i32) {
            match bases.entry(name) {
                btree_map::Entry::Vacant(e) => {
                    e.insert(power);
                }
                btree_map::Entry::Occupied(mut e) => {
                    *e.get_mut() += power;
                }
            }
        }

        match self {
            Unit::Second => {
                merge(bases, Unit::Second, power);
                false
            }
            Unit::KiloGram => {
                merge(bases, Unit::KiloGram, power);
                false
            }
            Unit::Meter => {
                merge(bases, Unit::Meter, power);
                false
            }
            Unit::Byte => {
                merge(bases, Unit::Byte, power);
                false
            }
            Unit::Year => {
                merge(bases, Unit::Second, power);
                true
            }
            Unit::Month => {
                merge(bases, Unit::Second, power);
                true
            }
            Unit::Week => {
                merge(bases, Unit::Second, power);
                true
            }
            Unit::Day => {
                merge(bases, Unit::Second, power);
                true
            }
            Unit::Hour => {
                merge(bases, Unit::Second, power);
                true
            }
            Unit::Minute => {
                merge(bases, Unit::Second, power);
                true
            }
            Unit::Btu | Unit::Joule => {
                // kg⋅m2⋅s−2
                merge(bases, Unit::KiloGram, power);
                merge(bases, Unit::Meter, power * 2);
                merge(bases, Unit::Second, power * -2);
                true
            }
            Unit::Au => {
                merge(bases, Unit::Meter, power);
                true
            }
            Unit::LightSpeed => {
                merge(bases, Unit::Meter, power);
                merge(bases, Unit::Second, power * -1);
                true
            }
        }
    }

    /// The multiplication factor for this component.
    pub fn multiple(&self) -> Option<BigDecimal> {
        let m: u64 = match self {
            Unit::Year => {
                return Some(BigDecimal::new(3147113076u32.into(), 2));
            }
            Unit::Month => {
                return Some(BigDecimal::new(262259423u32.into(), 2));
            }
            Unit::Week => 604800,
            Unit::Day => 86400,
            Unit::Hour => 3600,
            Unit::Minute => 60,
            Unit::Btu => 1055,
            Unit::Au => 149597870700,
            Unit::LightSpeed => 299792458,
            _ => return None,
        };

        Some(BigDecimal::from(m))
    }

    pub(crate) fn prefix_bias(&self) -> i32 {
        match self {
            Unit::KiloGram => 3,
            _ => 0,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Second => 's'.fmt(f),
            Unit::KiloGram => 'g'.fmt(f),
            Unit::Meter => 'm'.fmt(f),
            Unit::Byte => 'B'.fmt(f),
            Unit::Joule => 'J'.fmt(f),
            Unit::Year => "Y".fmt(f),
            Unit::Month => "M".fmt(f),
            Unit::Week => "W".fmt(f),
            Unit::Day => "d".fmt(f),
            Unit::Hour => "H".fmt(f),
            Unit::Minute => "m".fmt(f),
            Unit::Btu => "btu".fmt(f),
            Unit::Au => "au".fmt(f),
            Unit::LightSpeed => "c".fmt(f),
        }
    }
}
