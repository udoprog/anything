#[test]
fn test_litre() {
    assert_query!("1l to m^3", 1 / 1000, m ^ 3);
}

#[test]
fn test_cc() {
    assert_query!("1cc to m^3", 1 / 1000000, m ^ 3);
}
