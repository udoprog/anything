use facts::{CompoundUnit, Unit};

macro_rules! query {
    ($expr:expr) => {{
        let db = facts::db::open().unwrap();
        let mut values = facts::query($expr, &db);
        let value = values.next().unwrap().unwrap();
        assert!(values.next().is_none());
        value
    }};
}

#[test]
fn test_queries() {
    let c = CompoundUnit::from_iter([(Unit::LightSpeed, 1, 0)]);

    let n = query!("12c");

    assert_eq!(n.unit(), &c);
    assert_eq!(n.to_u32(), Some(12));
}
