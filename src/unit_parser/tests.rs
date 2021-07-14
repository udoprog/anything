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
    const PREFIXES: [(&[&str], i32); 19] = [
        (&["y", "yocto"], Prefix::YOCTO),
        (&["z", "zepto"], Prefix::ZEPTO),
        (&["a", "atto"], Prefix::ATTO),
        (&["f", "femto"], Prefix::FEMTO),
        (&["p", "pico"], Prefix::PICO),
        (&["n", "nano"], Prefix::NANO),
        (&["μ", "micro"], Prefix::MICRO),
        (&["m", "milli"], Prefix::MILLI),
        (&["c", "centi"], Prefix::CENTI),
        (&["d", "deci"], Prefix::DECI),
        (&[""], Prefix::NONE),
        (&["k", "kilo"], Prefix::KILO),
        (&["M", "mega"], Prefix::MEGA),
        (&["G", "giga"], Prefix::GIGA),
        (&["T", "tera"], Prefix::TERA),
        (&["P", "peta"], Prefix::PETA),
        (&["E", "exa"], Prefix::EXA),
        (&["Z", "zetta"], Prefix::ZETTA),
        (&["Y", "yotta"], Prefix::YOTTA),
    ];

    const UNITS: [(Unit, &[&str]); 20] = [
        (Unit::Second, &["s"]),
        (Unit::KiloGram, &["kg"]),
        (Unit::Meter, &["m", "meter", "meters"]),
        (Unit::Byte, &["B", "byte"]),
        (Unit::Acceleration, &["a"]),
        (Unit::Gforce, &["gforce"]),
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

    for (strings, prefix) in PREFIXES {
        for string in strings {
            for (unit, variants) in UNITS {
                if unit != Unit::KiloGram {
                    continue;
                }

                for v in variants.iter().copied() {
                    let s = format!("{}{}", string, v);

                    let result = parse!(&s);
                    assert_eq! {
                        result,
                        &[(prefix, unit)],
                        "`{}`",
                        s
                    };
                }
            }
        }
    }
}
