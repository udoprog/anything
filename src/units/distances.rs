use crate::{
    unit::{Derived, DerivedVtable},
    Unit,
};
use num::BigRational;

/// Inches `in` (`0.0254m`).
pub static INCH: Derived = Derived {
    id: 0xd3c90000,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "in"),
        multiple_ratio: Some(|| BigRational::new(254u32.into(), 10000u32.into())),
    },
};

/// Feet `ft` (`0.3048m`).
pub static FEET: Derived = Derived {
    id: 0xd3c90001,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "ft"),
        multiple_ratio: Some(|| BigRational::new(3048u32.into(), 10000u32.into())),
    },
};

/// Yards `yd` (`0.9144m`).
pub static YARD: Derived = Derived {
    id: 0xd3c90002,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "yd"),
        multiple_ratio: Some(|| BigRational::new(9144u32.into(), 10000u32.into())),
    },
};

/// Mile `mi` (`1609.344m`).
pub static MILE: Derived = Derived {
    id: 0xd3c90003,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "mi"),
        multiple_ratio: Some(|| BigRational::new(1609344u32.into(), 1000u32.into())),
    },
};
