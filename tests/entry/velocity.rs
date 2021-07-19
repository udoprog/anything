#[test]
fn test_knots() {
    assert_query!("1852m/hour to kt", 1, kt);
    assert_query!("1NM / 10kt to hours", 1 / 10, hour);
}
