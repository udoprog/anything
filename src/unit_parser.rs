use crate::unit::{Name, Prefix};
use anyhow::{anyhow, Result};
use logos::Logos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedUnit {
    pub prefix: i32,
    pub name: Name,
}

impl ParsedUnit {
    pub fn new(prefix: i32, name: Name) -> Self {
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
    #[token("W")]
    Week,
    #[token("month")]
    #[token("months")]
    Month,
    #[token("year")]
    #[token("years")]
    Year,

    #[token("metre")]
    #[token("meter")]
    #[token("meters")]
    Meter,

    #[token("g")]
    #[token("gram")]
    Gram,

    #[token("B")]
    #[token("byte")]
    Byte,

    #[token("J")]
    #[token("joule")]
    Joule,
    #[token("btu")]
    Btu,

    #[token("au")]
    Au,

    #[token("Y")]
    YottaOrYear,
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
    MegaOrMonth,
    #[token("mega")]
    Mega,
    #[token("k")]
    #[token("kilo")]
    Kilo,
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
    #[token("atto")]
    Atto,
    #[token("z")]
    #[token("zepto")]
    Zepto,
    #[token("y")]
    #[token("yocto")]
    Yocto,

    #[error]
    Error,
}

/// Helper to parse collection of units from a string.
pub struct UnitParser<'a> {
    lexer: logos::Lexer<'a, Token>,
}

impl<'a> UnitParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Token::lexer(source),
        }
    }

    /// Parse the next unit and base.
    pub fn next(&mut self) -> Result<Option<ParsedUnit>> {
        let mut prefix = 0;

        while let Some(token) = self.lexer.next() {
            match token {
                Token::Second => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Second)));
                }
                Token::Gram => {
                    return Ok(Some(ParsedUnit::new(prefix - 3, Name::KiloGram)));
                }
                Token::Joule => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Joule)));
                }
                Token::Byte => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Byte)));
                }
                Token::Minute => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Minute)));
                }
                Token::Hour => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Hour)));
                }
                Token::Day => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Day)));
                }
                Token::Week => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Week)));
                }
                Token::Month => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Month)));
                }
                Token::Year => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Year)));
                }
                Token::Btu => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Btu)));
                }
                Token::Au => {
                    return Ok(Some(ParsedUnit::new(prefix, Name::Au)));
                }
                Token::YottaOrYear => {
                    if self.lexer.remainder().is_empty() {
                        return Ok(Some(ParsedUnit::new(prefix, Name::Year)));
                    }

                    prefix += Prefix::YOTTA;
                }
                Token::Yotta => {
                    prefix += Prefix::YOTTA;
                }
                Token::Zetta => {
                    prefix += Prefix::ZETTA;
                }
                Token::Exa => {
                    prefix += Prefix::EXA;
                }
                Token::Peta => {
                    prefix += Prefix::PETA;
                }
                Token::Tera => {
                    prefix += Prefix::TERA;
                }
                Token::Giga => {
                    prefix += Prefix::GIGA;
                }
                Token::MegaOrMonth => {
                    if self.lexer.remainder().is_empty() {
                        return Ok(Some(ParsedUnit::new(prefix, Name::Month)));
                    }

                    prefix += Prefix::MEGA;
                }
                Token::Mega => {
                    prefix += Prefix::MEGA;
                }
                Token::Kilo => {
                    prefix += Prefix::KILO;
                }
                Token::DeciOrDay => {
                    if self.lexer.remainder().is_empty() {
                        return Ok(Some(ParsedUnit::new(prefix, Name::Day)));
                    }

                    prefix += Prefix::DECI;
                }
                Token::Deci => {
                    prefix += Prefix::DECI;
                }
                Token::CentiOrLightSpeed => {
                    if self.lexer.remainder().is_empty() {
                        return Ok(Some(ParsedUnit::new(prefix, Name::LightSpeed)));
                    }

                    prefix += Prefix::CENTI;
                }
                Token::Centi => {
                    prefix += Prefix::CENTI;
                }
                Token::MilliOrMeter => {
                    if self.lexer.remainder().is_empty() {
                        return Ok(Some(ParsedUnit::new(prefix, Name::Meter)));
                    }

                    prefix += Prefix::MILLI;
                }
                Token::Milli => {
                    prefix += Prefix::MILLI;
                }
                Token::Micro => {
                    prefix += Prefix::MICRO;
                }
                Token::Nano => {
                    prefix += Prefix::NANO;
                }
                Token::Pico => {
                    prefix += Prefix::PICO;
                }
                Token::Femto => {
                    prefix += Prefix::FEMTO;
                }
                Token::Atto => {
                    prefix += Prefix::ATTO;
                }
                Token::Zepto => {
                    prefix += Prefix::ZEPTO;
                }
                Token::Yocto => {
                    prefix += Prefix::YOCTO;
                }
                _ => {
                    return Err(anyhow!("not a valid unit `{}`", self.lexer.source(),));
                }
            }
        }

        if prefix == 0 {
            Ok(None)
        } else {
            Err(anyhow!("not a valid unit `{}`", self.lexer.source(),))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ParsedUnit, UnitParser};
    use crate::unit::{Name, Prefix};

    #[test]
    fn test_kilo() {
        let mut p = UnitParser::new("kilominutes");
        assert_eq!(
            p.next().unwrap(),
            Some(ParsedUnit {
                prefix: Prefix::KILO,
                name: Name::Minute
            })
        );
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("kminutes");
        assert_eq!(
            p.next().unwrap(),
            Some(ParsedUnit {
                prefix: Prefix::KILO,
                name: Name::Minute
            })
        );
        assert!(p.next().unwrap().is_none());
    }

    #[test]
    fn test_minutes() {
        let mut p = UnitParser::new("minutes");
        assert_eq!(
            p.next().unwrap(),
            Some(ParsedUnit {
                prefix: 0,
                name: Name::Minute
            })
        );
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("minute");
        assert_eq!(
            p.next().unwrap(),
            Some(ParsedUnit {
                prefix: 0,
                name: Name::Minute
            })
        );
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("min");
        assert_eq!(
            p.next().unwrap(),
            Some(ParsedUnit {
                prefix: 0,
                name: Name::Minute
            })
        );
        assert!(p.next().unwrap().is_none());
    }

    #[test]
    fn test_prefixes() {
        let tests = [
            (&["Pg", "petagram"][..], Prefix::PETA),
            (&["Tg", "teragram"][..], Prefix::TERA),
            (&["Gg", "gigagram"][..], Prefix::GIGA),
            (&["Mg", "megagram"][..], Prefix::MEGA),
            (&["kg", "kilogram"][..], Prefix::KILO),
            (&["mg", "milligram"][..], Prefix::MILLI),
            (&["μg", "microgram"][..], Prefix::MICRO),
            (&["ng", "nanogram"][..], Prefix::NANO),
        ];

        for (tests, prefix) in tests {
            for test in tests {
                let mut p = UnitParser::new(*test);

                assert_eq!(
                    p.next().unwrap(),
                    Some(ParsedUnit {
                        prefix: prefix - 3,
                        name: Name::KiloGram
                    }),
                    "failed prefix test: test = {}, prefix = {}",
                    test,
                    prefix
                );

                assert!(
                    p.next().unwrap().is_none(),
                    "failed prefix test: test = {}, prefix = {}",
                    test,
                    prefix
                );
            }
        }
    }
}
