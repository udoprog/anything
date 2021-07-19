//! Special temperature units (separate from Kelvin).

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use num::BigRational;

/// Celsius (`°C`) in based on kelvin (`K`).
pub static CELSIUS: Derived = Derived {
    id: 0xde39ff06,
    vtable: &DerivedVtable {
        powers: |powers, n| {
            powers.insert(Unit::Kelvin, n);
        },
        format: |f, _| write!(f, "°C"),
        conversion: Some(Conversion {
            to: |num| {
                *num += BigRational::new(27315.into(), 100.into());
            },
            from: |num| {
                *num -= BigRational::new(27315.into(), 100.into());
            },
        }),
    },
};

/// Fahrenheit (`°F`) in based on kelvin (`K`).
pub static FAHRENHEIT: Derived = Derived {
    id: 0x3a824baa,
    vtable: &DerivedVtable {
        powers: |powers, n| {
            powers.insert(Unit::Kelvin, n);
        },
        format: |f, _| write!(f, "°F"),
        conversion: Some(Conversion {
            to: |num| {
                *num -= BigRational::new(32.into(), 1.into());
                *num *= BigRational::new(5.into(), 9.into());
                *num += BigRational::new(27315.into(), 100.into());
            },
            from: |num| {
                *num -= BigRational::new(27315.into(), 100.into());
                *num *= BigRational::new(9.into(), 5.into());
                *num += BigRational::new(32.into(), 1.into());
            },
        }),
    },
};
