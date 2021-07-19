#[test]
fn test_tonne() {
    assert_query!("1ton to kg", 1000, kg);
}

#[test]
fn test_dalton() {
    assert_query!("1Da to kg", 332107813321 / 200000000000, kg);
}

#[test]
fn test_grains() {
    assert_query!("1g to gr", 100000000 / 6479891, gr);
    assert_query!("1gr to g", 6479891 / 100000000, g);
    assert_query!("(1lb / 7000) to gr", 1, gr);
}

#[test]
fn test_drachm() {
    assert_query!("(1gr * 27.34375) to dr", 1, dr);
    assert_query!("(1lb / 256) to dr", 1, dr);
}

#[test]
fn test_ounce() {
    assert_query!("16dr to oz", 1, oz);
    assert_query!("(1lb / 16) to oz", 1, oz);
}

#[test]
fn test_pound() {
    assert_query!("16oz to lb", 1, lb);
    assert_query!("(1st / 14) to lb", 1, lb);
}

#[test]
fn test_stone() {
    assert_query!("14lb to st", 1, st);
    assert_query!("(1qr / 2) to st", 1, st);
}

#[test]
fn test_quarter() {
    assert_query!("2st to qr", 1, qr);
    assert_query!("(1hundredweight / 4) to qr", 1, qr);
}

#[test]
fn test_hundredweight() {
    assert_query!("4qr to hundredweight", 1, hundredweight);
    assert_query!("(1t / 20) to hundredweight", 1, hundredweight);
}

#[test]
fn test_ton() {
    assert_query!("20hundredweight to t", 1, t);
}

#[test]
fn test_slug() {
    assert_query!("1slug to kg", 729695147 / 50000000, kg);
}
