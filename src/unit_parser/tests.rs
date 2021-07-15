use super::UnitParser;
use crate::prefix::Prefix;
use crate::unit::Unit;
use crate::units;

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
        (Unit::Second, &["s"][..]),
        (Unit::KiloGram, &["kg"][..]),
        (Unit::Meter, &["m", "meter", "meters"][..]),
        (Unit::Ampere, &["A", "ampere", "amperes"][..]),
        (Unit::Kelvin, &["K", "kelvin", "kelvins"][..]),
        (Unit::Mole, &["mol", "mols", "mole", "moles"][..]),
        (Unit::Candela, &["cd", "candela", "candelas"][..]),
        (Unit::Byte, &["B", "byte"][..]),
        (
            Unit::Derived(units::ACCELERATION),
            &["a", "acc", "acceleration"],
        ),
        (Unit::Derived(units::VELOCITY), &["v", "vel", "velocity"]),
        (Unit::Derived(units::BTU), &["btu"][..]),
        (Unit::Derived(units::AU), &["au"][..]),
        (Unit::Derived(units::LIGHTSPEED), &["c"][..]),
        (Unit::Derived(units::JOULE), &["J", "joule"][..]),
        (Unit::Derived(units::GFORCE), &["gforce", "g-force"][..]),
        (Unit::Derived(units::TON), &["ton", "tons"][..]),
        (Unit::Derived(units::YEAR), &["yr", "year", "years"][..]),
        (Unit::Derived(units::DECADE), &["decade", "decades"][..]),
        (Unit::Derived(units::CENTURY), &["century", "centuries"][..]),
        (Unit::Derived(units::MILLENIUM), &["M", "millenium"][..]),
        (
            Unit::Derived(units::MONTH),
            &["mth", "mths", "month", "months"][..],
        ),
        (Unit::Derived(units::WEEK), &["wk", "week", "weeks"][..]),
        (Unit::Derived(units::DAY), &["dy", "day", "days"][..]),
        (Unit::Derived(units::HOUR), &["hr", "hour", "hours"][..]),
        (
            Unit::Derived(units::MINUTE),
            &["min", "mins", "minute", "minutes"][..],
        ),
        (
            Unit::Derived(units::NEWTON),
            &["N", "newton", "newtons"][..],
        ),
        (
            Unit::Derived(units::PASCAL),
            &["Pa", "pascal", "pascals"][..],
        ),
        (Unit::Derived(units::WATT), &["W", "watt", "watts"][..]),
        (
            Unit::Derived(units::COULOMB),
            &["C", "coulomb", "coulombs"][..],
        ),
        (Unit::Derived(units::VOLT), &["V", "volt", "volts"][..]),
        (Unit::Derived(units::FARAD), &["F", "farad", "farads"][..]),
        (Unit::Derived(units::OHM), &["Ω", "ohm", "ohms"][..]),
        (Unit::Derived(units::SIEMENS), &["S", "siemens"][..]),
        (Unit::Derived(units::WEBER), &["Wb", "weber", "webers"][..]),
        (Unit::Derived(units::TESLA), &["T", "tesla", "teslas"][..]),
        (
            Unit::Derived(units::HENRY),
            &["H", "henry", "henrys", "henries"][..],
        ),
        (Unit::Derived(units::LUMEN), &["lm", "lumen", "lumens"][..]),
        (Unit::Derived(units::LUX), &["lx", "lux"][..]),
        (
            Unit::Derived(units::BECQUEREL),
            &["Bq", "becquerel", "becquerels"][..],
        ),
        (Unit::Derived(units::GRAY), &["Gy", "gray", "grays"][..]),
        (
            Unit::Derived(units::SIEVERT),
            &["Sv", "sievert", "sieverts"][..],
        ),
        (Unit::Derived(units::KATAL), &["kat", "katal", "katals"][..]),
    ];

    for (unit, variants) in units.iter().copied() {
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
                for variant in variants.iter().copied() {
                    // NB: dash to separate is always guaranteed to work, while
                    // others might be ambiguous.
                    let s = format!("{}-{}", string, variant);

                    assert_eq! {
                        parse!(&s).as_deref(),
                        Ok(&[(prefix, unit)][..]),
                        "`{}`; prefix=`{prefix}`, variant=`{variant}`",
                        s,
                        prefix = string,
                        variant = variant,
                    };
                }
            }
        }
    }
}
