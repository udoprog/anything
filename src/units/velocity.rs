//! Special velocity units.

use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable};

/// The speed of light in `m/s`.
///
/// See [VELOCITY].
pub static LIGHT_SPEED: Derived = Derived {
    id: crate::generated::ids::LIGHT_SPEED,
    vtable: &DerivedVtable {
        powers: crate::units::VELOCITY.vtable.powers,
        format: |f, _| write!(f, "c"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 299792458,
            denom: 1,
        })),
    },
};

/// Knot (`kt`) as `m/s`.
pub static KNOT: Derived = Derived {
    id: crate::generated::ids::KNOT,
    vtable: &DerivedVtable {
        powers: crate::units::VELOCITY.vtable.powers,
        format: |f, _| write!(f, "kt"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1852,
            denom: 3600,
        })),
    },
};
