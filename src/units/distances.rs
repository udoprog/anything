//! Special distance units.

use crate::unit::{Derived, DerivedVtable, Unit};
use num::BigRational;

/// An astronomical unit (`au`) in [Unit::Meter].
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

/// Nautical mile (`NM`) as `m` with the `NM` suffix.
pub static NAUTICAL_MILE: Derived = Derived {
    id: 0xd767fd82,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "NM"),
        multiple_ratio: Some(|| BigRational::new(1852.into(), 1.into())),
    },
};
