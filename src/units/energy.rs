//! Energy units.

use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable, Unit};

/// A Joule with the `J` suffix (`kg*m^2*s^-2`).
pub static JOULE: Derived = Derived {
    id: crate::generated::ids::JOULE,
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
    id: crate::generated::ids::BTU,
    vtable: &DerivedVtable {
        powers: JOULE.vtable.powers,
        format: |f, pluralize| {
            if pluralize {
                write!(f, "btus")
            } else {
                write!(f, "btu")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1055,
            denom: 1,
        })),
    },
};

/// Electronvolt `eV`.
pub static ELECTRONVOLT: Derived = Derived {
    id: crate::generated::ids::ELECTRONVOLT,
    vtable: &DerivedVtable {
        powers: JOULE.vtable.powers,
        format: |f, _| write!(f, "eV"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 801088317,
            denom: 5000000000000000000000000000,
        })),
    },
};
