use crate::prefix::Prefix;
use crate::unit::Unit;
use crate::units;
use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
enum Combined {
    #[token("s")]
    #[token("sec")]
    #[token("second")]
    #[token("seconds")]
    Second,
    #[token("minute")]
    #[token("minutes")]
    #[token("min")]
    #[token("mins")]
    Minute,
    #[token("hr")]
    #[token("hour")]
    #[token("hours")]
    Hour,
    #[token("dy")]
    #[token("day")]
    #[token("days")]
    Day,
    #[token("wk")]
    #[token("week")]
    #[token("weeks")]
    Week,
    #[token("mth")]
    #[token("mths")]
    #[token("month")]
    #[token("months")]
    Month,
    #[token("yr")]
    #[token("yrs")]
    #[token("year")]
    #[token("years")]
    Year,
    #[token("decade")]
    #[token("decades")]
    Decade,
    #[token("century")]
    #[token("centuries")]
    Century,
    #[token("millenium")]
    #[token("milleniums")]
    #[token("millenia")]
    Millenium,
    #[token("metre")]
    #[token("meter")]
    #[token("meters")]
    Meter,
    #[token("g")]
    #[token("gram")]
    Gram,
    #[token("tonnes")]
    #[token("tonne")]
    #[token("tons")]
    #[token("ton")]
    Ton,
    #[token("A")]
    #[token("ampere")]
    #[token("amperes")]
    Ampere,
    #[token("K")]
    #[token("kelvin")]
    #[token("kelvins")]
    Kelvin,
    #[token("mol")]
    #[token("mols")]
    #[token("mole")]
    #[token("moles")]
    Mole,
    #[token("cd")]
    #[token("candela")]
    #[token("candelas")]
    Candela,
    #[token("B")]
    #[token("byte")]
    Byte,
    #[token("au")]
    Au,
    #[token("acc")]
    #[token("acceleration")]
    Acceleration,
    #[token("v")]
    #[token("vel")]
    #[token("velocity")]
    Velocity,
    #[token("gforce")]
    #[token("g-force")]
    Gforce,
    #[token("N")]
    #[token("newton")]
    #[token("newtons")]
    Newton,
    #[token("Pa")]
    #[token("pascal")]
    #[token("pascals")]
    Pascal,
    #[token("J")]
    #[token("joule")]
    Joule,
    #[token("btu")]
    Btu,
    #[token("W")]
    #[token("watt")]
    #[token("watts")]
    Watt,
    #[token("C")]
    #[token("coulomb")]
    #[token("coulombs")]
    Coulomb,
    #[token("V")]
    #[token("volt")]
    #[token("volts")]
    Volt,
    #[token("F")]
    #[token("farad")]
    #[token("farads")]
    Farad,
    #[token("Ω")]
    #[token("ohm")]
    #[token("ohms")]
    Ohm,
    #[token("S")]
    #[token("siemens")]
    Siemens,
    #[token("Wb")]
    #[token("weber")]
    #[token("webers")]
    Weber,
    #[token("tesla")]
    #[token("teslas")]
    Tesla,
    #[token("H")]
    #[token("henry")]
    #[token("henrys")]
    #[token("henries")]
    Henry,
    #[token("lm")]
    #[token("lumen")]
    #[token("lumens")]
    Lumen,
    #[token("lx")]
    #[token("lux")]
    Lux,
    #[token("Bq")]
    #[token("becquerel")]
    #[token("becquerels")]
    Becquerel,
    #[token("Gy")]
    #[token("gray")]
    #[token("grays")]
    Gray,
    #[token("Sv")]
    #[token("sievert")]
    #[token("sieverts")]
    Sievert,
    #[token("kat")]
    #[token("katal")]
    #[token("katals")]
    Katal,
    #[token("NM")]
    #[token("nmi")]
    NauticalMile,
    #[token("kt")]
    #[token("knot")]
    #[token("knots")]
    Knot,
    #[token("th")]
    #[token("thou")]
    #[token("thous")]
    Thou,
    #[token("Bc")]
    #[token("barleycorn")]
    #[token("barleycorns")]
    Barleycorn,
    #[token("in")]
    #[token("inch")]
    #[token("inches")]
    Inch,
    #[token("hand")]
    #[token("hands")]
    Hand,
    #[token("ft")]
    #[token("feet")]
    #[token("feets")]
    Feet,
    #[token("yd")]
    #[token("yard")]
    #[token("yards")]
    Yard,
    #[token("ch")]
    #[token("chain")]
    #[token("chains")]
    Chain,
    #[token("fur")]
    #[token("furlong")]
    #[token("furlongs")]
    Furlong,
    #[token("mi")]
    #[token("mile")]
    #[token("miles")]
    Mile,
    #[token("lea")]
    #[token("league")]
    #[token("leagues")]
    League,
    /// Prefixes
    #[token("Y")]
    #[token("yotta")]
    Yotta,
    #[token("Z")]
    #[token("zetta")]
    Zetta,
    #[token("E")]
    #[token("exa")]
    Exa,
    #[token("P")]
    #[token("peta")]
    Peta,
    #[token("T")]
    #[token("tera")]
    Tera,
    #[token("G")]
    #[token("giga")]
    Giga,
    #[token("M")]
    #[token("mega")]
    Mega,
    #[token("k")]
    #[token("kilo")]
    Kilo,
    #[token("h")]
    #[token("hecto")]
    Hecto,
    #[token("da")]
    #[token("deca")]
    Deca,
    #[token("d")]
    #[token("deci")]
    Deci,
    #[token("c")]
    #[token("centi")]
    Centi,
    #[token("m")]
    #[token("milli")]
    Milli,
    #[token("μ")]
    #[token("micro")]
    Micro,
    #[token("n")]
    #[token("nano")]
    Nano,
    #[token("p")]
    #[token("pico")]
    Pico,
    #[token("f")]
    #[token("femto")]
    Femto,
    #[token("a")]
    #[token("atto")]
    Atto,
    #[token("z")]
    #[token("zepto")]
    Zepto,
    #[token("y")]
    #[token("yocto")]
    Yocto,
    #[token("-")]
    Separator,
    #[error]
    Error,
}

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
enum Units {
    #[token("s")]
    #[token("sec")]
    #[token("second")]
    #[token("seconds")]
    Second,
    #[token("minute")]
    #[token("minutes")]
    #[token("min")]
    #[token("mins")]
    Minute,
    #[token("hr")]
    #[token("hour")]
    #[token("hours")]
    Hour,
    #[token("dy")]
    #[token("day")]
    #[token("days")]
    Day,
    #[token("wk")]
    #[token("week")]
    #[token("weeks")]
    Week,
    #[token("mth")]
    #[token("mths")]
    #[token("month")]
    #[token("months")]
    Month,
    #[token("y")]
    #[token("yr")]
    #[token("yrs")]
    #[token("year")]
    #[token("years")]
    Year,
    #[token("decade")]
    #[token("decades")]
    Decade,
    #[token("century")]
    #[token("centuries")]
    Century,
    #[token("M")]
    #[token("millenium")]
    #[token("milleniums")]
    #[token("millenia")]
    Millenium,
    #[token("m")]
    #[token("metre")]
    #[token("meter")]
    #[token("meters")]
    Meter,
    #[token("g")]
    #[token("gram")]
    Gram,
    #[token("tonnes")]
    #[token("tonne")]
    #[token("tons")]
    #[token("ton")]
    Ton,
    #[token("A")]
    #[token("ampere")]
    #[token("amperes")]
    Ampere,
    #[token("K")]
    #[token("kelvin")]
    #[token("kelvins")]
    Kelvin,
    #[token("mol")]
    #[token("mols")]
    #[token("mole")]
    #[token("moles")]
    Mole,
    #[token("cd")]
    #[token("candela")]
    #[token("candelas")]
    Candela,
    #[token("B")]
    #[token("byte")]
    Byte,
    #[token("au")]
    Au,
    #[token("a")]
    #[token("acc")]
    #[token("acceleration")]
    Acceleration,
    #[token("v")]
    #[token("vel")]
    #[token("velocity")]
    Velocity,
    #[token("gforce")]
    #[token("g-force")]
    Gforce,
    #[token("N")]
    #[token("newton")]
    #[token("newtons")]
    Newton,
    #[token("Pa")]
    #[token("pascal")]
    #[token("pascals")]
    Pascal,
    #[token("J")]
    #[token("joule")]
    Joule,
    #[token("btu")]
    Btu,
    #[token("W")]
    #[token("watt")]
    #[token("watts")]
    Watt,
    #[token("C")]
    #[token("coulomb")]
    #[token("coulombs")]
    Coulomb,
    #[token("V")]
    #[token("volt")]
    #[token("volts")]
    Volt,
    #[token("F")]
    #[token("farad")]
    #[token("farads")]
    Farad,
    #[token("Ω")]
    #[token("ohm")]
    #[token("ohms")]
    Ohm,
    #[token("S")]
    #[token("siemens")]
    Siemens,
    #[token("Wb")]
    #[token("weber")]
    #[token("webers")]
    Weber,
    #[token("T")]
    #[token("tesla")]
    #[token("teslas")]
    Tesla,
    #[token("H")]
    #[token("henry")]
    #[token("henrys")]
    #[token("henries")]
    Henry,
    #[token("lm")]
    #[token("lumen")]
    #[token("lumens")]
    Lumen,
    #[token("lx")]
    #[token("lux")]
    Lux,
    #[token("Bq")]
    #[token("becquerel")]
    #[token("becquerels")]
    Becquerel,
    #[token("Gy")]
    #[token("gray")]
    #[token("grays")]
    Gray,
    #[token("Sv")]
    #[token("sievert")]
    #[token("sieverts")]
    Sievert,
    #[token("kat")]
    #[token("katal")]
    #[token("katals")]
    Katal,
    #[token("NM")]
    #[token("nmi")]
    NauticalMile,
    #[token("c")]
    LightSpeed,
    #[token("kt")]
    #[token("knot")]
    #[token("knots")]
    Knot,
    #[token("th")]
    #[token("thou")]
    #[token("thous")]
    Thou,
    #[token("Bc")]
    #[token("barleycorn")]
    #[token("barleycorns")]
    Barleycorn,
    #[token("in")]
    #[token("inch")]
    #[token("inches")]
    Inch,
    #[token("hand")]
    #[token("hands")]
    Hand,
    #[token("ft")]
    #[token("feet")]
    #[token("feets")]
    Feet,
    #[token("yd")]
    #[token("yard")]
    #[token("yards")]
    Yard,
    #[token("ch")]
    #[token("chain")]
    #[token("chains")]
    Chain,
    #[token("fur")]
    #[token("furlong")]
    #[token("furlongs")]
    Furlong,
    #[token("mi")]
    #[token("mile")]
    #[token("miles")]
    Mile,
    #[token("lea")]
    #[token("league")]
    #[token("leagues")]
    League,
    #[token("-")]
    Separator,
    #[error]
    Error,
}

