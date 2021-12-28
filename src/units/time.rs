//! Special time units.

use crate::powers::Powers;
use crate::unit::{Conversion, ConversionFraction, Derived, DerivedVtable, Unit};

fn time_powers(powers: &mut Powers, power: i32) {
    powers.insert(Unit::Second, power);
}

macro_rules! time {
    ($(#[$meta:meta])* pub static $name:ident = ($id:expr, $num:literal / $den:literal), $f:expr) => {
        $(#[$meta])*
        pub static $name: Derived = Derived {
            id: $id,
            vtable: &DerivedVtable {
                powers: time_powers,
                format: $f,
                conversion: Some(Conversion::Factor(ConversionFraction {
                    numer: $num,
                    denom: $den,
                })),
            },
        };
    };
}

time! {
    /// A minute `m` (`60s`) in [Unit::Second].
    pub static MINUTE = (crate::generated::ids::MINUTE, 60 / 1), |f, _| f.write_str("min")
}

time! {
    /// An hour `H` (`3600s`) in [Unit::Second].
    pub static HOUR = (crate::generated::ids::HOUR, 3600 / 1), |f, _| f.write_str("hr")
}

time! {
    /// A day `dy` (`86400s`) in [Unit::Second].
    pub static DAY = (crate::generated::ids::DAY, 86400 / 1), |f, _| f.write_str("dy")
}

time! {
    /// A week `wk` (`604800s`) in [Unit::Second].
    pub static WEEK = (crate::generated::ids::WEEK, 604800 / 1), |f, _| f.write_str("wk")
}

time! {
    /// A month in [Unit::Second] defined as `1/12` of [YEAR].
    pub static MONTH = (crate::generated::ids::MONTH, 31557600 / 12), |f, _| f.write_str("mth")
}

time! {
    /// A year `yr` (`31557600s`) in [Unit::Second].
    pub static YEAR = (crate::generated::ids::YEAR, 31557600 / 1), |f, _| f.write_str("yr")
}

time! {
    /// A Century (`10yr`) in [Unit::Second] defined as `10` times [YEAR].
    pub static DECADE = (crate::generated::ids::DECADE, 315576000 / 1), |f, pluralize| if pluralize {
        f.write_str("decades")
    } else {
        f.write_str("decade")
    }
}

time! {
    /// A Century (`100yr`) in [Unit::Second] defined as `100` times [YEAR].
    pub static CENTURY = (crate::generated::ids::CENTURY, 3155760000 / 1), |f, pluralize| if pluralize {
        f.write_str("centuries")
    } else {
        f.write_str("century")
    }
}

time! {
    /// A Millenium (`1000yr`) in [Unit::Second] defined as `1000` times [YEAR].
    pub static MILLENIUM = (crate::generated::ids::MILLENIUM, 31557600000 / 1), |f, pluralize| if pluralize {
        f.write_str("millenia")
    } else {
        f.write_str("millenium")
    }
}
