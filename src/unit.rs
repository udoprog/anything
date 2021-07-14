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

    /// Acceleration or `m/s^2`.
    Acceleration,
    /// A g or ~ `9.8a`.
    Gforce,
    /// A ton or `1000kg`.
    Ton,
    // Derived units
    /// A Joule `kg*m^2*s^-2`.
    Joule,
    /// `Y` or `(3600 * 24 * 365)s`.
    Year,
    /// A decade, or 10 years.
    Decade,
    /// A century, or 100 years.
    Century,
    /// A milennium, or 1000 years.
    Millenium,
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
            Unit::Ampere => {
                merge(bases, Unit::Ampere, power);
                false
            }
            Unit::Kelvin => {
                merge(bases, Unit::Kelvin, power);
                false
            }
            Unit::Mole => {
                merge(bases, Unit::Mole, power);
                false
            }
            Unit::Candela => {
                merge(bases, Unit::Candela, power);
                false
            }
            Unit::Byte => {
                merge(bases, Unit::Byte, power);
                false
            }
            Unit::Year | Unit::Decade | Unit::Century | Unit::Millenium => {
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
            Unit::Acceleration | Unit::Gforce => {
                merge(bases, Unit::Meter, power);
                merge(bases, Unit::Second, power * -2);
                true
            }
            Unit::Ton => {
                merge(bases, Unit::KiloGram, power);
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
            Unit::Decade => {
                return Some(BigDecimal::new(3147113076u32.into(), 1));
            }
            Unit::Century => {
                return Some(BigDecimal::new(3147113076u32.into(), 0));
            }
            Unit::Millenium => {
                return Some(BigDecimal::new(3147113076u32.into(), -1));
            }
            Unit::Month => {
                return Some(BigDecimal::new(262259423u32.into(), 2));
            }
            Unit::Week => 604800,
            Unit::Day => 86400,
            Unit::Hour => 3600,
            Unit::Minute => 60,
            Unit::Gforce => {
                return Some(BigDecimal::new(980665u32.into(), 5));
            }
            Unit::Ton => 1000,
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

    pub(crate) fn format(&self, f: &mut fmt::Formatter<'_>, pluralize: bool) -> fmt::Result {
        use std::fmt::Display as _;

        match (self, pluralize) {
            (Unit::Second, _) => 's'.fmt(f),
            (Unit::KiloGram, _) => 'g'.fmt(f),
            (Unit::Meter, _) => 'm'.fmt(f),
            (Unit::Ampere, _) => 'A'.fmt(f),
            (Unit::Kelvin, _) => 'K'.fmt(f),
            (Unit::Mole, _) => "mol".fmt(f),
            (Unit::Candela, _) => "cd".fmt(f),
            (Unit::Byte, _) => 'B'.fmt(f),
            (Unit::Acceleration, _) => 'a'.fmt(f),
            (Unit::Gforce, _) => 'g'.fmt(f),
            (Unit::Ton, false) => "ton".fmt(f),
            (Unit::Ton, true) => "Tons".fmt(f),
            (Unit::Joule, _) => 'J'.fmt(f),
            (Unit::Year, _) => "yr".fmt(f),
            (Unit::Decade, false) => "decade".fmt(f),
            (Unit::Decade, true) => "decades".fmt(f),
            (Unit::Century, false) => "century".fmt(f),
            (Unit::Century, true) => "centuries".fmt(f),
            (Unit::Millenium, false) => "millenium".fmt(f),
            (Unit::Millenium, true) => "millenia".fmt(f),
            (Unit::Month, _) => "mth".fmt(f),
            (Unit::Week, _) => "W".fmt(f),
            (Unit::Day, _) => "d".fmt(f),
            (Unit::Hour, _) => "H".fmt(f),
            (Unit::Minute, _) => "m".fmt(f),
            (Unit::Btu, false) => "btu".fmt(f),
            (Unit::Btu, true) => "btus".fmt(f),
            (Unit::Au, _) => "au".fmt(f),
            (Unit::LightSpeed, _) => "c".fmt(f),
        }
    }
}
