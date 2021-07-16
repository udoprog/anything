use crate::powers::Powers;
use crate::unit::{Derived, DerivedVtable, Unit};
use num::BigRational;

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
                multiple_ratio: Some(|| BigRational::new($num.into(), $den.into())),
            },
        };
    };
}

time! {
    /// A minute `m` (`60s`) in [Unit::Second].
    pub static MINUTE = (0x3cea0000, 60u32 / 1u32), |f, _| f.write_str("min")
}

time! {
    /// An hour `H` (`3600s`) in [Unit::Second].
    pub static HOUR = (0x3cea0001, 3600u32 / 1u32), |f, _| f.write_str("hr")
}

time! {
    /// A day `dy` (`86400s`) in [Unit::Second].
    pub static DAY = (0x3cea0003, 86400u32 / 1u32), |f, _| f.write_str("dy")
}

time! {
    /// A week `wk` (`604800s`) in [Unit::Second].
    pub static WEEK = (0x3cea0004, 604800u32 / 1u32), |f, _| f.write_str("wk")
}

time! {
    /// A month in [Unit::Second] defined as `1/12` of [YEAR].
    pub static MONTH = (0x3cea0005, 31557600u32 / 12u32), |f, _| f.write_str("mth")
}

time! {
    /// A year `yr` (`31557600s`) in [Unit::Second].
    pub static YEAR = (0x3cea1000, 31557600u32 / 1u32), |f, _| f.write_str("yr")
}

time! {
    /// A Century (`10yr`) in [Unit::Second] defined as `10` times [YEAR].
    pub static DECADE = (0x3cea2000, 315576000u32 / 1u32), |f, pluralize| if pluralize {
        f.write_str("decades")
    } else {
        f.write_str("decade")
    }
}

time! {
    /// A Century (`100yr`) in [Unit::Second] defined as `100` times [YEAR].
    pub static CENTURY = (0x3cea3000, 3155760000u32 / 1u32), |f, pluralize| if pluralize {
        f.write_str("centuries")
    } else {
        f.write_str("century")
    }
}

time! {
    /// A Millenium (`1000yr`) in [Unit::Second] defined as `1000` times [YEAR].
    pub static MILLENIUM = (0x3cea4000, 31557600000u64 / 1u32), |f, pluralize| if pluralize {
        f.write_str("millenia")
    } else {
        f.write_str("millenium")
    }
}
