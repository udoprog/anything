use super::FormatRatio;
use num::BigRational;

#[test]
fn test_ratio() {
    let s = FormatRatio::new(&BigRational::new(1u32.into(), 3u32.into()), 8, -6).to_string();
    assert_eq!(s, "0.33333333..");

    let s = FormatRatio::new(&BigRational::new(1u32.into(), 1415u32.into()), 8, -6).to_string();
    assert_eq!(s, "0.00070671378..");

    let s = FormatRatio::new(
        &BigRational::new(1u32.into(), 1415123312312333214u64.into()),
        8,
        -6,
    )
    .to_string();
    assert_eq!(s, "7.0665219..e-19");

    let s = FormatRatio::new(
        &BigRational::new(1u32.into(), 1415123312312333214u64.into()),
        40,
        -6,
    )
    .to_string();
    assert_eq!(s, "7.066521986454909362816626182915140226065..e-19");
}
