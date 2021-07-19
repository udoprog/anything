//! Special area units.

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use num::BigRational;

/// Hectare `ha` or `10000m^2`.
pub static HECTARE: Derived = Derived {
    id: 0xbf2e000f,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 2);
        },
        format: |f, _| write!(f, "ha"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(10000u32.into(), 1u32.into());
            },
            from: |num| {
                *num /= BigRational::new(10000u32.into(), 1u32.into());
            },
        }),
    },
};

/// Perch `perch` (`1rd^2`) or `25.29285264m^2`.
pub static PERCH: Derived = Derived {
    id: 0xf153d092,
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(2529285264u64.into(), 100000000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(2529285264u64.into(), 100000000u32.into());
            },
        }),
    },
};

/// Rood `rood` (`1fur * 1rd`) or `1011.7141056m^2`.
pub static ROOD: Derived = Derived {
    id: 0x20541ce3,
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(10117141056u64.into(), 10000000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(10117141056u64.into(), 10000000u32.into());
            },
        }),
    },
};

/// Acre `acre` (`1fur * 1chain`) or `4046.8564224m^2`.
pub static ACRE: Derived = Derived {
    id: 0xe44777d2,
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(40468564224u64.into(), 10000000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(40468564224u64.into(), 10000000u32.into());
            },
        }),
    },
};
