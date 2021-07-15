//! All available units.

use crate::unit::{Derived, Powers, Unit, UnitVtable};
use bigdecimal::BigDecimal;

/// Acceleration.
pub static ACCELERATION: Derived = Derived {
    id: 0x47dd35dc,
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::Meter, power);
            powers.add(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "a"),
        multiple: None,
    },
};

/// A g or ~ `9.8a`.
pub static GFORCE: Derived = Derived {
    id: 0xb82b2151,
    vtable: &UnitVtable {
        powers: ACCELERATION.vtable.powers,
        format: |f, _| write!(f, "g"),
        multiple: Some(|| BigDecimal::new(980665u32.into(), 5)),
    },
};

/// A ton or `1000kg`.
pub static TON: Derived = Derived {
    id: 0x7b15d4d8,
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::KiloGram, power);
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

/// Force as `kh*m*s^-2`.
pub static NEWTON: Derived = Derived {
    id: 0x150ab031,
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::KiloGram, power);
            powers.add(Unit::Meter, power);
            powers.add(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "N"),
        multiple: Some(|| BigDecimal::from(1000)),
    },
};

/// Pressure as `kg*m^-1*s^-2`.
pub static PASCAL: Derived = Derived {
    id: 0xd575976d,
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::KiloGram, power);
            powers.add(Unit::Meter, power * -1);
            powers.add(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "Pa"),
        multiple: None,
    },
};

/// An astronomical unit.
pub static AU: Derived = Derived {
    id: 0xc790db55,
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::Meter, power);
        },
        format: |f, _| write!(f, "au"),
        multiple: Some(|| BigDecimal::from(149597870700u64)),
    },
};

/// The speed of light.
pub static LIGHTSPEED: Derived = Derived {
    id: 0x8e8393e6,
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::Meter, power);
            powers.add(Unit::Second, power * -1);
        },
        format: |f, _| write!(f, "c"),
        multiple: Some(|| BigDecimal::from(299792458u32)),
    },
};

/// A Joule `kg*m^2*s^-2`.
pub static JOULE: Derived = Derived {
    id: 0xe0796773,
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::KiloGram, power);
            powers.add(Unit::Meter, power * 2);
            powers.add(Unit::Second, power * -2);
        },
        format: |f, _| write!(f, "J"),
        multiple: None,
    },
};

/// A British Thermal Unit, or `1055J`.
pub static BTU: Derived = Derived {
    id: 0xcf847a94,
    vtable: &UnitVtable {
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
    vtable: &UnitVtable {
        powers: |powers, power| {
            powers.add(Unit::KiloGram, power);
            powers.add(Unit::Meter, power * 2);
            powers.add(Unit::Second, power * -3);
        },
        format: |f, _| write!(f, "W"),
        multiple: None,
    },
};

static TIME_POWERS: fn(&mut Powers, i32) = |powers, power| {
    powers.add(Unit::Second, power);
};

macro_rules! time {
    ($(#[$meta:meta])* pub static $name:ident = ($id:expr, $multiple:expr, $scale:expr), $f:expr) => {
        $(#[$meta])*
        pub static $name: Derived = Derived {
            id: $id,
            vtable: &UnitVtable {
                powers: TIME_POWERS,
                format: $f,
                multiple: Some(|| BigDecimal::new($multiple.into(), $scale)),
            },
        };
    };
}

time! {
    /// `m` or `60s`.
    pub static MINUTE = (0x3cea90d3, 60, 0), |f, _| f.write_str("mn")
}
time! {
    /// `H` or `3600s`.
    pub static HOUR = (0x8884f852, 3600, 0), |f, _| f.write_str("H")
}
time! {
    /// `d` or `86400s`.
    pub static DAY = (0xdacd8d53, 86400, 0), |f, _| f.write_str("dy")
}
time! {
    /// `d` or `(3600 * 24 * 7)s`.
    pub static WEEK = (0xd6d4f93f, 604800, 0), |f, _| f.write_str("wk")
}
time! {
    /// A month.
    pub static MONTH = (0x458a3642, 262259423u32, 2), |f, _| f.write_str("mth")
}
time! {
    /// A year.
    pub static YEAR = (0xe923ce05, 3147113076u32, 2), |f, _| f.write_str("yr")
}
time! {
    /// A decade, or 10 years.
    pub static DECADE = (0xbed4a84b, 3147113076u32, 1), |f, pluralize| if pluralize {
        f.write_str("decades")
    } else {
        f.write_str("decade")
    }
}
time! {
    /// A century.
    pub static CENTURY = (0x8efe5bbc, 3147113076u32, 0), |f, pluralize| if pluralize {
        f.write_str("centuries")
    } else {
        f.write_str("century")
    }
}
time! {
    /// A milennium, or 1000 years.
    pub static MILLENIUM = (0x0d2818da, 3147113076u32, -1), |f, pluralize| if pluralize {
        f.write_str("millenia")
    } else {
        f.write_str("millenium")
    }
}
