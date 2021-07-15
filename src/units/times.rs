use crate::powers::Powers;
use crate::unit::{Derived, DerivedVtable, Unit};
use bigdecimal::BigDecimal;

fn time_powers(powers: &mut Powers, power: i32) {
    powers.insert(Unit::Second, power);
}

macro_rules! time {
    ($(#[$meta:meta])* pub static $name:ident = ($id:expr, $multiple:expr, $scale:expr), $f:expr) => {
        $(#[$meta])*
        pub static $name: Derived = Derived {
            id: $id,
            vtable: &DerivedVtable {
                powers: time_powers,
                format: $f,
                multiple: Some(|| BigDecimal::new($multiple.into(), $scale)),
            },
        };
    };
}

time! {
    /// A minute `m` (`60s`) in [Unit::Second].
    pub static MINUTE = (0x3cea90d3, 60, 0), |f, _| f.write_str("mn")
}
time! {
    /// An hour `H` (`3600s`) in [Unit::Second].
    pub static HOUR = (0x8884f852, 3600, 0), |f, _| f.write_str("H")
}
time! {
    /// A day `dy` (`86400s`) in [Unit::Second].
    pub static DAY = (0xdacd8d53, 86400, 0), |f, _| f.write_str("dy")
}
time! {
    /// A week `wk` (`604800s`) in [Unit::Second].
    pub static WEEK = (0xd6d4f93f, 604800, 0), |f, _| f.write_str("wk")
}
time! {
    /// A month in [Unit::Second] defined as `1/12` of [YEAR].
    pub static MONTH = (0x458a3642, 262259423u32, 2), |f, _| f.write_str("mth")
}
time! {
    /// A year `yr` (`31471130.76s`) in [Unit::Second].
    pub static YEAR = (0xe923ce05, 3147113076u32, 2), |f, _| f.write_str("yr")
}
time! {
    /// A Century (`10yr`) in [Unit::Second] defined as `10` times [YEAR].
    pub static DECADE = (0xbed4a84b, 3147113076u32, 1), |f, pluralize| if pluralize {
        f.write_str("decades")
    } else {
        f.write_str("decade")
    }
}
time! {
    /// A Century (`100yr`) in [Unit::Second] defined as `100` times [YEAR].
    pub static CENTURY = (0x8efe5bbc, 3147113076u32, 0), |f, pluralize| if pluralize {
        f.write_str("centuries")
    } else {
        f.write_str("century")
    }
}
time! {
    /// A Millenium (`1000yr`) in [Unit::Second] defined as `1000` times [YEAR].
    pub static MILLENIUM = (0x0d2818da, 3147113076u32, -1), |f, pluralize| if pluralize {
        f.write_str("millenia")
    } else {
        f.write_str("millenium")
    }
}
