//! Special distance units.

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use rational::Rational;

/// Astronomical unit (`au`) or `149597870700m`.
pub static AU: Derived = Derived {
    id: crate::generated::ids::AU,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "au"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(149597870700u64, 1u32);
            },
            from: |num| {
                *num /= Rational::new(149597870700u64, 1u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1852, 1000u32);
            },
            from: |num| {
                *num /= Rational::new(1852, 1000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1852, 10u32);
            },
            from: |num| {
                *num /= Rational::new(1852, 10u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1852, 1u32);
            },
            from: |num| {
                *num /= Rational::new(1852, 1u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(201168u32, 1000000u32);
            },
            from: |num| {
                *num /= Rational::new(201168u32, 1000000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(50292u32, 10000u32);
            },
            from: |num| {
                *num /= Rational::new(50292u32, 10000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(254u32, 10000000u32);
            },
            from: |num| {
                *num /= Rational::new(254u32, 10000000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(254u32, 30000u32);
            },
            from: |num| {
                *num /= Rational::new(254u32, 30000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(254u32, 10000u32);
            },
            from: |num| {
                *num /= Rational::new(254u32, 10000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1016, 10000u32);
            },
            from: |num| {
                *num /= Rational::new(1016, 10000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(3048u32, 10000u32);
            },
            from: |num| {
                *num /= Rational::new(3048u32, 10000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(9144u32, 10000u32);
            },
            from: |num| {
                *num /= Rational::new(9144u32, 10000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(201168u32, 10000u32);
            },
            from: |num| {
                *num /= Rational::new(201168u32, 10000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(201168u32, 1000u32);
            },
            from: |num| {
                *num /= Rational::new(201168u32, 1000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(1609344u32, 1000u32);
            },
            from: |num| {
                *num /= Rational::new(1609344u32, 1000u32);
            },
        }),
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
        conversion: Some(Conversion {
            to: |num| {
                *num *= Rational::new(4828032u32, 1000u32);
            },
            from: |num| {
                *num /= Rational::new(4828032u32, 1000u32);
            },
        }),
    },
};
