//! Imperial weights.
//!
//! See [Imperial units on Wikipedia](https://en.wikipedia.org/wiki/Imperial_units).

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use num::BigRational;

/// Grain `gr` (`1⁄7000lb`).
pub static GRAIN: Derived = Derived {
    id: 0xf4321939,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "gr"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(6479891u32.into(), 100000000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(6479891u32.into(), 100000000000u64.into());
            },
        }),
    },
};

/// Drachm `dr` (`1⁄256lb`).
pub static DRACHM: Derived = Derived {
    id: 0xa3592b8c,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "dr"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(17718451953125u64.into(), 10000000000000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(17718451953125u64.into(), 10000000000000000u64.into());
            },
        }),
    },
};

/// Ounce `oz` (`1⁄16lb`) (`0.028349523125kg`).
pub static OUNCE: Derived = Derived {
    id: 0x7c3b47da,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "oz"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(28349523125u64.into(), 1000000000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(28349523125u64.into(), 1000000000000u64.into());
            },
        }),
    },
};

/// Pound `lb` (`16oz`) or (`0.45359237kg`).
pub static POUND: Derived = Derived {
    id: 0xe0482a36,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "lb"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(45359237u32.into(), 100000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(45359237u32.into(), 100000000u64.into());
            },
        }),
    },
};

/// Stone `st` (`14lb`) or (`6.35029318kg`).
pub static STONE: Derived = Derived {
    id: 0xc827fd0d,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "st"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(635029318u64.into(), 100000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(635029318u64.into(), 100000000u64.into());
            },
        }),
    },
};

/// Stone `qr` (`28lb`) or (`12.70058636kg`).
pub static QUARTER: Derived = Derived {
    id: 0x20f6787b,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "qr"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1270058636u64.into(), 100000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(1270058636u64.into(), 100000000u64.into());
            },
        }),
    },
};

/// Hundredweight `hundredweight` (`4qr`) or (`50.80234544kg`).
pub static HUNDREDWEIGHT: Derived = Derived {
    id: 0xf97a5980,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "hundredweight"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(5080234544u64.into(), 100000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(5080234544u64.into(), 100000000u64.into());
            },
        }),
    },
};

/// Ton `t` (`20hundredweights`) or (`1016.0469088kg`).
pub static TON: Derived = Derived {
    id: 0xccbb6466,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "t"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(10160469088u64.into(), 10000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(10160469088u64.into(), 10000000u64.into());
            },
        }),
    },
};

/// Slug `slug` or (`14.59390294kg`).
pub static SLUG: Derived = Derived {
    id: 0x28eaf41b,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "slug"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1459390294u64.into(), 100000000u64.into());
            },
            from: |num| {
                *num /= BigRational::new(1459390294u64.into(), 100000000u64.into());
            },
        }),
    },
};
