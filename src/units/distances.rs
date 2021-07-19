//! Special distance units.

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use num::BigRational;

/// An astronomical unit (`au`) in [Unit::Meter].
pub static AU: Derived = Derived {
    id: 0xc790db55,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "au"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(149597870700u64.into(), 1u32.into());
            },
            from: |num| {
                *num /= BigRational::new(149597870700u64.into(), 1u32.into());
            },
        }),
    },
};

/// Nautical mile (`NM`) as `m` with the `NM` suffix.
pub static NAUTICAL_MILE: Derived = Derived {
    id: 0xd767fd82,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "NM"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1852.into(), 1.into());
            },
            from: |num| {
                *num /= BigRational::new(1852.into(), 1.into());
            },
        }),
    },
};
