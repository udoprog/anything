//! All available units.

use crate::powers::Powers;
use crate::unit::{Derived, DerivedVtable, Unit};
use bigdecimal::BigDecimal;

/// Velocity in `m*s` with the `v` suffix.
pub static VELOCITY: Derived = Derived {
    id: 0x47dd35dc,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::Meter, power);
            powers.insert(Unit::Second, power * -1);
        },
        format: |f, _| write!(f, "v"),
        multiple: None,
    },
};

/// Acceleration in `m*s^-2` with the `a` suffix.
pub static ACCELERATION: Derived = Derived {
    id: 0x47dd35dc,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::Meter, power);
            powers.insert(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "a"),
        multiple: None,
    },
};

/// A `g` (`98.0665a`) in `m*s^-2`.
///
/// Since this uses the same suffix as [Unit::KiloGram] it must be
/// disambiguiated during parsing. This can be accomplished with an expression
/// like `10g as a`.
///
/// See [ACCELERATION].
pub static GFORCE: Derived = Derived {
    id: 0xb82b2151,
    vtable: &DerivedVtable {
        powers: ACCELERATION.vtable.powers,
        format: |f, _| write!(f, "g"),
        multiple: Some(|| BigDecimal::new(980665u32.into(), 5)),
    },
};

/// A ton or `1000kg`.
///
/// See [Unit::KiloGram].
pub static TON: Derived = Derived {
    id: 0x7b15d4d8,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::KiloGram, power);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "tons")
            } else {
                write!(f, "ton")
            }
        },
        multiple: Some(|| BigDecimal::from(1000)),
    },
};

/// A newton of force in `kh*m*s^-2`.
pub static NEWTON: Derived = Derived {
    id: 0x150ab031,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::KiloGram, power);
            powers.insert(Unit::Meter, power);
            powers.insert(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "N"),
        multiple: Some(|| BigDecimal::from(1000)),
    },
};

/// A pascal of pressure in `kg*m^-1*s^-2`.
pub static PASCAL: Derived = Derived {
    id: 0xd575976d,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::KiloGram, power);
            powers.insert(Unit::Meter, power * -1);
            powers.insert(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "Pa"),
        multiple: None,
    },
};

/// An astronomical unit in `m`.
///
/// See [Unit::Meter].
pub static AU: Derived = Derived {
    id: 0xc790db55,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::Meter, power);
        },
        format: |f, _| write!(f, "au"),
        multiple: Some(|| BigDecimal::from(149597870700u64)),
    },
};

/// The speed of light in `m/s`.
///
/// See [VELOCITY].
pub static LIGHTSPEED: Derived = Derived {
    id: 0x8e8393e6,
    vtable: &DerivedVtable {
        powers: VELOCITY.vtable.powers,
        format: |f, _| write!(f, "c"),
        multiple: Some(|| BigDecimal::from(299792458u32)),
    },
};

/// A joule in `kg*m^2*s^-2`.
pub static JOULE: Derived = Derived {
    id: 0xe0796773,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::KiloGram, power);
            powers.insert(Unit::Meter, power * 2);
            powers.insert(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "J"),
        multiple: None,
    },
};

/// A [British Thermal Unit] or `1055J` with the `btu` suffix.
///
/// See [JOULE].
///
/// [British Thermal Unit]: https://en.wikipedia.org/wiki/British_thermal_unit
pub static BTU: Derived = Derived {
    id: 0xcf847a94,
    vtable: &DerivedVtable {
        powers: JOULE.vtable.powers,
        format: |f, pluralize| {
            if pluralize {
                write!(f, "btus")
            } else {
                write!(f, "btu")
            }
        },
        multiple: Some(|| BigDecimal::from(1055u32)),
    },
};

/// Designated as `kg*m^2*s−3`.
pub static WATT: Derived = Derived {
    id: 0xa977f890,
    vtable: &DerivedVtable {
        powers: |powers, power| {
            powers.insert(Unit::KiloGram, power);
            powers.insert(Unit::Meter, power * 2);
            powers.insert(Unit::Second, power * -3);
        },
        format: |f, _| write!(f, "W"),
        multiple: None,
    },
};

fn time_powers(powers: &mut Powers, power: i32) {
    powers.insert(Unit::Second, power);
}

macro_rules! time {
    ($(#[$meta:meta])* pub static $name:ident = ($id:expr, $multiple:expr, $scale:expr), $f:expr) => {
        $(#[$meta])*
        pub static $name: Derived = Derived {
            id: $id,
            vtable: &DerivedVtable {
                powers: time_powers,
                format: $f,
                multiple: Some(|| BigDecimal::new($multiple.into(), $scale)),
            },
        };
    };
}

time! {
    /// A minute `m` (`60s`) in [Unit::Second].
    pub static MINUTE = (0x3cea90d3, 60, 0), |f, _| f.write_str("mn")
}
time! {
    /// An hour `H` (`3600s`) in [Unit::Second].
    pub static HOUR = (0x8884f852, 3600, 0), |f, _| f.write_str("H")
}
time! {
    /// A day `dy` (`86400s`) in [Unit::Second].
    pub static DAY = (0xdacd8d53, 86400, 0), |f, _| f.write_str("dy")
}
time! {
    /// A week `wk` (`604800s`) in [Unit::Second].
    pub static WEEK = (0xd6d4f93f, 604800, 0), |f, _| f.write_str("wk")
}
time! {
    /// A month in [Unit::Second] defined as `1/12` of [YEAR].
    pub static MONTH = (0x458a3642, 262259423u32, 2), |f, _| f.write_str("mth")
}
time! {
    /// A year `yr` (`31471130.76s`) in [Unit::Second].
    pub static YEAR = (0xe923ce05, 3147113076u32, 2), |f, _| f.write_str("yr")
}
time! {
    /// A Century (`10yr`) in [Unit::Second] defined as `10` times [YEAR].
    pub static DECADE = (0xbed4a84b, 3147113076u32, 1), |f, pluralize| if pluralize {
        f.write_str("decades")
    } else {
        f.write_str("decade")
    }
}
time! {
    /// A Century (`100yr`) in [Unit::Second] defined as `100` times [YEAR].
    pub static CENTURY = (0x8efe5bbc, 3147113076u32, 0), |f, pluralize| if pluralize {
        f.write_str("centuries")
    } else {
        f.write_str("century")
    }
}
time! {
    /// A Millenium (`1000yr`) in [Unit::Second] defined as `1000` times [YEAR].
    pub static MILLENIUM = (0x0d2818da, 3147113076u32, -1), |f, pluralize| if pluralize {
        f.write_str("millenia")
    } else {
        f.write_str("millenium")
    }
}
