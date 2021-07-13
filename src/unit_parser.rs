use crate::unit::Special;
use crate::{Base, Prefix};
use anyhow::{anyhow, Result};
use logos::Logos;

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
    #[token("D")]
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
    #[token("BTU")]
    #[token("Btu")]
    Btu,

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
    #[token("deci")]
    #[token("d")]
    Deci,
    #[token("centi")]
    #[token("c")]
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
    pub fn next(&mut self) -> Result<Option<(Prefix, Base, Option<Special>)>> {
        let mut prefix = Prefix::None;

        while let Some(token) = self.lexer.next() {
            match token {
                Token::Second => {
                    return Ok(Some((prefix, Base::Second, None)));
                }
                Token::Gram => {
                    return Ok(Some((prefix, Base::Gram, None)));
                }
                Token::Joule => {
                    return Ok(Some((prefix, Base::Joule, None)));
                }
                Token::Byte => {
                    return Ok(Some((prefix, Base::Byte, None)));
                }
                Token::Minute => {
                    return Ok(Some((prefix, Base::Second, Some(Special::Minute))));
                }
                Token::Hour => {
                    return Ok(Some((prefix, Base::Second, Some(Special::Hour))));
                }
                Token::Day => {
                    return Ok(Some((prefix, Base::Second, Some(Special::Day))));
                }
                Token::Week => {
                    return Ok(Some((prefix, Base::Second, Some(Special::Week))));
                }
                Token::Month => {
                    return Ok(Some((prefix, Base::Second, Some(Special::Month))));
                }
                Token::Year => {
                    return Ok(Some((prefix, Base::Second, Some(Special::Year))));
                }
                Token::Btu => {
                    return Ok(Some((prefix, Base::Joule, Some(Special::Btu))));
                }
                Token::YottaOrYear if prefix.is_none() => {
                    if self.lexer.remainder().is_empty() || !prefix.is_none() {
                        return Ok(Some((prefix, Base::Second, Some(Special::Year))));
                    }

                    prefix = Prefix::Yotta;
                }
                Token::Yotta if prefix.is_none() => {
                    prefix = Prefix::Yotta;
                }
                Token::Zetta if prefix.is_none() => {
                    prefix = Prefix::Zetta;
                }
                Token::Exa if prefix.is_none() => {
                    prefix = Prefix::Exa;
                }
                Token::Peta if prefix.is_none() => {
                    prefix = Prefix::Peta;
                }
                Token::Tera if prefix.is_none() => {
                    prefix = Prefix::Tera;
                }
                Token::Giga if prefix.is_none() => {
                    prefix = Prefix::Giga;
                }
                Token::MegaOrMonth if prefix.is_none() => {
                    if self.lexer.remainder().is_empty() || !prefix.is_none() {
                        return Ok(Some((prefix, Base::Second, Some(Special::Month))));
                    }

                    prefix = Prefix::Mega;
                }
                Token::Mega if prefix.is_none() => {
                    prefix = Prefix::Mega;
                }
                Token::Kilo if prefix.is_none() => {
                    prefix = Prefix::Kilo;
                }
                Token::Deci if prefix.is_none() => {
                    prefix = Prefix::Deci;
                }
                Token::Centi if prefix.is_none() => {
                    prefix = Prefix::Centi;
                }
                Token::MilliOrMeter => {
                    if self.lexer.remainder().is_empty() || !prefix.is_none() {
                        return Ok(Some((prefix, Base::Meter, None)));
                    }

                    prefix = Prefix::Milli;
                }
                Token::Milli if prefix.is_none() => {
                    prefix = Prefix::Milli;
                }
                Token::Micro if prefix.is_none() => {
                    prefix = Prefix::Micro;
                }
                Token::Nano if prefix.is_none() => {
                    prefix = Prefix::Nano;
                }
                Token::Pico if prefix.is_none() => {
                    prefix = Prefix::Pico;
                }
                Token::Femto if prefix.is_none() => {
                    prefix = Prefix::Femto;
                }
                Token::Atto if prefix.is_none() => {
                    prefix = Prefix::Atto;
                }
                Token::Zepto if prefix.is_none() => {
                    prefix = Prefix::Zepto;
                }
                Token::Yocto if prefix.is_none() => {
                    prefix = Prefix::Yocto;
                }
                _ => {
                    return Err(anyhow!(
                        "not a valid unit `{}{}{}`",
                        prefix,
                        self.lexer.slice(),
                        self.lexer.remainder()
                    ));
                }
            }
        }

        if prefix.is_none() {
            Ok(None)
        } else {
            Err(anyhow!(
                "not a valid unit `{}{}{}`",
                prefix,
                self.lexer.slice(),
                self.lexer.remainder()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnitParser;
    use crate::{unit::Special, Base, Prefix};

    #[test]
    fn test_kilo() {
        let mut p = UnitParser::new("kilominutes");
        assert_eq!(
            p.next().unwrap(),
            Some((Prefix::Kilo, Base::Second, Some(Special::Minute)))
        );
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("kminutes");
        assert_eq!(
            p.next().unwrap(),
            Some((Prefix::Kilo, Base::Second, Some(Special::Minute)))
        );
        assert!(p.next().unwrap().is_none());
    }

    #[test]
    fn test_minutes() {
        let mut p = UnitParser::new("minutes");
        assert_eq!(
            p.next().unwrap(),
            Some((Prefix::None, Base::Second, Some(Special::Minute)))
        );
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("minute");
        assert_eq!(
            p.next().unwrap(),
            Some((Prefix::None, Base::Second, Some(Special::Minute)))
        );
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("min");
        assert_eq!(
            p.next().unwrap(),
            Some((Prefix::None, Base::Second, Some(Special::Minute)))
        );
        assert!(p.next().unwrap().is_none());
    }

    #[test]
    fn test_prefixes() {
        let tests = [
            (&["Pg", "petagram"][..], Prefix::Peta),
            (&["Tg", "teragram"][..], Prefix::Tera),
            (&["Gg", "gigagram"][..], Prefix::Giga),
            (&["Mg", "megagram"][..], Prefix::Mega),
            (&["kg", "kilogram"][..], Prefix::Kilo),
            (&["mg", "milligram"][..], Prefix::Milli),
            (&["μg", "microgram"][..], Prefix::Micro),
            (&["ng", "nanogram"][..], Prefix::Nano),
        ];

        for (tests, prefix) in tests {
            for test in tests {
                let mut p = UnitParser::new(*test);

                assert_eq!(
                    p.next().unwrap(),
                    Some((prefix, Base::Gram, None)),
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
