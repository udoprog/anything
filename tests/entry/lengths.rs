#[test]
fn test_nautical_miles() {
    assert_query!("1852m to NM", 1, NM);
}

#[test]
fn test_au() {
    assert_query!("1.495978707e11m to au", 1, au);
}
