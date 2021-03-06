use crate::rational::DisplaySpec;
use crate::Rational;

#[test]
fn test_zeros() {
    assert_eq! {
        str::parse::<Rational>("0").unwrap(),
        Rational::new(0u32, 1u32),
    };

    assert_eq! {
        str::parse::<Rational>("01").unwrap(),
        Rational::new(1u32, 1u32),
    };
}

#[test]
fn test_decimals() {
    assert_eq! {
        str::parse::<Rational>("3.1415").unwrap(),
        Rational::new(31415u32, 10000u32),
    };
}

#[test]
fn test_exponent() {
    assert_eq! {
        str::parse::<Rational>("1.1234e10").unwrap(),
        Rational::new(11234000000u64, 1u32),
    };

    assert_eq! {
        str::parse::<Rational>("1.1234e-10").unwrap(),
        Rational::new(5617u32, 50000000000000u64),
    };

    assert_eq! {
        str::parse::<Rational>("0e10").unwrap(),
        Rational::new(0u32, 1u32),
    };

    assert_eq! {
        str::parse::<Rational>("1e10").unwrap(),
        Rational::new(10000000000u64, 1u32),
    };
}

#[test]
fn test_display() {
    let basic = DisplaySpec {
        limit: 8,
        exponent_limit: 6,
        show_continuation: true,
    };

    let extended = DisplaySpec {
        limit: 40,
        exponent_limit: 6,
        show_continuation: true,
    };

    let s = Rational::new(1u32, 3u32).display(&basic).to_string();
    assert_eq!(s, "0.33333333…");

    let s = Rational::new(1u32, 1415u32).display(&basic).to_string();
    assert_eq!(s, "0.00070671378…");

    let s = Rational::new(1u32, 1415123312312333214u64)
        .display(&basic)
        .to_string();
    assert_eq!(s, "7.0665219…e-19");

    let s = Rational::new(1u32, 1415123312312333214u64)
        .display(&extended)
        .to_string();
    assert_eq!(s, "7.066521986454909362816626182915140226065…e-19");
}

#[test]
fn deserialize() {
    let r = Rational::new(1u32, 100u32);
    let s = serde_json::to_string(&r).unwrap();

    let num: Rational = serde_json::from_str(&s).unwrap();
    assert_eq!(num, Rational::new(1u32, 100u32));
}
