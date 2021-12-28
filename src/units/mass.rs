//! Special mass units.

use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable, Unit};

/// Tonne (metric ton) or `1000kg`.
pub static TONNE: Derived = Derived {
    id: crate::generated::ids::TONNE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "tons")
            } else {
                write!(f, "ton")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1000,
            denom: 1,
        })),
    },
};

/// Dalton.
pub static DALTON: Derived = Derived {
    id: crate::generated::ids::DALTON,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "Da"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 332107813321,
            denom: 200000000000,
        })),
    },
};

/// Grain `gr` (`1⁄7000lb`).
pub static GRAIN: Derived = Derived {
    id: crate::generated::ids::GRAIN,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "gr"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 6479891,
            denom: 100000000000,
        })),
    },
};

/// Drachm `dr` (`1⁄256lb`).
pub static DRACHM: Derived = Derived {
    id: crate::generated::ids::DRACHM,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "dr"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 17718451953125,
            denom: 10000000000000000,
        })),
    },
};

/// Ounce `oz` (`1⁄16lb`) (`0.028349523125kg`).
pub static OUNCE: Derived = Derived {
    id: crate::generated::ids::OUNCE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "oz"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 28349523125,
            denom: 1000000000000,
        })),
    },
};

/// Pound `lb` (`16oz`) or (`0.45359237kg`).
pub static POUND: Derived = Derived {
    id: crate::generated::ids::POUND,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "lb"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 45359237,
            denom: 100000000,
        })),
    },
};

/// Stone `st` (`14lb`) or (`6.35029318kg`).
pub static STONE: Derived = Derived {
    id: crate::generated::ids::STONE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "st"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 635029318,
            denom: 100000000,
        })),
    },
};

/// Stone `qr` (`28lb`) or (`12.70058636kg`).
pub static QUARTER: Derived = Derived {
    id: crate::generated::ids::QUARTER,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "qr"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1270058636,
            denom: 100000000,
        })),
    },
};

/// Hundredweight `hundredweight` (`4qr`) or (`50.80234544kg`).
pub static HUNDREDWEIGHT: Derived = Derived {
    id: crate::generated::ids::HUNDREDWEIGHT,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "hundredweight"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 5080234544,
            denom: 100000000,
        })),
    },
};

/// Ton `t` (`20hundredweights`) or (`1016.0469088kg`).
pub static TON: Derived = Derived {
    id: crate::generated::ids::TON,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "t"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 10160469088,
            denom: 10000000,
        })),
    },
};

/// Slug `slug` or (`14.59390294kg`).
pub static SLUG: Derived = Derived {
    id: crate::generated::ids::SLUG,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::KiloGram, p);
        },
        format: |f, _| write!(f, "slug"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1459390294,
            denom: 100000000,
        })),
    },
};
