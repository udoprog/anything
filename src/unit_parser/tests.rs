use super::UnitParser;
use crate::prefix::Prefix;
use crate::unit::Unit;

macro_rules! parse {
    ($expr:expr) => {{
        let mut parser = UnitParser::new($expr);

        let mut out = Vec::new();

        while let Some(parsed) = parser.next().unwrap() {
            out.push((parsed.prefix, parsed.name));
        }

        out
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
        (Unit::Byte, &["B", "byte"]),
        (Unit::Acceleration, &["a", "acc", "acceleration"]),
        (Unit::Gforce, &["gforce", "g-force"]),
        (Unit::Ton, &["ton", "tons"]),
        (Unit::Joule, &["J", "joule"]),
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
    ];

    for (unit, variants) in UNITS.iter().copied() {
        for v in variants.iter().copied() {
            assert_eq! {
                parse!(v),
                &[(Prefix::NONE, unit)],
                "`{}`",
                v
            };
        }

        for (prefix, strings) in PREFIXES.iter().copied() {
            for string in strings.iter().copied() {
                for v in variants.iter().copied() {
                    let s = format!("{}{}", string, v);

                    assert_eq! {
                        parse!(&s),
                        &[(prefix, unit)],
                        "`{}`",
                        s
                    };
                }
            }
        }
    }
}
