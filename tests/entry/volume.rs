#[test]
fn test_litre() {
    assert_query!("1l to m^3", 1 / 1000, m ^ 3);
}
