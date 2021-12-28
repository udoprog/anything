#[test]
fn test_litre() {
    assert_query!("1l to m^3", 1 / 1000, m ^ 3);
}

#[test]
fn test_cc() {
    assert_query!("1cc to m^3", 1 / 1000000, m ^ 3);
}

#[test]
fn test_gal() {
    assert_query!("1gal to m^3", 3785411784 / 1000000000000, m ^ 3);
}

#[test]
fn test_pint() {
    assert_query!("1pint to gal", 1 / 2, gal);
    assert_query!("1gal to pint", 2, pint);
}

#[test]
fn test_quart() {
    assert_query!("1quart to gal", 1 / 4, gal);
    assert_query!("1gal to quarts", 4, quart);
}

#[test]
fn test_cup() {
    assert_query!("1cup to gal", 1 / 16, gal);
    assert_query!("1gal to cup", 16, cups);
}

#[test]
fn test_gill() {
    assert_query!("1cup to gill", 2, gill);
    assert_query!("1gill to cup", 1 / 2, cups);
}

#[test]
fn test_floz() {
    assert_query!("1gill to floz", 4, floz);
    assert_query!("1cup to floz", 8, floz);
    assert_query!("1floz to cup", 1 / 8, cups);
}

#[test]
fn test_tbsp() {
    assert_query!("1floz to tbsp", 2, tbsp);
    assert_query!("1tbsp to floz", 1 / 2, floz);
}

#[test]
fn test_tsp() {
    assert_query!("1floz to tsp", 6, tsp);
    assert_query!("1tsp to floz", 1 / 6, floz);
}
