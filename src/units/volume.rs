//! Special volume units.

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use num::BigRational;

/// Litre `l` or `0.001m^3`.
pub static LITRE: Derived = Derived {
    id: 0x1c108ba2,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, _| write!(f, "l"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1u32.into(), 1000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(1u32.into(), 1000u32.into());
            },
        }),
    },
};

/// Cubic centimetre `cc` or `1cm^3`.
pub static CUBIC_CENTIMETER: Derived = Derived {
    id: 0x1c108ba2,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, _| write!(f, "cc"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1u32.into(), 1000000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(1u32.into(), 1000000u32.into());
            },
        }),
    },
};
