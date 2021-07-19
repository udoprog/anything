#[test]
fn test_au() {
    assert_query!("1.495978707e11m to au", 1, au);
}

#[test]
fn test_thous() {
    assert_query!("(1ft / 12000) to th", 1, th);
}

#[test]
fn test_barleycorn() {
    assert_query!("(333th + 1/3) to Bc", 1, Bc);
    assert_query!("(1ft / 36) to Bc", 1, Bc);
}

#[test]
fn test_inch() {
    assert_query!("3Bc to in", 1, in);
    assert_query!("(1ft / 12) to in", 1, in);
}

#[test]
fn test_hand() {
    assert_query!("4in to hand", 1, hand);
    assert_query!("4000thou to hand", 1, hand);
    assert_query!("(1ft / 3) to hand", 1, hand);
}

#[test]
fn test_foot() {
    assert_query!("12in to ft", 1, ft);
    assert_query!("3hand to ft", 1, ft);
    assert_query!("5ft + 12in", 6 / 1, ft);
}

#[test]
fn test_yard() {
    assert_query!("3ft to yd", 1, yd);
    assert_query!("(1ch / 22) to yd", 1, yd);
}

#[test]
fn test_chain() {
    assert_query!("22yd to ch", 1, ch);
    assert_query!("(1fur / 10) to ch", 1, ch);
}

#[test]
fn test_furlong() {
    assert_query!("(1mi / 8) to fur", 1, fur);
    assert_query!("220yd to fur", 1, fur);
    assert_query!("10ch to fur", 1, fur);
}

#[test]
fn test_mile() {
    assert_query!("8fur to mi", 1, mi);
    assert_query!("(1lea / 3) to mi", 1, mi);
    assert_query!("5280ft to mi", 1, mi);
    assert_query!("1760yd to mi", 1, mi);
}

#[test]
fn test_league() {
    assert_query!("3mi to lea", 1, lea);
}

#[test]
fn test_fathom() {
    assert_query!("1.852m to ftm", 1, ftm);
    assert_query!("(1cable / 100) to ftm", 1, ftm);
}

#[test]
fn test_cable() {
    assert_query!("185.2m to cable", 1, cable);
    assert_query!("100ftm to cable", 1, cable);
    assert_query!("(1NM / 10) to cable", 1, cable);
}

#[test]
fn test_nautical_miles() {
    assert_query!("1852m to NM", 1, NM);
}

#[test]
fn test_link() {
    assert_query!("(66ft / 100) to link", 1, link);
}

#[test]
fn test_rod() {
    assert_query!("(66ft / 4) to rod", 1, rod);
}

#[test]
fn test_imperial_addition() {
    assert_query!("1m + 1cm", 101 / 100, m);
    assert_query!("5yd + 3ft", 6 / 1, yd);
    assert_query!("5mi + 1760yd", 6 / 1, mi);
}
