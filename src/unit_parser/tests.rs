use super::UnitParser;
use crate::prefix::Prefix;
use crate::unit::Unit;

macro_rules! parse {
    ($expr:expr) => {{
        (|| {
            let mut parser = UnitParser::new($expr);

            let mut out = Vec::new();

            while let Some(parsed) = parser.next()? {
                out.push((parsed.prefix, parsed.name));
            }

            Ok::<_, &str>(out)
        })()
    }};
}

#[test]
fn test_prefixes() {
    const PREFIXES: &[(i32, &[&str])] = &[
        (Prefix::YOCTO, &["y", "yocto"]),
        (Prefix::ZEPTO, &["z", "zepto"]),
        (Prefix::ATTO, &["a", "atto"]),
        (Prefix::FEMTO, &["f", "femto"]),
        (Prefix::PICO, &["p", "pico"]),
        (Prefix::NANO, &["n", "nano"]),
        (Prefix::MICRO, &["μ", "micro"]),
        (Prefix::MILLI, &["m", "milli"]),
        (Prefix::CENTI, &["c", "centi"]),
        (Prefix::DECI, &["d", "deci"]),
        (Prefix::DECA, &["da", "deca"]),
        (Prefix::HECTO, &["h", "hecto"]),
        (Prefix::KILO, &["k", "kilo"]),
        (Prefix::MEGA, &["M", "mega"]),
        (Prefix::GIGA, &["G", "giga"]),
        (Prefix::TERA, &["T", "tera"]),
        (Prefix::PETA, &["P", "peta"]),
        (Prefix::EXA, &["E", "exa"]),
        (Prefix::ZETTA, &["Z", "zetta"]),
        (Prefix::YOTTA, &["Y", "yotta"]),
    ];

    const UNITS: &[(Unit, &[&str])] = &[
        (Unit::Second, &["s"]),
        (Unit::KiloGram, &["kg"]),
        (Unit::Meter, &["m", "meter", "meters"]),
        (Unit::Ampere, &["A", "ampere", "amperes"]),
        (Unit::Kelvin, &["K", "kelvin", "kelvins"]),
        (Unit::Mole, &["mol", "mols", "mole", "moles"]),
        (Unit::Candela, &["cd", "candela", "candelas"]),
        (Unit::Byte, &["B", "byte"]),
        (Unit::Acceleration, &["a", "acc", "acceleration"]),
        (Unit::Gforce, &["gforce", "g-force"]),
        (Unit::Ton, &["ton", "tons"]),
        (Unit::Year, &["y", "year", "years"]),
        (Unit::Decade, &["decade", "decades"]),
        (Unit::Century, &["century", "centuries"]),
        (Unit::Millenium, &["M", "millenium"]),
        (Unit::Month, &["mth", "mths", "month", "months"]),
        (Unit::Week, &["w", "week", "weeks"]),
        (Unit::Day, &["day", "days"]),
        (Unit::Hour, &["hour", "hours"]),
        (Unit::Minute, &["min", "mins", "minute", "minutes"]),
        (Unit::Btu, &["btu"]),
        (Unit::Au, &["au"]),
        (Unit::LightSpeed, &["c"]),
        (Unit::Newton, &["N", "newton", "newtons"]),
        (Unit::Pascal, &["Pa", "pascal", "pascals"]),
        (Unit::Joule, &["J", "joule"]),
        (Unit::Watt, &["W", "watt", "watts"]),
    ];

    for (unit, variants) in UNITS.iter().copied() {
        for v in variants.iter().copied() {
            assert_eq! {
                parse!(v).as_deref(),
                Ok(&[(Prefix::NONE, unit)][..]),
                "`{}`",
                v
            };
        }

        for (prefix, strings) in PREFIXES.iter().copied() {
            for string in strings.iter().copied() {
                for v in variants.iter().copied() {
                    // NB: dash to separate is always guaranteed to work, while
                    // others might be ambiguous.
                    let s = format!("{}-{}", string, v);

                    assert_eq! {
                        parse!(&s).as_deref(),
                        Ok(&[(prefix, unit)][..]),
                        "`{}`; prefix=`{prefix}`, variant=`{variant}`",
                        s,
                        prefix = string,
                        variant = v,
                    };

                    // NB: parsing ambiguity with `cd` (candela).
                    // Use requires long prefix instead, like `centidays`.
                    match (string, v) {
                        ("c", "decade" | "decades" | "day" | "days") => continue,
                        ("deca", "decade" | "decades") => continue,
                        ("d", "ampere" | "amperes" | "au" | "a" | "acc" | "acceleration") => {
                            continue
                        }
                        ("P", "ampere" | "amperes" | "au" | "a" | "acc" | "acceleration") => {
                            continue
                        }
                        ("da", "y" | "year" | "years") => continue,
                        _ => {}
                    }

                    let s = format!("{}{}", string, v);

                    assert_eq! {
                        parse!(&s).as_deref(),
                        Ok(&[(prefix, unit)][..]),
                        "`{}`; prefix=`{prefix}`, variant=`{variant}`",
                        s,
                        prefix = string,
                        variant = v,
                    };
                }
            }
        }
    }
}
