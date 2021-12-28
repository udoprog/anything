//! Special volume units.

use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable, Unit};

/// Litre `l` or `0.001m^3`.
pub static LITRE: Derived = Derived {
    id: crate::generated::ids::LITRE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, _| write!(f, "l"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1,
            denom: 1000,
        })),
    },
};

/// Cubic centimetre `cc` or `1cm^3`.
pub static CUBIC_CENTIMETER: Derived = Derived {
    id: crate::generated::ids::CUBIC_CENTIMETER,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, _| write!(f, "cc"),
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 1,
            denom: 1000000,
        })),
    },
};

/// Gallon `gal` or `0.003785411784m^3`.
pub static GALLON: Derived = Derived {
    id: crate::generated::ids::GALLON,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "gallons")
            } else {
                write!(f, "gallon")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 3785411784,
            denom: 1000000000000,
        })),
    },
};

/// Pint `pint` or `1/2 of a gallon`.
pub static PINT: Derived = Derived {
    id: crate::generated::ids::PINT,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "pints")
            } else {
                write!(f, "pint")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 473176473,
            denom: 250000000000,
        })),
    },
};

/// Quart `quart` or `1/4 of a gallon`.
pub static QUART: Derived = Derived {
    id: crate::generated::ids::QUART,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "quarts")
            } else {
                write!(f, "quart")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 473176473,
            denom: 500000000000,
        })),
    },
};

/// Cup `cup` or `1/16th of a gallon`.
pub static CUP: Derived = Derived {
    id: crate::generated::ids::CUP,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "cups")
            } else {
                write!(f, "cup")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 473176473,
            denom: 2000000000000,
        })),
    },
};

/// Gill `gill` or 1/2th of a cup.
pub static GILL: Derived = Derived {
    id: crate::generated::ids::GILL,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "gills")
            } else {
                write!(f, "gill")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 473176473,
            denom: 4000000000000,
        })),
    },
};

/// Fluid ounce `floz` or 1/4 of a gill.
pub static FLUID_OUNCE: Derived = Derived {
    id: crate::generated::ids::FLUID_OUNCE,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "fl ozs")
            } else {
                write!(f, "fl oz")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 473176473,
            denom: 16000000000000,
        })),
    },
};

/// Table spoons `tbsp` or 1/2 of a floz.
pub static TABLE_SPOON: Derived = Derived {
    id: crate::generated::ids::TABLE_SPOON,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "tbsps")
            } else {
                write!(f, "tbsp")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 473176473,
            denom: 32000000000000,
        })),
    },
};

/// Tea spoons `tsp` or 1/3rd of a tbsp.
pub static TEA_SPOON: Derived = Derived {
    id: crate::generated::ids::TEA_SPOON,
    vtable: &DerivedVtable {
        powers: |powers, p| {
            powers.insert(Unit::Meter, p * 3);
        },
        format: |f, pl| {
            if pl {
                write!(f, "tsps")
            } else {
                write!(f, "tsp")
            }
        },
        conversion: Some(Conversion::Factor(ConversionFraction {
            numer: 157725491,
            denom: 32000000000000,
        })),
    },
};
