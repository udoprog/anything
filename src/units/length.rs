//! Special distance units.

use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable, Unit};

/// Astronomical unit (`au`) or `149597870700m`.
pub static AU: Derived = Derived {
    id: crate::generated::ids::AU,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "au"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 149597870700,
            denom: 1,
        })),
    },
};

/// Fathom (`ftm`) or `1.852m`.
pub static FATHOM: Derived = Derived {
    id: crate::generated::ids::FATHOM,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "ftm"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1852,
            denom: 1000,
        })),
    },
};

/// Cable (`ftm`) or `1.852m`.
pub static CABLE: Derived = Derived {
    id: crate::generated::ids::CABLE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "cables")
            } else {
                write!(f, "cable")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1852,
            denom: 10,
        })),
    },
};

/// Nautical mile (`NM`) or `1852m`.
pub static NAUTICAL_MILE: Derived = Derived {
    id: crate::generated::ids::NAUTICAL_MILE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "NM"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1852,
            denom: 1,
        })),
    },
};

/// Link (`link`) or `0.201168m`.
pub static LINK: Derived = Derived {
    id: crate::generated::ids::LINK,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, pluralize| {
            if pluralize {
                write!(f, "links")
            } else {
                write!(f, "link")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 201168,
            denom: 1000000,
        })),
    },
};

/// Rod (`rod`) or `5.0292m`.
pub static ROD: Derived = Derived {
    id: crate::generated::ids::ROD,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "rd"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 50292,
            denom: 10000,
        })),
    },
};

/// Thou `thou` (`1⁄12000ft`).
pub static THOU: Derived = Derived {
    id: crate::generated::ids::THOU,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "th"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 254,
            denom: 10000000,
        })),
    },
};

/// Barleycorns `Bc` (`1⁄3 in`).
pub static BARLEYCORN: Derived = Derived {
    id: crate::generated::ids::BARLEYCORN,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "Bc"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 254,
            denom: 30000,
        })),
    },
};

/// Inches `in` (`3Bc`) or (`0.0254m`).
pub static INCH: Derived = Derived {
    id: crate::generated::ids::INCH,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "in"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 254,
            denom: 10000,
        })),
    },
};

/// Hand `hand` (`4in`) or (`0.1016m`).
pub static HAND: Derived = Derived {
    id: crate::generated::ids::HAND,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "hand"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1016,
            denom: 10000,
        })),
    },
};

/// Foot `ft` (`12in`) or (`0.3048m`).
pub static FOOT: Derived = Derived {
    id: crate::generated::ids::FOOT,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "ft"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 3048,
            denom: 10000,
        })),
    },
};

/// Yards `yd` (`3ft`) or (`0.9144m`).
pub static YARD: Derived = Derived {
    id: crate::generated::ids::YARD,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "yd"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 9144,
            denom: 10000,
        })),
    },
};

/// Chain `ch` (`22yd`) or (`20.1168m`).
pub static CHAIN: Derived = Derived {
    id: crate::generated::ids::CHAIN,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "ch"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 201168,
            denom: 10000,
        })),
    },
};

/// Furlong `fur` (`10ch`) or (`201.168m`).
pub static FURLONG: Derived = Derived {
    id: crate::generated::ids::FURLONG,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "fur"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 201168,
            denom: 1000,
        })),
    },
};

/// Mile `mi` (`8fur`) or (`1609.344m`).
pub static MILE: Derived = Derived {
    id: crate::generated::ids::MILE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "mi"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1609344,
            denom: 1000,
        })),
    },
};

/// League `lea` (`3mi`) or (`4828.032m`).
pub static LEAGUE: Derived = Derived {
    id: crate::generated::ids::LEAGUE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "lea"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 4828032,
            denom: 1000,
        })),
    },
};
