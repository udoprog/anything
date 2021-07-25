//! Special velocity units.

use crate::unit::{Conversion, Derived, DerivedVtable};
use rational::Rational;

/// The speed of light in `m/s`.
///
/// See [VELOCITY].
pub static LIGHT_SPEED: Derived = Derived {
    id: crate::generated::ids::LIGHT_SPEED,
    vtable: &DerivedVtable {
        powers: crate::units::VELOCITY.vtable.powers,
        format: |f, _| write!(f, "c"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(299792458u32, 1u32);
            },
            from: |num| {
                *num /= Rational::new(299792458u32, 1u32);
            },
        }),
    },
};

/// Knot (`kt`) as `m/s`.
pub static KNOT: Derived = Derived {
    id: crate::generated::ids::KNOT,
    vtable: &DerivedVtable {
        powers: crate::units::VELOCITY.vtable.powers,
        format: |f, _| write!(f, "kt"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1852, 3600u32);
            },
            from: |num| {
                *num /= Rational::new(1852, 3600u32);
            },
        }),
    },
};
