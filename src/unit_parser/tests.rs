use super::UnitParser;
use crate::prefix::Prefix;
use crate::unit::Unit;
use crate::units::{self, distances, times, velocities};

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

    let units = &[
        (Prefix::NONE, Unit::Second, &["s"][..]),
        (-3, Unit::KiloGram, &["g"][..]),
        (Prefix::NONE, Unit::Meter, &["m", "meter", "meters"][..]),
        (Prefix::NONE, Unit::Ampere, &["A", "ampere", "amperes"][..]),
        (Prefix::NONE, Unit::Kelvin, &["K", "kelvin", "kelvins"][..]),
        (
            Prefix::NONE,
            Unit::Mole,
            &["mol", "mols", "mole", "moles"][..],
        ),
        (
            Prefix::NONE,
            Unit::Candela,
            &["cd", "candela", "candelas"][..],
        ),
        (Prefix::NONE, Unit::Byte, &["B", "byte"][..]),
        (
            Prefix::NONE,
            Unit::Derived(units::ACCELERATION),
            &["a", "acc", "acceleration"],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::VELOCITY),
            &["v", "vel", "velocity"],
        ),
        (Prefix::NONE, Unit::Derived(units::BTU), &["btu"][..]),
        (Prefix::NONE, Unit::Derived(distances::AU), &["au"][..]),
        (
            Prefix::NONE,
            Unit::Derived(velocities::LIGHT_SPEED),
            &["c"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::JOULE),
            &["J", "joule"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::GFORCE),
            &["gforce", "g-force"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::TON),
            &["ton", "tons"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::YEAR),
            &["yr", "year", "years"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::DECADE),
            &["decade", "decades"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::CENTURY),
            &["century", "centuries"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::MILLENIUM),
            &["M", "millenium"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::MONTH),
            &["mth", "mths", "month", "months"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::WEEK),
            &["wk", "week", "weeks"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::DAY),
            &["dy", "day", "days"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::HOUR),
            &["hr", "hour", "hours"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(times::MINUTE),
            &["min", "mins", "minute", "minutes"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::NEWTON),
            &["N", "newton", "newtons"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::PASCAL),
            &["Pa", "pascal", "pascals"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::WATT),
            &["W", "watt", "watts"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::COULOMB),
            &["C", "coulomb", "coulombs"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::VOLT),
            &["V", "volt", "volts"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::FARAD),
            &["F", "farad", "farads"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::OHM),
            &["Ω", "ohm", "ohms"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::SIEMENS),
            &["S", "siemens"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::WEBER),
            &["Wb", "weber", "webers"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::TESLA),
            &["T", "tesla", "teslas"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::HENRY),
            &["H", "henry", "henrys", "henries"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::LUMEN),
            &["lm", "lumen", "lumens"][..],
        ),
        (Prefix::NONE, Unit::Derived(units::LUX), &["lx", "lux"][..]),
        (
            Prefix::NONE,
            Unit::Derived(units::BECQUEREL),
            &["Bq", "becquerel", "becquerels"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::GRAY),
            &["Gy", "gray", "grays"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::SIEVERT),
            &["Sv", "sievert", "sieverts"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::KATAL),
            &["kat", "katal", "katals"][..],
        ),
        // misc
        (
            Prefix::NONE,
            Unit::Derived(distances::NAUTICAL_MILE),
            &["NM", "nmi"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(velocities::KNOT),
            &["kt", "knot", "knots"][..],
        ),
        // imperial units
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::THOU),
            &["th", "thou", "thous"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::BARLEYCORN),
            &["Bc", "barleycorn", "barleycorns"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::INCH),
            &["in", "inch", "inches"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::HAND),
            &["hand", "hands"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::FOOT),
            &["ft", "feet", "feets"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::YARD),
            &["yd", "yard", "yards"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::CHAIN),
            &["ch", "chain", "chains"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::FURLONG),
            &["fur", "furlong", "furlongs"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::MILE),
            &["mi", "mile", "miles"][..],
        ),
        (
            Prefix::NONE,
            Unit::Derived(units::imperial::LEAGUE),
            &["lea", "league", "leagues"][..],
        ),
    ];

    for (prefix_bias, unit, variants) in units.iter().copied() {
        for v in variants.iter().copied() {
            assert_eq! {
                parse!(v).as_deref(),
                Ok(&[(0 + prefix_bias, unit)][..]),
                "`{}`",
                v
            };
        }

        for (p, strings) in PREFIXES.iter().copied() {
            for prefix in strings.iter().copied() {
                for variant in variants.iter().copied() {
                    // NB: dash to separate is always guaranteed to work, while
                    // others might be ambiguous.
                    let s = format!("{}-{}", prefix, variant);

                    assert_eq! {
                        parse!(&s).as_deref(),
                        Ok(&[(p + prefix_bias, unit)][..]),
                        "`{}`; prefix=`{prefix}`, variant=`{variant}`",
                        s,
                        prefix = prefix,
                        variant = variant,
                    };
                }
            }
        }
    }
}
