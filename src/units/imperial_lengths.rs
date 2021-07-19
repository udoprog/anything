//! Imperial lengths.
//!
//! See [Imperial units on Wikipedia](https://en.wikipedia.org/wiki/Imperial_units).

use crate::unit::{Conversion, Derived, DerivedVtable, Unit};
use num::BigRational;

/// Thou `thou` (`1⁄12000ft`).
pub static THOU: Derived = Derived {
    id: 0xd3c90010,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "th"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(254u32.into(), 10000000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(254u32.into(), 10000000u32.into());
            },
        }),
    },
};

/// Barleycorns `Bc` (`1⁄3 in`).
pub static BARLEYCORN: Derived = Derived {
    id: 0xd3c90020,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "Bc"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(254u32.into(), 30000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(254u32.into(), 30000u32.into());
            },
        }),
    },
};

/// Inches `in` (`3Bc`) or (`0.0254m`).
pub static INCH: Derived = Derived {
    id: 0xd3c90000,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "in"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(254u32.into(), 10000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(254u32.into(), 10000u32.into());
            },
        }),
    },
};

/// Hand `hand` (`4in`) or (`0.1016m`).
pub static HAND: Derived = Derived {
    id: 0xd3c90030,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "hand"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1016.into(), 10000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(1016.into(), 10000u32.into());
            },
        }),
    },
};

/// Foot `ft` (`12in`) or (`0.3048m`).
pub static FOOT: Derived = Derived {
    id: 0xd3c90001,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "ft"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(3048u32.into(), 10000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(3048u32.into(), 10000u32.into());
            },
        }),
    },
};

/// Yards `yd` (`3ft`) or (`0.9144m`).
pub static YARD: Derived = Derived {
    id: 0xd3c90002,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "yd"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(9144u32.into(), 10000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(9144u32.into(), 10000u32.into());
            },
        }),
    },
};

/// Chain `ch` (`22yd`) or (`20.1168m`).
pub static CHAIN: Derived = Derived {
    id: 0xd3c90040,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "ch"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(201168u32.into(), 10000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(201168u32.into(), 10000u32.into());
            },
        }),
    },
};

/// Furlong `fur` (`10ch`) or (`201.168m`).
pub static FURLONG: Derived = Derived {
    id: 0xd3c90040,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "fur"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(201168u32.into(), 1000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(201168u32.into(), 1000u32.into());
            },
        }),
    },
};

/// Mile `mi` (`8fur`) or (`1609.344m`).
pub static MILE: Derived = Derived {
    id: 0xd3c90003,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "mi"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(1609344u32.into(), 1000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(1609344u32.into(), 1000u32.into());
            },
        }),
    },
};

/// League `lea` (`3mi`) or (`4828.032m`).
pub static LEAGUE: Derived = Derived {
    id: 0xd3c90004,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p);
        },
        format: |f, _| write!(f, "lea"),
        conversion: Some(Conversion {
            to: |num| {
                *num *= BigRational::new(4828032u32.into(), 1000u32.into());
            },
            from: |num| {
                *num /= BigRational::new(4828032u32.into(), 1000u32.into());
            },
        }),
    },
};
