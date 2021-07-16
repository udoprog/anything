//! Available derived units.

use crate::unit::{Derived, DerivedVtable, Unit};
use num::BigRational;

mod times;
pub use self::times::*;

pub mod imperial;

/// Velocity in `m*s` with the `v` suffix.
pub static VELOCITY: Derived = Derived {
    id: 0x47dd35dc,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
            powers.insert(Unit::Second, p * -1);
        },
        format: |f, _| write!(f, "v"),
        multiple_ratio: None,
    },
};

/// Acceleration in `m*s^-2` with the `a` suffix.
pub static ACCELERATION: Derived = Derived {
    id: 0x47dd35dc,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "a"),
        multiple_ratio: None,
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
        multiple_ratio: Some(|| BigRational::new(980665u32.into(), 100000u32.into())),
    },
};

/// A ton or `1000kg`.
///
/// See [Unit::KiloGram].
pub static TON: Derived = Derived {
    id: 0x7b15d4d8,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "tons")
            } else {
                write!(f, "ton")
            }
        },
        multiple_ratio: Some(|| BigRational::new(1000u32.into(), 1u32.into())),
    },
};

/// A Newton of force in `kh*m*s^-2` with the `N` suffix.
pub static NEWTON: Derived = Derived {
    id: 0x150ab031,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "N"),
        multiple_ratio: None,
    },
};

/// A pascal of pressure in `kg*m^-1*s^-2`.
pub static PASCAL: Derived = Derived {
    id: 0xd575976d,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * -1);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "Pa"),
        multiple_ratio: None,
    },
};

/// An astronomical unit in [Unit::Meter].
pub static AU: Derived = Derived {
    id: 0xc790db55,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "au"),
        multiple_ratio: Some(|| BigRational::new(149597870700u64.into(), 1u32.into())),
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
        multiple_ratio: Some(|| BigRational::new(299792458u32.into(), 1u32.into())),
    },
};

/// A Joule with the `J` suffix (`kg*m^2*s^-2`).
pub static JOULE: Derived = Derived {
    id: 0xe0796773,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "J"),
        multiple_ratio: None,
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
        multiple_ratio: Some(|| BigRational::new(1055u32.into(), 1u32.into())),
    },
};

/// Watt as `J/s` with the `W` suffix (`kg* m^2 * s^-3`).
pub static WATT: Derived = Derived {
    id: 0xa977f890,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -3);
        },
        format: |f, _| write!(f, "W"),
        multiple_ratio: None,
    },
};

/// Coulomb as `s*A` with the `C` suffix.
pub static COULOMB: Derived = Derived {
    id: 0xf57d5095,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Second, p);
            powers.insert(Unit::Ampere, p);
        },
        format: |f, _| write!(f, "C"),
        multiple_ratio: None,
    },
};

/// Volt as `W/A` with the `V` suffix (`kg * m^2 * s^-3 * A^-1`).
pub static VOLT: Derived = Derived {
    id: 0x27475ce0,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -3);
            powers.insert(Unit::Ampere, p * -1);
        },
        format: |f, _| write!(f, "V"),
        multiple_ratio: None,
    },
};

/// Farad as `C/V` with the `F` suffix (`kg^-1 * m^-2 * s^4 * A^2`).
pub static FARAD: Derived = Derived {
    id: 0xcea46875,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p * -1);
            powers.insert(Unit::Meter, p * -2);
            powers.insert(Unit::Second, p * 4);
            powers.insert(Unit::Ampere, p * 2);
        },
        format: |f, _| write!(f, "F"),
        multiple_ratio: None,
    },
};

/// Ohm as `V/A` with the `Ω` suffix (`kg * m^2 * s^-3 * A^-2`).
pub static OHM: Derived = Derived {
    id: 0x4c6815d9,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -3);
            powers.insert(Unit::Ampere, p * -2);
        },
        format: |f, _| write!(f, "Ω"),
        multiple_ratio: None,
    },
};

/// Siemens as `Ω^-1` with the `S` suffix.
pub static SIEMENS: Derived = Derived {
    id: 0xd87739a9,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p * -1);
            powers.insert(Unit::Meter, p * -2);
            powers.insert(Unit::Second, p * 3);
            powers.insert(Unit::Ampere, p * 2);
        },
        format: |f, _| write!(f, "S"),
        multiple_ratio: None,
    },
};

/// Weber as `V*s` with the `Wb` suffix (`kg * m^2 * s^-2 * A^-1`).
pub static WEBER: Derived = Derived {
    id: 0x69ca6c0a,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
            powers.insert(Unit::Ampere, p * -1);
        },
        format: |f, _| write!(f, "Wb"),
        multiple_ratio: None,
    },
};

/// Tesla as `Wb/m^2` with the `T` suffix (`kg * s^-2 * A ^ -1`).
pub static TESLA: Derived = Derived {
    id: 0x731514a7,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Second, p * -2);
            powers.insert(Unit::Ampere, p * -1);
        },
        format: |f, _| write!(f, "T"),
        multiple_ratio: None,
    },
};

/// Henry as `Wb/A` with the `H` suffix (`kg * m^2 * s^-2 * A^-2`).
pub static HENRY: Derived = Derived {
    id: 0xef26a9d5,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
            powers.insert(Unit::Ampere, p * -2);
        },
        format: |f, _| write!(f, "H"),
        multiple_ratio: None,
    },
};

/// Lumen as `cd*sr` with the `lm` suffix.
pub static LUMEN: Derived = Derived {
    id: 0x359318c2,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Candela, p);
        },
        format: |f, _| write!(f, "lm"),
        multiple_ratio: None,
    },
};

/// Lux as `lm/m^2` with the `lx` suffix.
pub static LUX: Derived = Derived {
    id: 0xad603e6d,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Candela, p);
            powers.insert(Unit::Meter, p * -2);
        },
        format: |f, _| write!(f, "lx"),
        multiple_ratio: None,
    },
};

/// Becquerel as `s^-1` with the `Bq` suffix.
pub static BECQUEREL: Derived = Derived {
    id: 0x7c25d25c,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Second, p * -1);
        },
        format: |f, _| write!(f, "Bq"),
        multiple_ratio: None,
    },
};

/// Gray as `m^2*s^-2` with the `Gy` suffix.
pub static GRAY: Derived = Derived {
    id: 0x6008fcb5,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "Gy"),
        multiple_ratio: None,
    },
};

/// Sievert as `m^2*s^-2` with the `Sv` suffix.
pub static SIEVERT: Derived = Derived {
    id: 0xcd0fdf3b,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "Sv"),
        multiple_ratio: None,
    },
};

/// Katal as `mol*s^-1` with the `kat` suffix.
pub static KATAL: Derived = Derived {
    id: 0x9645d02f,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Mole, p);
            powers.insert(Unit::Second, p * -1);
        },
        format: |f, _| write!(f, "kat"),
        multiple_ratio: None,
    },
};

/// Specific impuse as `s` with the `sp` suffix.
pub static SPECIFIC_IMPUSE: Derived = Derived {
    id: 0x9645d02f,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Second, p);
        },
        format: |f, _| write!(f, "sp"),
        multiple_ratio: None,
    },
};
