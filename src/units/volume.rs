//! Special volume units.

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use rational::Rational;

/// Litre `l` or `0.001m^3`.
pub static LITRE: Derived = Derived {
    id: crate::generated::ids::LITRE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, _| write!(f, "l"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1u32, 1000u32);
            },
            from: |num| {
                *num /= Rational::new(1u32, 1000u32);
            },
        }),
    },
};

/// Cubic centimetre `cc` or `1cm^3`.
pub static CUBIC_CENTIMETER: Derived = Derived {
    id: crate::generated::ids::CUBIC_CENTIMETER,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, _| write!(f, "cc"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1u32, 1000000u32);
            },
            from: |num| {
                *num /= Rational::new(1u32, 1000000u32);
            },
        }),
    },
};
