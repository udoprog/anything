#[test]
fn test_btu() {
    assert_query!("1btu to J", 1055, J);
}

#[test]
fn test_btu_conversions() {
    assert_query!("1Gbtu to J", 1055000000000, J);
    assert_query!("1btu^2 to J^2", 1113025, J ^ 2);
    assert_query!("1Gbtu^2 to J^2", 1113025000000000000000000, J ^ 2);
    assert_query!("1Gbtu^2 / 1113025kJ^2", 1000000000000);
    assert_query!("1btu^2 * 1113025J^2 as J^4", 1238824650625, J ^ 4);
    assert_query!("1Gbtu^2 * 1113025kJ^2", 1113025000000000000000000000000, btu^2J^2);
    assert_query!(
        "1Gbtu^2 * 1113025kJ^2 to J^4",
        1238824650625000000000000000000000000,
        J ^ 4
    );
}

#[test]
fn test_electronvolt() {
    assert_query!("1eV to J", 801088317 / 5000000000000000000000000000, J);
}
