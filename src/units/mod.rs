//! Available derived units.

#![allow(clippy::neg_multiply)]

use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable, Unit};

pub mod area;
pub mod energy;
pub mod length;
pub mod mass;
pub mod temperature;
pub mod time;
pub mod velocity;
pub mod volume;

/// Velocity in `m*s` with the `v` suffix.
pub static VELOCITY: Derived = Derived {
    id: crate::generated::ids::VELOCITY,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
            powers.insert(Unit::Second, p * -1);
        },
        format: |f, _| write!(f, "v"),
        conversion: None,
    },
};

/// Acceleration in `m*s^-2` with the `a` suffix.
pub static ACCELERATION: Derived = Derived {
    id: crate::generated::ids::ACCELERATION,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "a"),
        conversion: None,
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
    id: crate::generated::ids::GFORCE,
    vtable: &DerivedVtable {
        powers: ACCELERATION.vtable.powers,
        format: |f, _| write!(f, "g"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 980665,
            denom: 100000,
        })),
    },
};

/// A Newton of force in `kh*m*s^-2` with the `N` suffix.
pub static NEWTON: Derived = Derived {
    id: crate::generated::ids::NEWTON,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "N"),
        conversion: None,
    },
};

/// A pascal of pressure in `kg*m^-1*s^-2`.
pub static PASCAL: Derived = Derived {
    id: crate::generated::ids::PASCAL,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * -1);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "Pa"),
        conversion: None,
    },
};

/// Watt as `J/s` with the `W` suffix (`kg* m^2 * s^-3`).
pub static WATT: Derived = Derived {
    id: crate::generated::ids::WATT,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -3);
        },
        format: |f, _| write!(f, "W"),
        conversion: None,
    },
};

/// Coulomb as `s*A` with the `C` suffix.
pub static COULOMB: Derived = Derived {
    id: crate::generated::ids::COULOMB,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Second, p);
            powers.insert(Unit::Ampere, p);
        },
        format: |f, _| write!(f, "C"),
        conversion: None,
    },
};

/// Volt as `W/A` with the `V` suffix (`kg * m^2 * s^-3 * A^-1`).
pub static VOLT: Derived = Derived {
    id: crate::generated::ids::VOLT,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -3);
            powers.insert(Unit::Ampere, p * -1);
        },
        format: |f, _| write!(f, "V"),
        conversion: None,
    },
};

/// Farad as `C/V` with the `F` suffix (`kg^-1 * m^-2 * s^4 * A^2`).
pub static FARAD: Derived = Derived {
    id: crate::generated::ids::FARAD,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p * -1);
            powers.insert(Unit::Meter, p * -2);
            powers.insert(Unit::Second, p * 4);
            powers.insert(Unit::Ampere, p * 2);
        },
        format: |f, _| write!(f, "F"),
        conversion: None,
    },
};

/// Ohm as `V/A` with the `Ω` suffix (`kg * m^2 * s^-3 * A^-2`).
pub static OHM: Derived = Derived {
    id: crate::generated::ids::OHM,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -3);
            powers.insert(Unit::Ampere, p * -2);
        },
        format: |f, _| write!(f, "Ω"),
        conversion: None,
    },
};

/// Siemens as `Ω^-1` with the `S` suffix.
pub static SIEMENS: Derived = Derived {
    id: crate::generated::ids::SIEMENS,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p * -1);
            powers.insert(Unit::Meter, p * -2);
            powers.insert(Unit::Second, p * 3);
            powers.insert(Unit::Ampere, p * 2);
        },
        format: |f, _| write!(f, "S"),
        conversion: None,
    },
};

/// Weber as `V*s` with the `Wb` suffix (`kg * m^2 * s^-2 * A^-1`).
pub static WEBER: Derived = Derived {
    id: crate::generated::ids::WEBER,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
            powers.insert(Unit::Ampere, p * -1);
        },
        format: |f, _| write!(f, "Wb"),
        conversion: None,
    },
};

/// Tesla as `Wb/m^2` with the `T` suffix (`kg * s^-2 * A ^ -1`).
pub static TESLA: Derived = Derived {
    id: crate::generated::ids::TESLA,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Second, p * -2);
            powers.insert(Unit::Ampere, p * -1);
        },
        format: |f, _| write!(f, "T"),
        conversion: None,
    },
};

/// Henry as `Wb/A` with the `H` suffix (`kg * m^2 * s^-2 * A^-2`).
pub static HENRY: Derived = Derived {
    id: crate::generated::ids::HENRY,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
            powers.insert(Unit::Ampere, p * -2);
        },
        format: |f, _| write!(f, "H"),
        conversion: None,
    },
};

/// Lumen as `cd*sr` with the `lm` suffix.
pub static LUMEN: Derived = Derived {
    id: crate::generated::ids::LUMEN,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Candela, p);
        },
        format: |f, _| write!(f, "lm"),
        conversion: None,
    },
};

/// Lux as `lm/m^2` with the `lx` suffix.
pub static LUX: Derived = Derived {
    id: crate::generated::ids::LUX,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Candela, p);
            powers.insert(Unit::Meter, p * -2);
        },
        format: |f, _| write!(f, "lx"),
        conversion: None,
    },
};

/// Becquerel as `s^-1` with the `Bq` suffix.
pub static BECQUEREL: Derived = Derived {
    id: crate::generated::ids::BECQUEREL,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Second, p * -1);
        },
        format: |f, _| write!(f, "Bq"),
        conversion: None,
    },
};

/// Gray as `m^2*s^-2` with the `Gy` suffix.
pub static GRAY: Derived = Derived {
    id: crate::generated::ids::GRAY,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "Gy"),
        conversion: None,
    },
};

/// Sievert as `m^2*s^-2` with the `Sv` suffix.
pub static SIEVERT: Derived = Derived {
    id: crate::generated::ids::SIEVERT,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "Sv"),
        conversion: None,
    },
};

/// Katal as `mol*s^-1` with the `kat` suffix.
pub static KATAL: Derived = Derived {
    id: crate::generated::ids::KATAL,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Mole, p);
            powers.insert(Unit::Second, p * -1);
        },
        format: |f, _| write!(f, "kat"),
        conversion: None,
    },
};

/// Specific impulse as `s` with the `sp` suffix.
pub static SPECIFIC_IMPULSE: Derived = Derived {
    id: crate::generated::ids::SPECIFIC_IMPULSE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Second, p);
        },
        format: |f, _| write!(f, "sp"),
        conversion: None,
    },
};
