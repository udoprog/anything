use super::parse_decimal_big_rational;
use num::BigRational;

#[test]
fn test_zeros() {
    assert_eq! {
        parse_decimal_big_rational("0").unwrap(),
        BigRational::new(0u32.into(), 1u32.into()),
    };

    assert_eq! {
        parse_decimal_big_rational("01").unwrap(),
        BigRational::new(1u32.into(), 1u32.into()),
    };
}

#[test]
fn test_decimals() {
    assert_eq! {
        parse_decimal_big_rational("3.1415").unwrap(),
        BigRational::new(31415u32.into(), 10000u32.into()),
    };
}

#[test]
fn test_exponent() {
    assert_eq! {
        parse_decimal_big_rational("1.1234e10").unwrap(),
        BigRational::new(11234000000u64.into(), 1u32.into()),
    };

    assert_eq! {
        parse_decimal_big_rational("1.1234e-10").unwrap(),
        BigRational::new(5617u32.into(), 50000000000000u64.into()),
    };

    assert_eq! {
        parse_decimal_big_rational("0e10").unwrap(),
        BigRational::new(0u32.into(), 1u32.into()),
    };

    assert_eq! {
        parse_decimal_big_rational("1e10").unwrap(),
        BigRational::new(10000000000u64.into(), 1u32.into()),
    };
}
