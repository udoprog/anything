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
    #[token("metre")]
    #[token("meter")]
    #[token("meters")]
    Meter,
    #[token("g")]
    #[token("gram")]
    Gram,
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
    #[token("ton")]
    #[token("tons")]
    #[token("tonne")]
    #[token("tonnes")]
    Tonne,
    #[token("Da")]
    #[token("dalton")]
    #[token("daltons")]
    Dalton,
    #[token("au")]
    Au,
    #[token("ftm")]
    #[token("fathom")]
    #[token("fathoms")]
    Fathom,
    #[token("cable")]
    #[token("cables")]
    Cable,
    #[token("NM")]
    #[token("nmi")]
    NauticalMile,
    #[token("link")]
    #[token("links")]
    Link,
    #[token("rd")]
    #[token("rod")]
    #[token("rods")]
    Rod,
    #[token("ha")]
    #[token("hectare")]
    #[token("hectares")]
    Hectare,
    #[token("l")]
    #[token("L")]
    #[token("litre")]
    #[token("litres")]
    Litre,
    #[token("cc")]
    CubicCentimetre,
    #[token("perch")]
    #[token("perches")]
    Perch,
    #[token("rood")]
    #[token("roods")]
    Rood,
    #[token("acre")]
    #[token("acres")]
    Acre,
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
    #[token("eV")]
    #[token("electronvolt")]
    #[token("electronvolts")]
    Electronvolt,
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
    #[token("gr")]
    #[token("grain")]
    #[token("grains")]
    Grain,
    #[token("dr")]
    #[token("drachm")]
    #[token("drachms")]
    Drachm,
    #[token("oz")]
    #[token("ounce")]
    #[token("ounces")]
    Ounce,
    #[token("lb")]
    #[token("pound")]
    #[token("pounds")]
    Pound,
    #[token("st")]
    #[token("stone")]
    #[token("stones")]
    Stone,
    #[token("qr")]
    #[token("qtr")]
    #[token("quarter")]
    #[token("quarters")]
    Quarter,
    #[token("cwt")]
    #[token("hundredweight")]
    #[token("hundredweights")]
    Hundredweight,
    #[token("t")]
    ImperialTon,
    #[token("slug")]
    #[token("slugs")]
    Slug,
    #[token("°C")]
    #[token("celsius")]
    Celsius,
    #[token("°F")]
    #[token("fahrenheit")]
    Fahrenheit,
    #[token("sp")]
    SpecificImpulse,
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
    #[token("m")]
    #[token("metre")]
    #[token("meter")]
    #[token("meters")]
    Meter,
    #[token("g")]
    #[token("gram")]
    Gram,
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
    #[token("minute")]
    #[token("minutes")]
    #[token("min")]
    #[token("mins")]
    Minute,
    #[token("h")]
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
    #[token("ton")]
    #[token("tons")]
    #[token("tonne")]
    #[token("tonnes")]
    Tonne,
    #[token("Da")]
    #[token("dalton")]
    #[token("daltons")]
    Dalton,
    #[token("au")]
    Au,
    #[token("ftm")]
    #[token("fathom")]
    #[token("fathoms")]
    Fathom,
    #[token("cable")]
    #[token("cables")]
    Cable,
    #[token("NM")]
    #[token("nmi")]
    NauticalMile,
    #[token("link")]
    #[token("links")]
    Link,
    #[token("rd")]
    #[token("rod")]
    #[token("rods")]
    Rod,
    #[token("ha")]
    #[token("hectare")]
    #[token("hectares")]
    Hectare,
    #[token("l")]
    #[token("L")]
    #[token("litre")]
    #[token("litres")]
    Litre,
    #[token("cc")]
    CubicCentimetre,
    #[token("perch")]
    #[token("perches")]
    Perch,
    #[token("rood")]
    #[token("roods")]
    Rood,
    #[token("acre")]
    #[token("acres")]
    Acre,
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
    #[token("eV")]
    #[token("electronvolt")]
    #[token("electronvolts")]
    Electronvolt,
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
    #[token("gr")]
    #[token("grain")]
    #[token("grains")]
    Grain,
    #[token("dr")]
    #[token("drachm")]
    #[token("drachms")]
    Drachm,
    #[token("oz")]
    #[token("ounce")]
    #[token("ounces")]
    Ounce,
    #[token("lb")]
    #[token("pound")]
    #[token("pounds")]
    Pound,
    #[token("st")]
    #[token("stone")]
    #[token("stones")]
    Stone,
    #[token("qr")]
    #[token("qtr")]
    #[token("quarter")]
    #[token("quarters")]
    Quarter,
    #[token("cwt")]
    #[token("hundredweight")]
    #[token("hundredweights")]
    Hundredweight,
    #[token("t")]
    ImperialTon,
    #[token("slug")]
    #[token("slugs")]
    Slug,
    #[token("°C")]
    #[token("celsius")]
    Celsius,
    #[token("°F")]
    #[token("fahrenheit")]
    Fahrenheit,
    #[token("sp")]
    SpecificImpulse,
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
            Combined::Meter => Unit::Meter,
            Combined::Gram => {
                prefix += -3;
                Unit::KiloGram
            }
            Combined::Ampere => Unit::Ampere,
            Combined::Kelvin => Unit::Kelvin,
            Combined::Mole => Unit::Mole,
            Combined::Candela => Unit::Candela,
            Combined::Byte => Unit::Byte,
            Combined::Minute => Unit::Derived(units::time::MINUTE),
            Combined::Hour => Unit::Derived(units::time::HOUR),
            Combined::Day => Unit::Derived(units::time::DAY),
            Combined::Week => Unit::Derived(units::time::WEEK),
            Combined::Month => Unit::Derived(units::time::MONTH),
            Combined::Year => Unit::Derived(units::time::YEAR),
            Combined::Decade => Unit::Derived(units::time::DECADE),
            Combined::Century => Unit::Derived(units::time::CENTURY),
            Combined::Millenium => Unit::Derived(units::time::MILLENIUM),
            Combined::Tonne => Unit::Derived(units::mass::TONNE),
            Combined::Dalton => Unit::Derived(units::mass::DALTON),
            Combined::Au => Unit::Derived(units::length::AU),
            Combined::Fathom => Unit::Derived(units::length::FATHOM),
            Combined::Cable => Unit::Derived(units::length::CABLE),
            Combined::NauticalMile => Unit::Derived(units::length::NAUTICAL_MILE),
            Combined::Link => Unit::Derived(units::length::LINK),
            Combined::Rod => Unit::Derived(units::length::ROD),
            Combined::Hectare => Unit::Derived(units::area::HECTARE),
            Combined::Litre => Unit::Derived(units::volume::LITRE),
            Combined::CubicCentimetre => Unit::Derived(units::volume::CUBIC_CENTIMETER),
            Combined::Perch => Unit::Derived(units::area::PERCH),
            Combined::Rood => Unit::Derived(units::area::ROOD),
            Combined::Acre => Unit::Derived(units::area::ACRE),
            Combined::Acceleration => Unit::Derived(units::ACCELERATION),
            Combined::Velocity => Unit::Derived(units::VELOCITY),
            Combined::Gforce => Unit::Derived(units::GFORCE),
            Combined::Newton => Unit::Derived(units::NEWTON),
            Combined::Pascal => Unit::Derived(units::PASCAL),
            Combined::Joule => Unit::Derived(units::energy::JOULE),
            Combined::Btu => Unit::Derived(units::energy::BTU),
            Combined::Electronvolt => Unit::Derived(units::energy::ELECTRONVOLT),
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
            Combined::Knot => Unit::Derived(units::velocity::KNOT),
            Combined::Thou => Unit::Derived(units::length::THOU),
            Combined::Barleycorn => Unit::Derived(units::length::BARLEYCORN),
            Combined::Inch => Unit::Derived(units::length::INCH),
            Combined::Hand => Unit::Derived(units::length::HAND),
            Combined::Feet => Unit::Derived(units::length::FOOT),
            Combined::Yard => Unit::Derived(units::length::YARD),
            Combined::Chain => Unit::Derived(units::length::CHAIN),
            Combined::Furlong => Unit::Derived(units::length::FURLONG),
            Combined::Mile => Unit::Derived(units::length::MILE),
            Combined::League => Unit::Derived(units::length::LEAGUE),
            Combined::Grain => Unit::Derived(units::mass::GRAIN),
            Combined::Drachm => Unit::Derived(units::mass::DRACHM),
            Combined::Ounce => Unit::Derived(units::mass::OUNCE),
            Combined::Pound => Unit::Derived(units::mass::POUND),
            Combined::Stone => Unit::Derived(units::mass::STONE),
            Combined::Quarter => Unit::Derived(units::mass::QUARTER),
            Combined::Hundredweight => Unit::Derived(units::mass::HUNDREDWEIGHT),
            Combined::ImperialTon => Unit::Derived(units::mass::TON),
            Combined::Slug => Unit::Derived(units::mass::SLUG),
            Combined::Celsius => Unit::Derived(units::temperature::CELSIUS),
            Combined::Fahrenheit => Unit::Derived(units::temperature::FAHRENHEIT),
            Combined::SpecificImpulse => Unit::Derived(units::SPECIFIC_IMPULSE),
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
                    return Some(("", prefix, Unit::Derived(units::time::MILLENIUM)));
                }

                prefix += Prefix::MEGA;
                break;
            }
            Combined::Kilo => {
                prefix += Prefix::KILO;
                break;
            }
            Combined::Hecto => {
                if lexer.remainder().is_empty() {
                    return Some(("", prefix, Unit::Derived(units::time::HOUR)));
                }

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
                    return Some(("", prefix, Unit::Derived(units::velocity::LIGHT_SPEED)));
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
                    return Some(("", prefix, Unit::Derived(units::time::YEAR)));
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
            Units::Meter => {
                break Unit::Meter;
            }
            Units::Gram => {
                prefix += -3;
                break Unit::KiloGram;
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
            Units::Minute => {
                break Unit::Derived(units::time::MINUTE);
            }
            Units::Hour => {
                break Unit::Derived(units::time::HOUR);
            }
            Units::Day => {
                break Unit::Derived(units::time::DAY);
            }
            Units::Week => {
                break Unit::Derived(units::time::WEEK);
            }
            Units::Month => {
                break Unit::Derived(units::time::MONTH);
            }
            Units::Year => {
                break Unit::Derived(units::time::YEAR);
            }
            Units::Decade => {
                break Unit::Derived(units::time::DECADE);
            }
            Units::Century => {
                break Unit::Derived(units::time::CENTURY);
            }
            Units::Millenium => {
                break Unit::Derived(units::time::MILLENIUM);
            }
            Units::Tonne => {
                break Unit::Derived(units::mass::TONNE);
            }
            Units::Dalton => {
                break Unit::Derived(units::mass::DALTON);
            }
            Units::Au => {
                break Unit::Derived(units::length::AU);
            }
            Units::Fathom => {
                break Unit::Derived(units::length::FATHOM);
            }
            Units::Cable => {
                break Unit::Derived(units::length::CABLE);
            }
            Units::NauticalMile => {
                break Unit::Derived(units::length::NAUTICAL_MILE);
            }
            Units::Link => {
                break Unit::Derived(units::length::LINK);
            }
            Units::Rod => {
                break Unit::Derived(units::length::ROD);
            }
            Units::Hectare => {
                break Unit::Derived(units::area::HECTARE);
            }
            Units::Litre => {
                break Unit::Derived(units::volume::LITRE);
            }
            Units::CubicCentimetre => {
                break Unit::Derived(units::volume::CUBIC_CENTIMETER);
            }
            Units::Perch => {
                break Unit::Derived(units::area::PERCH);
            }
            Units::Rood => {
                break Unit::Derived(units::area::ROOD);
            }
            Units::Acre => {
                break Unit::Derived(units::area::ACRE);
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
                break Unit::Derived(units::energy::JOULE);
            }
            Units::Btu => {
                break Unit::Derived(units::energy::BTU);
            }
            Units::Electronvolt => {
                break Unit::Derived(units::energy::ELECTRONVOLT);
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
            Units::LightSpeed => {
                break Unit::Derived(units::velocity::LIGHT_SPEED);
            }
            Units::Knot => {
                break Unit::Derived(units::velocity::KNOT);
            }
            Units::Thou => {
                break Unit::Derived(units::length::THOU);
            }
            Units::Barleycorn => {
                break Unit::Derived(units::length::BARLEYCORN);
            }
            Units::Inch => {
                break Unit::Derived(units::length::INCH);
            }
            Units::Hand => {
                break Unit::Derived(units::length::HAND);
            }
            Units::Feet => {
                break Unit::Derived(units::length::FOOT);
            }
            Units::Yard => {
                break Unit::Derived(units::length::YARD);
            }
            Units::Chain => {
                break Unit::Derived(units::length::CHAIN);
            }
            Units::Furlong => {
                break Unit::Derived(units::length::FURLONG);
            }
            Units::Mile => {
                break Unit::Derived(units::length::MILE);
            }
            Units::League => {
                break Unit::Derived(units::length::LEAGUE);
            }
            Units::Grain => {
                break Unit::Derived(units::mass::GRAIN);
            }
            Units::Drachm => {
                break Unit::Derived(units::mass::DRACHM);
            }
            Units::Ounce => {
                break Unit::Derived(units::mass::OUNCE);
            }
            Units::Pound => {
                break Unit::Derived(units::mass::POUND);
            }
            Units::Stone => {
                break Unit::Derived(units::mass::STONE);
            }
            Units::Quarter => {
                break Unit::Derived(units::mass::QUARTER);
            }
            Units::Hundredweight => {
                break Unit::Derived(units::mass::HUNDREDWEIGHT);
            }
            Units::ImperialTon => {
                break Unit::Derived(units::mass::TON);
            }
            Units::Slug => {
                break Unit::Derived(units::mass::SLUG);
            }
            Units::Celsius => {
                break Unit::Derived(units::temperature::CELSIUS);
            }
            Units::Fahrenheit => {
                break Unit::Derived(units::temperature::FAHRENHEIT);
            }
            Units::SpecificImpulse => {
                break Unit::Derived(units::SPECIFIC_IMPULSE);
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
