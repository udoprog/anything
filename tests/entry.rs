use facts::units;
use facts::{Compound, Unit};
use num::ToPrimitive;
use std::iter::FromIterator;

#[macro_export]
macro_rules! query {
    ($expr:expr) => {{
        let db = facts::Db::in_memory().unwrap();
        let mut values = facts::query($expr, &db);
        let value = values.next().unwrap().unwrap();
        assert!(values.next().is_none());
        value
    }};
}

#[macro_export]
macro_rules! unit {
    ($expr:expr) => {
        str::parse::<facts::Compound>($expr).unwrap()
    };
}

#[macro_export]
macro_rules! ratio {
    ($a:literal) => {{
        let _a: u128 = $a;
        num::BigRational::new(_a.into(), 1.into())
    }};

    ($a:literal / $b:literal) => {{
        let _a: u128 = $a;
        let _b: u128 = $b;
        num::BigRational::new(_a.into(), _b.into())
    }};
}

#[macro_export]
macro_rules! lit {
    ($a:literal $(/ $b:literal)? $(, $($tt:tt)*)?) => {
        facts::Numeric::new(
            ratio!($a $(/ $b)*),
            str::parse::<facts::Compound>(concat!($($(stringify!($tt)),*)*)).unwrap(),
        )
    }
}

/// Assert that the result of the given query matches the given literal value.
#[macro_export]
macro_rules! assert_query {
    ($query:expr, $($lit:tt)*) => {
        assert_eq!(query!($query), lit!($($lit)*));
    };
}

#[path = "entry/imperial_lengths.rs"]
mod imperial_lengths;
#[path = "entry/imperial_weights.rs"]
mod imperial_weights;
#[path = "entry/lengths.rs"]
mod lengths;
#[path = "entry/temperatures.rs"]
mod temperatures;
#[path = "entry/velocities.rs"]
mod velocities;

#[test]
fn test_queries() {
    let c = Compound::from_iter([(Unit::Derived(units::velocities::LIGHT_SPEED), (1, 0))]);

    let n = query!("12c");

    assert_eq!(n.unit(), &c);
    assert_eq!(n.to_u32(), Some(12));
}

#[test]
fn test_compound_division() {
    let c = Compound::from_iter([(Unit::Derived(units::VOLT), (-7, 0))]);

    let n = query!("1V^3 / 1V^10");

    assert_eq!(n.unit(), &c);
    assert_eq!(n.to_u32(), Some(1));
}

#[test]
fn test_compound_mul() {
    let n = query!("1Wb*V * 1V");

    assert_eq!(n.unit(), &unit!("WbV^2"));
    assert_eq!(n.to_u32(), Some(1));
}

#[test]
fn test_velocities() {
    let value = query!("10m / 10km/s");
    assert_eq!(value.value(), &ratio!(10 / 10000));

    let value = query!("10km / 10km/s");
    assert_eq!(value.value(), &ratio!(1 / 1));

    let value = query!("10km / 1c");
    assert_eq!(value.value(), &ratio!(5000 / 149896229));
}

#[test]
fn test_multiple_division() {
    assert_query!("1Gbtu to J", 1055000000000, J);
    assert_query!("1btu^2 to J^2", 1113025, J ^ 2);
    assert_query!("1Gbtu^2 to J^2", 1113025000000000000000000, J ^ 2);
    assert_query!("1Gbtu^2 / 1113025kJ^2", 1000000000000);
    assert_query!("1btu^2 * 1113025J^2 as J^4", 1238824650625, J ^ 4);
    assert_query!("1Gbtu^2 * 1113025kJ^2", 1113025000000000000000000000000, btu^2J^2);
    assert_query!(
        "1Gbtu^2 * 1113025kJ^2 to J^4",
        1238824650625000000000000000000000000,
        J ^ 4
    );
}

#[test]
fn test_multiple_identity_sheds() {
    let expected = query!("0.05c / 500 years * mass of earth as N");

    assert_eq!(
        expected.value().clone(),
        ratio!(223795069897000000000000000 / 39447)
    );

    let mut alternatives = Vec::new();
    alternatives.push("(0.05c as m/s) / 500years * mass of earth as N");
    alternatives.push("(0.05c as m/s) / (500years as seconds) * mass of earth as N");
    alternatives.push("0.05c / (500years as seconds) * mass of earth as N");
    alternatives.push("(0.05c / 500years as m/s^2) * mass of earth as N");

    for alt in alternatives {
        let actual = query!(alt);
        assert_eq!(actual, expected, "{} != {}", actual, expected);
    }
}

#[test]
fn test_times() {
    assert_query!("1s + 59s to min", 1, min);
    assert_query!("5min + 55min to hour", 1, hr);
    assert_query!("5hours + 19hours to days", 1, days);
    assert_query!("5days + 25days to months", 480 / 487, months);
    assert_query!("1month + 11months to years", 1, years);
    assert_query!("4year + 6years to decades", 1, decades);
    assert_query!("4decades + 6decades to centuries", 1, centuries);
}
