#[test]
fn test_hectare() {
    assert_query!("1ha to m^2", 10000, m ^ 2);
}

#[test]
fn test_perch() {
    assert_query!("(30yd^2 + 1/4) to perch", 1, perch);
    assert_query!("(1acre / 160) to perch", 1, perch);
    assert_query!("(1rd * 1rd) to perch", 1, perch);
}

#[test]
fn test_rood() {
    assert_query!("1210yd^2 to rood", 1, rood);
    assert_query!("40perches to rood", 1, rood);
    assert_query!("(1acre / 4) to rood", 1, rood);
    assert_query!("(1fur * 1rd) to rood", 1, rood);
}

#[test]
fn test_acre() {
    assert_query!("4840yd^2 to acre", 1, acre);
    assert_query!("4roods to acre", 1, acre);
    assert_query!("(1fur * 1chain) to acre", 1, acre);
}
