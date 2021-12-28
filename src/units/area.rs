//! Special area units.

use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable, Unit};

/// Hectare `ha` or `10000m^2`.
pub static HECTARE: Derived = Derived {
    id: crate::generated::ids::HECTARE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
        },
        format: |f, _| write!(f, "ha"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 10000,
            denom: 1,
        })),
    },
};

/// Perch `perch` (`1rd^2`) or `25.29285264m^2`.
pub static PERCH: Derived = Derived {
    id: crate::generated::ids::PERCH,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "perches")
            } else {
                write!(f, "perch")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 2529285264,
            denom: 100000000,
        })),
    },
};

/// Rood `rood` (`1fur * 1rd`) or `1011.7141056m^2`.
pub static ROOD: Derived = Derived {
    id: crate::generated::ids::ROOD,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "roods")
            } else {
                write!(f, "rood")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 10117141056,
            denom: 10000000,
        })),
    },
};

/// Acre `acre` (`1fur * 1chain`) or `4046.8564224m^2`.
pub static ACRE: Derived = Derived {
    id: crate::generated::ids::ACRE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "acres")
            } else {
                write!(f, "acre")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 40468564224,
            denom: 10000000,
        })),
    },
};
