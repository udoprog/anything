use facts::units;
use facts::{Compound, Unit};
use num::BigRational;

macro_rules! query {
    ($expr:expr) => {{
        let db = facts::Db::open().unwrap();
        let mut values = facts::query($expr, &db);
        let value = values.next().unwrap().unwrap();
        assert!(values.next().is_none());
        value
    }};
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
    let c = Compound::from_iter([
        (Unit::Derived(units::WEBER), 1, 0),
        (Unit::Derived(units::VOLT), 2, 0),
    ]);

    let n = query!("1Wb*V * 1V");

    assert_eq!(n.unit(), &c);
    assert_eq!(n.to_u32(), Some(1));
}

#[test]
fn test_multiple_division() {
    let c = Compound::from_iter([]);
    let n = query!("1Gbtu^2 / 1113025kJ^2");

    assert_eq!(n.unit(), &c);
    assert_eq!(n.to_u32(), Some(1000000));
}

#[test]
fn test_multiple_identity_sheds() {
    let expected = query!("0.05c / 500years * mass of earth as N");

    assert_eq!(
        expected.value().clone(),
        BigRational::new(223795069897000000000000000i128.into(), 39447u32.into())
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
