//! Energy units.

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use num::BigRational;

/// A Joule with the `J` suffix (`kg*m^2*s^-2`).
pub static JOULE: Derived = Derived {
    id: 0xe0796773,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
            powers.insert(Unit::Meter, p * 2);
            powers.insert(Unit::Second, p * -2);
        },
        format: |f, _| write!(f, "J"),
        conversion: None,
    },
};

/// A [British Thermal Unit] or `1055J` with the `btu` suffix.
///
/// See [JOULE].
///
/// [British Thermal Unit]: https://en.wikipedia.org/wiki/British_thermal_unit
pub static BTU: Derived = Derived {
    id: 0xcf847a94,
    vtable: &DerivedVtable {
        powers: JOULE.vtable.powers,
        format: |f, pluralize| {
            if pluralize {
                write!(f, "btus")
            } else {
                write!(f, "btu")
            }
        },
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1055u32.into(), 1u32.into());
            },
            from: |num| {
                *num /= BigRational::new(1055u32.into(), 1u32.into());
            },
        }),
    },
};

/// Electronvolt `eV`.
pub static ELECTRONVOLT: Derived = Derived {
    id: 0x007adc81,
    vtable: &DerivedVtable {
        powers: JOULE.vtable.powers,
        format: |f, _| write!(f, "eV"),
        conversion: Some(Conversion {
            to: |num| {
                *num *=
                    BigRational::new(801088317u32.into(), 5000000000000000000000000000u128.into());
            },
            from: |num| {
                *num /=
                    BigRational::new(801088317u32.into(), 5000000000000000000000000000u128.into());
            },
        }),
    },
};
