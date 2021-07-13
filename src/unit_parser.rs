use crate::unit::Multiple;
use crate::{Base, Prefix};
use anyhow::{anyhow, Result};

/// Helper to parse collection of units from a string.
pub struct UnitParser<'a> {
    source: &'a str,
}

impl<'a> UnitParser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }

    /// Parse the next unit and base.
    pub fn next(&mut self) -> Result<Option<(Prefix, Base, Multiple)>> {
        if self.source.is_empty() {
            return Ok(None);
        }

        let mut s = self.source;
        let mut it = s.chars();
        let mut prefix = Prefix::None;
        let mut might_be_meter = false;

        loop {
            match it.as_str() {
                "P" | "peta" if prefix.is_none() => {
                    prefix = Prefix::Peta;
                }
                "T" | "tera" if prefix.is_none() => {
                    prefix = Prefix::Tera;
                }
                "G" | "giga" if prefix.is_none() => {
                    prefix = Prefix::Giga;
                }
                "M" | "mega" if prefix.is_none() => {
                    prefix = Prefix::Mega;
                }
                "k" | "kilo" if prefix.is_none() => {
                    prefix = Prefix::Kilo;
                }
                "m" if prefix.is_none() => {
                    prefix = Prefix::Milli;
                    might_be_meter = true;
                }
                "milli" if prefix.is_none() => {
                    prefix = Prefix::Milli;
                }
                "μ" | "micro" if prefix.is_none() => {
                    prefix = Prefix::Micro;
                }
                "n" | "nano" if prefix.is_none() => {
                    prefix = Prefix::Nano;
                }
                "s" | "sec" | "second" | "seconds" => {
                    self.source = &s[it.as_str().len()..];
                    return Ok(Some((prefix, Base::Second, Multiple::None)));
                }
                "m" | "metre" | "meter" | "meters" => {
                    self.source = &s[it.as_str().len()..];
                    return Ok(Some((prefix, Base::Meter, Multiple::None)));
                }
                "g" | "gram" => {
                    self.source = &s[it.as_str().len()..];
                    return Ok(Some((prefix, Base::Gram, Multiple::None)));
                }
                "minute" | "minutes" | "min" => {
                    self.source = &s[it.as_str().len()..];
                    return Ok(Some((prefix, Base::Second, Multiple::Minute)));
                }
                "hour" | "hours" => {
                    self.source = &s[it.as_str().len()..];
                    return Ok(Some((prefix, Base::Second, Multiple::Hour)));
                }
                "day" | "days" => {
                    self.source = &s[it.as_str().len()..];
                    return Ok(Some((prefix, Base::Second, Multiple::Day)));
                }
                _ => {
                    if it.next_back().is_none() {
                        break;
                    }

                    continue;
                }
            }

            s = &s[it.as_str().len()..];

            if s.is_empty() {
                // Special case: the lone milli prefix can also be
                // interpreted as "meter".
                if might_be_meter {
                    self.source = s;
                    return Ok(Some((Prefix::None, Base::Meter, Multiple::None)));
                }
            }

            it = s.chars();
        }

        Err(anyhow!("not a valid unit: {}", self.source))
    }
}

#[cfg(test)]
mod tests {
    use super::UnitParser;
    use crate::{Base, Prefix};

    #[test]
    fn test_kilo() {
        let mut p = UnitParser::new("kilominutes");
        assert_eq!(p.next().unwrap(), Some((Prefix::Kilo, Base::Second, 60)));
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("kminutes");
        assert_eq!(p.next().unwrap(), Some((Prefix::Kilo, Base::Second, 60)));
        assert!(p.next().unwrap().is_none());
    }

    #[test]
    fn test_minutes() {
        let mut p = UnitParser::new("minutes");
        assert_eq!(p.next().unwrap(), Some((Prefix::None, Base::Second, 60)));
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("minute");
        assert_eq!(p.next().unwrap(), Some((Prefix::None, Base::Second, 60)));
        assert!(p.next().unwrap().is_none());

        let mut p = UnitParser::new("min");
        assert_eq!(p.next().unwrap(), Some((Prefix::None, Base::Second, 60)));
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
                    Some((prefix, Base::Gram, 1)),
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
