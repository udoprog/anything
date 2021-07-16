use facts::units;
use facts::{Compound, Unit};
use num::{BigRational, ToPrimitive};

macro_rules! query {
    ($expr:expr) => {{
        let db = facts::Db::open().unwrap();
        let mut values = facts::query($expr, &db);
        let value = values.next().unwrap().unwrap();
        assert!(values.next().is_none());
        value
    }};
}

macro_rules! unit {
    ($expr:expr) => {
        str::parse::<Compound>($expr).unwrap()
    };
}

macro_rules! ratio {
    ($a:literal / $b:literal) => {
        BigRational::new($a.into(), $b.into())
    };
}

#[test]
fn test_queries() {
    let c = Compound::from_iter([(Unit::Derived(units::LIGHTSPEED), 1, 0)]);

    let n = query!("12c");

    assert_eq!(n.unit(), &c);
    assert_eq!(n.to_u32(), Some(12));
}

#[test]
fn test_compound_division() {
    let c = Compound::from_iter([(Unit::Derived(units::VOLT), -7, 0)]);

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
    let value = query!("1Gbtu to J");
    assert_eq!(value.to_u128(), Some(1055000000000));

    let value = query!("1btu^2 to J^2");
    assert_eq!(value.to_u128(), Some(1113025));

    let value = query!("1Gbtu^2 to J^2");
    assert_eq!(value.to_u128(), Some(1113025000000000000000000));

    let value = query!("1Gbtu^2 / 1113025kJ^2");
    assert_eq!(value.to_u128(), Some(1000000000000));

    let value = query!("1btu^2 * 1113025J^2 as J^4");
    assert_eq!(value.to_u128(), Some(1238824650625));
    assert_eq!(value.unit(), &unit!("J^4"));

    let value = query!("1Gbtu^2 * 1113025kJ^2");
    assert_eq!(value.to_u128(), Some(1113025000000000000000000000000));
    assert_eq!(value.unit(), &unit!("btu^2J^2"));

    let value = query!("1Gbtu^2 * 1113025kJ^2 to J^4");
    assert_eq!(value.to_u128(), Some(1238824650625000000000000000000000000));
}

#[test]
fn test_multiple_identity_sheds() {
    let expected = query!("0.05c / 500 years * mass of earth as N");

    assert_eq!(
        expected.value().clone(),
        ratio!(223795069897000000000000000i128 / 39447)
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
fn test_addition() {
    let value = query!("1m + 1cm");
    assert_eq!(value.value(), &ratio!(101 / 100));
}

#[test]
fn test_imperial() {
    let value = query!("12in to ft");
    assert_eq!(value.split(), (ratio!(1 / 1), unit!("ft")));

    let value = query!("5ft + 12in");
    assert_eq!(value.split(), (ratio!(6 / 1), unit!("ft")));

    let value = query!("5yd + 3ft");
    assert_eq!(value.split(), (ratio!(6 / 1), unit!("yd")));

    let value = query!("5mi + 1760yd");
    assert_eq!(value.split(), (ratio!(6 / 1), unit!("mi")));
}

#[test]
fn test_times() {
    let value = query!("1s + 59s to min");
    assert_eq!(value.split(), (ratio!(1 / 1), unit!("min")));

    let value = query!("5min + 55min to hour");
    assert_eq!(value.split(), (ratio!(1 / 1), unit!("hr")));

    let value = query!("5hours + 19hours to days");
    assert_eq!(value.split(), (ratio!(1 / 1), unit!("days")));

    let value = query!("5days + 25days to months");
    assert_eq!(value.split(), (ratio!(480 / 487), unit!("months")));

    let value = query!("1month + 11months to years");
    assert_eq!(value.split(), (ratio!(1 / 1), unit!("years")));

    let value = query!("4year + 6years to decades");
    assert_eq!(value.split(), (ratio!(1 / 1), unit!("decades")));

    let value = query!("4decades + 6decades to centuries");
    assert_eq!(value.split(), (ratio!(1 / 1), unit!("centuries")));
}
