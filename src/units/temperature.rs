//! Special temperature units (separate from Kelvin).

use crate::unit::{
    Conversion, ConversionFraction, ConversionMethods, Derived, DerivedVtable, Unit,
};
use rational::Rational;

/// Celsius (`°C`) in based on kelvin (`K`).
pub static CELSIUS: Derived = Derived {
    id: crate::generated::ids::CELSIUS,
    vtable: &DerivedVtable {
        powers: |powers, n| {
            powers.insert(Unit::Kelvin, n);
        },
        format: |f, _| write!(f, "°C"),
        conversion: Some(Conversion::Offset(ConversionFraction {
            numer: 27315,
            denom: 100,
        })),
    },
};

/// Fahrenheit (`°F`) in based on kelvin (`K`).
pub static FAHRENHEIT: Derived = Derived {
    id: crate::generated::ids::FAHRENHEIT,
    vtable: &DerivedVtable {
        powers: |powers, n| {
            powers.insert(Unit::Kelvin, n);
        },
        format: |f, _| write!(f, "°F"),
        conversion: Some(Conversion::Methods(ConversionMethods {
            to: |num| {
                *num -= Rational::new(32, 1);
                *num *= Rational::new(5, 9);
                *num += Rational::new(27315, 100);
            },
            from: |num| {
                *num -= Rational::new(27315, 100);
                *num *= Rational::new(9, 5);
                *num += Rational::new(32, 1);
            },
        })),
    },
};
