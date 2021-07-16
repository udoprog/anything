//! Special velocity units.

use crate::unit::{Derived, DerivedVtable};
use num::BigRational;

/// The speed of light in `m/s`.
///
/// See [VELOCITY].
pub static LIGHT_SPEED: Derived = Derived {
    id: 0x8e8393e6,
    vtable: &DerivedVtable {
        powers: crate::units::VELOCITY.vtable.powers,
        format: |f, _| write!(f, "c"),
        multiple_ratio: Some(|| BigRational::new(299792458u32.into(), 1u32.into())),
    },
};

/// Knot (`kt`) as `m/s`.
pub static KNOT: Derived = Derived {
    id: 0xc8545958,
    vtable: &DerivedVtable {
        powers: crate::units::VELOCITY.vtable.powers,
        format: |f, _| write!(f, "kt"),
        multiple_ratio: Some(|| BigRational::new(1852.into(), 3600.into())),
    },
};