/// Generated unit parsing function
pub fn parse(s: &str) -> Option<(&str, i32, Unit)> {
    let mut lexer = Combined::lexer(s);
    let mut prefix = 0;

    loop {
        let token = lexer.next()?;

        let unit = match token {
            Combined::Second => Unit::Second,
            Combined::Minute => Unit::Derived(units::times::MINUTE),
            Combined::Hour => Unit::Derived(units::times::HOUR),
            Combined::Day => Unit::Derived(units::times::DAY),
            Combined::Week => Unit::Derived(units::times::WEEK),
            Combined::Month => Unit::Derived(units::times::MONTH),
            Combined::Year => Unit::Derived(units::times::YEAR),
            Combined::Decade => Unit::Derived(units::times::DECADE),
            Combined::Century => Unit::Derived(units::times::CENTURY),
            Combined::Millenium => Unit::Derived(units::times::MILLENIUM),
            Combined::Meter => Unit::Meter,
            Combined::Gram => {
                prefix += -3;
                Unit::KiloGram
            }
            Combined::Ton => Unit::Derived(units::TON),
            Combined::Ampere => Unit::Ampere,
            Combined::Kelvin => Unit::Kelvin,
            Combined::Mole => Unit::Mole,
            Combined::Candela => Unit::Candela,
            Combined::Byte => Unit::Byte,
            Combined::Au => Unit::Derived(units::distances::AU),
            Combined::Acceleration => Unit::Derived(units::ACCELERATION),
            Combined::Velocity => Unit::Derived(units::VELOCITY),
            Combined::Gforce => Unit::Derived(units::GFORCE),
            Combined::Newton => Unit::Derived(units::NEWTON),
            Combined::Pascal => Unit::Derived(units::PASCAL),
            Combined::Joule => Unit::Derived(units::JOULE),
            Combined::Btu => Unit::Derived(units::BTU),
            Combined::Watt => Unit::Derived(units::WATT),
            Combined::Coulomb => Unit::Derived(units::COULOMB),
            Combined::Volt => Unit::Derived(units::VOLT),
            Combined::Farad => Unit::Derived(units::FARAD),
            Combined::Ohm => Unit::Derived(units::OHM),
            Combined::Siemens => Unit::Derived(units::SIEMENS),
            Combined::Weber => Unit::Derived(units::WEBER),
            Combined::Tesla => Unit::Derived(units::TESLA),
            Combined::Henry => Unit::Derived(units::HENRY),
            Combined::Lumen => Unit::Derived(units::LUMEN),
            Combined::Lux => Unit::Derived(units::LUX),
            Combined::Becquerel => Unit::Derived(units::BECQUEREL),
            Combined::Gray => Unit::Derived(units::GRAY),
            Combined::Sievert => Unit::Derived(units::SIEVERT),
            Combined::Katal => Unit::Derived(units::KATAL),
            Combined::NauticalMile => Unit::Derived(units::distances::NAUTICAL_MILE),
            Combined::Knot => Unit::Derived(units::velocities::KNOT),
            Combined::Thou => Unit::Derived(units::imperial::THOU),
            Combined::Barleycorn => Unit::Derived(units::imperial::BARLEYCORN),
            Combined::Inch => Unit::Derived(units::imperial::INCH),
            Combined::Hand => Unit::Derived(units::imperial::HAND),
            Combined::Feet => Unit::Derived(units::imperial::FOOT),
            Combined::Yard => Unit::Derived(units::imperial::YARD),
            Combined::Chain => Unit::Derived(units::imperial::CHAIN),
            Combined::Furlong => Unit::Derived(units::imperial::FURLONG),
            Combined::Mile => Unit::Derived(units::imperial::MILE),
            Combined::League => Unit::Derived(units::imperial::LEAGUE),
            Combined::Yotta => {
                prefix += Prefix::YOTTA;
                break;
            }
            Combined::Zetta => {
                prefix += Prefix::ZETTA;
                break;
            }
            Combined::Exa => {
                prefix += Prefix::EXA;
                break;
            }
            Combined::Peta => {
                prefix += Prefix::PETA;
                break;
            }
            Combined::Tera => {
                if lexer.remainder().is_empty() {
                    return Some(("", prefix, Unit::Derived(units::TESLA)));
                }

                prefix += Prefix::TERA;
                break;
            }
            Combined::Giga => {
                prefix += Prefix::GIGA;
                break;
            }
            Combined::Mega => {
                if lexer.remainder().is_empty() {
                    return Some(("", prefix, Unit::Derived(units::times::MILLENIUM)));
                }

                prefix += Prefix::MEGA;
                break;
            }
            Combined::Kilo => {
                prefix += Prefix::KILO;
                break;
            }
            Combined::Hecto => {
                prefix += Prefix::HECTO;
                break;
            }
            Combined::Deca => {
                prefix += Prefix::DECA;
                break;
            }
            Combined::Deci => {
                prefix += Prefix::DECI;
                break;
            }
            Combined::Centi => {
                if lexer.remainder().is_empty() {
                    return Some(("", prefix, Unit::Derived(units::velocities::LIGHT_SPEED)));
                }

                prefix += Prefix::CENTI;
                break;
            }
            Combined::Milli => {
                if lexer.remainder().is_empty() {
                    return Some(("", prefix, Unit::Meter));
                }

                prefix += Prefix::MILLI;
                break;
            }
            Combined::Micro => {
                prefix += Prefix::MICRO;
                break;
            }
            Combined::Nano => {
                prefix += Prefix::NANO;
                break;
            }
            Combined::Pico => {
                prefix += Prefix::PICO;
                break;
            }
            Combined::Femto => {
                prefix += Prefix::FEMTO;
                break;
            }
            Combined::Atto => {
                if lexer.remainder().is_empty() {
                    return Some(("", prefix, Unit::Derived(units::ACCELERATION)));
                }

                prefix += Prefix::ATTO;
                break;
            }
            Combined::Zepto => {
                prefix += Prefix::ZEPTO;
                break;
            }
            Combined::Yocto => {
                if lexer.remainder().is_empty() {
                    return Some(("", prefix, Unit::Derived(units::times::YEAR)));
                }

                prefix += Prefix::YOCTO;
                break;
            }
            Combined::Separator => {
                continue;
            }
            Combined::Error => {
                return None;
            }
        };

        return Some((lexer.remainder(), prefix, unit));
    }

    let mut lexer = Units::lexer(lexer.remainder());

    let unit = loop {
        let token = lexer.next()?;

        match token {
            Units::Second => {
                break Unit::Second;
            }
            Units::Minute => {
                break Unit::Derived(units::times::MINUTE);
            }
            Units::Hour => {
                break Unit::Derived(units::times::HOUR);
            }
            Units::Day => {
                break Unit::Derived(units::times::DAY);
            }
            Units::Week => {
                break Unit::Derived(units::times::WEEK);
            }
            Units::Month => {
                break Unit::Derived(units::times::MONTH);
            }
            Units::Year => {
                break Unit::Derived(units::times::YEAR);
            }
            Units::Decade => {
                break Unit::Derived(units::times::DECADE);
            }
            Units::Century => {
                break Unit::Derived(units::times::CENTURY);
            }
            Units::Millenium => {
                break Unit::Derived(units::times::MILLENIUM);
            }
            Units::Meter => {
                break Unit::Meter;
            }
            Units::Gram => {
                prefix += -3;
                break Unit::KiloGram;
            }
            Units::Ton => {
                break Unit::Derived(units::TON);
            }
            Units::Ampere => {
                break Unit::Ampere;
            }
            Units::Kelvin => {
                break Unit::Kelvin;
            }
            Units::Mole => {
                break Unit::Mole;
            }
            Units::Candela => {
                break Unit::Candela;
            }
            Units::Byte => {
                break Unit::Byte;
            }
            Units::Au => {
                break Unit::Derived(units::distances::AU);
            }
            Units::Acceleration => {
                break Unit::Derived(units::ACCELERATION);
            }
            Units::Velocity => {
                break Unit::Derived(units::VELOCITY);
            }
            Units::Gforce => {
                break Unit::Derived(units::GFORCE);
            }
            Units::Newton => {
                break Unit::Derived(units::NEWTON);
            }
            Units::Pascal => {
                break Unit::Derived(units::PASCAL);
            }
            Units::Joule => {
                break Unit::Derived(units::JOULE);
            }
            Units::Btu => {
                break Unit::Derived(units::BTU);
            }
            Units::Watt => {
                break Unit::Derived(units::WATT);
            }
            Units::Coulomb => {
                break Unit::Derived(units::COULOMB);
            }
            Units::Volt => {
                break Unit::Derived(units::VOLT);
            }
            Units::Farad => {
                break Unit::Derived(units::FARAD);
            }
            Units::Ohm => {
                break Unit::Derived(units::OHM);
            }
            Units::Siemens => {
                break Unit::Derived(units::SIEMENS);
            }
            Units::Weber => {
                break Unit::Derived(units::WEBER);
            }
            Units::Tesla => {
                break Unit::Derived(units::TESLA);
            }
            Units::Henry => {
                break Unit::Derived(units::HENRY);
            }
            Units::Lumen => {
                break Unit::Derived(units::LUMEN);
            }
            Units::Lux => {
                break Unit::Derived(units::LUX);
            }
            Units::Becquerel => {
                break Unit::Derived(units::BECQUEREL);
            }
            Units::Gray => {
                break Unit::Derived(units::GRAY);
            }
            Units::Sievert => {
                break Unit::Derived(units::SIEVERT);
            }
            Units::Katal => {
                break Unit::Derived(units::KATAL);
            }
            Units::NauticalMile => {
                break Unit::Derived(units::distances::NAUTICAL_MILE);
            }
            Units::LightSpeed => {
                break Unit::Derived(units::velocities::LIGHT_SPEED);
            }
            Units::Knot => {
                break Unit::Derived(units::velocities::KNOT);
            }
            Units::Thou => {
                break Unit::Derived(units::imperial::THOU);
            }
            Units::Barleycorn => {
                break Unit::Derived(units::imperial::BARLEYCORN);
            }
            Units::Inch => {
                break Unit::Derived(units::imperial::INCH);
            }
            Units::Hand => {
                break Unit::Derived(units::imperial::HAND);
            }
            Units::Feet => {
                break Unit::Derived(units::imperial::FOOT);
            }
            Units::Yard => {
                break Unit::Derived(units::imperial::YARD);
            }
            Units::Chain => {
                break Unit::Derived(units::imperial::CHAIN);
            }
            Units::Furlong => {
                break Unit::Derived(units::imperial::FURLONG);
            }
            Units::Mile => {
                break Unit::Derived(units::imperial::MILE);
            }
            Units::League => {
                break Unit::Derived(units::imperial::LEAGUE);
            }
            Units::Separator => {
                continue;
            }
            Units::Error => {
                return None;
            }
        }
    };

    Some((lexer.remainder(), prefix, unit))
}
