use facts::units;
use facts::{Compound, Unit};

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
