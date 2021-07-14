use crate::prefix::Prefix;
use crate::unit::Unit;
use logos::Logos;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedUnit {
    pub prefix: i32,
    pub name: Unit,
}

impl ParsedUnit {
    pub fn new(prefix: i32, name: Unit) -> Self {
        Self { prefix, name }
    }
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
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
    #[token("hour")]
    #[token("hours")]
    #[token("H")]
    Hour,
    #[token("day")]
    #[token("days")]
    Day,
    #[token("week")]
    #[token("weeks")]
    #[token("w")]
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
    GramOrGforce,
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
    MegaOrMillenium,
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
    DeciOrDay,
    #[token("deci")]
    Deci,
    #[token("c")]
    CentiOrLightSpeed,
    #[token("centi")]
    Centi,
    #[token("m")]
    MilliOrMeter,
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
    AttoOrAcceleration,
    #[token("atto")]
    Atto,
    #[token("z")]
    #[token("zepto")]
    Zepto,
    #[token("y")]
    YoctoOrYear,
    #[token("yocto")]
    Yocto,

    #[token("-")]
    Separator,

    #[error]
    Error,
}

/// Helper to parse collection of units from a string.
pub struct UnitParser<'a> {
    lexer: logos::Lexer<'a, Token>,
    acceleration_bias: bool,
}

impl<'a> UnitParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Token::lexer(source),
            acceleration_bias: false,
        }
    }

    /// Make the parser have acceleration bias.
    pub fn with_acceleration_bias(self, acceleration_bias: bool) -> Self {
        Self {
            acceleration_bias,
            ..self
        }
    }

    fn inner_next(&mut self) -> Result<Option<(i32, Unit)>, ()> {
        let mut prefix = 0;

        loop {
            let token = match self.lexer.next() {
                Some(token) => token,
                None => return Ok(None),
            };

            let unit = match token {
                Token::Second => Unit::Second,
                Token::GramOrGforce => {
                    if self.acceleration_bias {
                        Unit::Gforce
                    } else {
                        prefix -= 3;
                        Unit::KiloGram
                    }
                }
                Token::Gram => {
                    prefix -= 3;
                    Unit::KiloGram
                }
                Token::Ampere => Unit::Ampere,
                Token::Kelvin => Unit::Kelvin,
                Token::Mole => Unit::Mole,
                Token::Candela => Unit::Candela,
                Token::Ton => Unit::Ton,
                Token::Byte => Unit::Byte,
                Token::Minute => Unit::Minute,
                Token::Hour => Unit::Hour,
                Token::Day => Unit::Day,
                Token::Week => Unit::Week,
                Token::Month => Unit::Month,
                Token::Year => Unit::Year,
                Token::Decade => Unit::Decade,
                Token::Century => Unit::Century,
                Token::Millenium => Unit::Millenium,
                Token::Meter => Unit::Meter,
                Token::Btu => Unit::Btu,
                Token::Au => Unit::Au,
                Token::Acceleration => Unit::Acceleration,
                Token::Gforce => Unit::Gforce,
                Token::Newton => Unit::Newton,
                Token::Pascal => Unit::Pascal,
                Token::Joule => Unit::Joule,
                Token::Watt => Unit::Watt,
                Token::Yotta => {
                    prefix += Prefix::YOTTA;
                    continue;
                }
                Token::Zetta => {
                    prefix += Prefix::ZETTA;
                    continue;
                }
                Token::Exa => {
                    prefix += Prefix::EXA;
                    continue;
                }
                Token::Peta => {
                    prefix += Prefix::PETA;
                    continue;
                }
                Token::Tera => {
                    prefix += Prefix::TERA;
                    continue;
                }
                Token::Giga => {
                    prefix += Prefix::GIGA;
                    continue;
                }
                Token::MegaOrMillenium => {
                    if !self.lexer.remainder().is_empty() {
                        prefix += Prefix::MEGA;
                        continue;
                    }

                    Unit::Millenium
                }
                Token::Mega => {
                    prefix += Prefix::MEGA;
                    continue;
                }
                Token::Kilo => {
                    prefix += Prefix::KILO;
                    continue;
                }
                Token::Hecto => {
                    prefix += Prefix::HECTO;
                    continue;
                }
                Token::Deca => {
                    prefix += Prefix::DECA;
                    continue;
                }
                Token::DeciOrDay => {
                    if !self.lexer.remainder().is_empty() {
                        prefix += Prefix::DECI;
                        continue;
                    }

                    Unit::Day
                }
                Token::Deci => {
                    prefix += Prefix::DECI;
                    continue;
                }
                Token::CentiOrLightSpeed => {
                    if !self.lexer.remainder().is_empty() {
                        prefix += Prefix::CENTI;
                        continue;
                    }

                    Unit::LightSpeed
                }
                Token::Centi => {
                    prefix += Prefix::CENTI;
                    continue;
                }
                Token::MilliOrMeter => {
                    if !self.lexer.remainder().is_empty() {
                        prefix += Prefix::MILLI;
                        continue;
                    }

                    Unit::Meter
                }
                Token::Milli => {
                    prefix += Prefix::MILLI;
                    continue;
                }
                Token::Micro => {
                    prefix += Prefix::MICRO;
                    continue;
                }
                Token::Nano => {
                    prefix += Prefix::NANO;
                    continue;
                }
                Token::Pico => {
                    prefix += Prefix::PICO;
                    continue;
                }
                Token::Femto => {
                    prefix += Prefix::FEMTO;
                    continue;
                }
                Token::AttoOrAcceleration => {
                    if !self.lexer.remainder().is_empty() {
                        prefix += Prefix::ATTO;
                        continue;
                    }

                    Unit::Acceleration
                }
                Token::Atto => {
                    prefix += Prefix::ATTO;
                    continue;
                }
                Token::Zepto => {
                    prefix += Prefix::ZEPTO;
                    continue;
                }
                Token::YoctoOrYear => {
                    if !self.lexer.remainder().is_empty() {
                        prefix += Prefix::YOCTO;
                        continue;
                    }

                    Unit::Year
                }
                Token::Yocto => {
                    prefix += Prefix::YOCTO;
                    continue;
                }
                Token::Separator => {
                    continue;
                }
                Token::Error => {
                    return Err(());
                }
            };

            return Ok(Some((prefix, unit)));
        }
    }

    /// Parse the next unit and base.
    pub fn next(&mut self) -> Result<Option<ParsedUnit>, &'a str> {
        if let Some((prefix, unit)) = self.inner_next().map_err(|()| self.lexer.source())? {
            return Ok(Some(ParsedUnit::new(prefix, unit)));
        }

        Ok(None)
    }
}
