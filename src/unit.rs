use anyhow::anyhow;
use std::collections::{btree_map, BTreeMap};
use std::fmt;
use std::iter::Peekable;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    num: BTreeMap<char, u32>,
    den: BTreeMap<char, u32>,
}

impl Unit {
    pub fn empty() -> Self {
        Self {
            num: BTreeMap::new(),
            den: BTreeMap::new(),
        }
    }

    /// Test if the unit is the special empty unit.
    pub fn is_empty(&self) -> bool {
        self.num.iter().all(|e| *e.1 == 0) && self.den.iter().all(|e| *e.1 == 0)
    }

    /// Create a new unit.
    pub fn new<N, D>(num: N, den: D) -> Self
    where
        N: IntoIterator<Item = (char, u32)>,
        D: IntoIterator<Item = (char, u32)>,
    {
        Self {
            num: num.into_iter().collect(),
            den: den.into_iter().collect(),
        }
    }

    pub fn div(mut self, other: Self) -> Option<Self> {
        for (unit, pow) in other.num {
            match self.num.entry(unit) {
                btree_map::Entry::Vacant(..) => return None,
                btree_map::Entry::Occupied(mut o) => {
                    let n = (*o.get()).checked_sub(pow)?;

                    if n == 0 {
                        let _ = o.remove_entry();
                    } else {
                        *o.get_mut() = n;
                    }
                }
            }
        }

        for (unit, pow) in other.den {
            match self.den.entry(unit) {
                btree_map::Entry::Vacant(..) => return None,
                btree_map::Entry::Occupied(mut o) => {
                    let n = (*o.get()).checked_sub(pow)?;

                    if n == 0 {
                        let _ = o.remove_entry();
                    } else {
                        *o.get_mut() = n;
                    }
                }
            }
        }

        Some(self)
    }

    pub fn mul(mut self, other: Self) -> Self {
        for (unit, pow) in other.num {
            *self.num.entry(unit).or_default() += pow;
        }

        for (unit, pow) in other.den {
            *self.den.entry(unit).or_default() += pow;
        }

        self
    }
}

impl std::str::FromStr for Unit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "1" {
            return Ok(Unit::default());
        }

        let mut it = s.char_indices().peekable();

        let mut num = BTreeMap::new();
        let mut den = BTreeMap::new();

        inner(s, &mut num, &mut it)?;
        inner(s, &mut den, &mut it)?;

        return Ok(Self { num, den });

        fn inner(
            s: &str,
            map: &mut BTreeMap<char, u32>,
            it: &mut Peekable<impl Iterator<Item = (usize, char)>>,
        ) -> anyhow::Result<()> {
            let mut unit = None;

            while let Some((_, c)) = it.next() {
                match (unit, c) {
                    (_, '/') => break,
                    (Some(add), '^') => {
                        let (start, _) = it.next().ok_or_else(|| anyhow!("missing digit"))?;

                        while let Some((_, '0'..='9')) = it.peek().copied() {
                            it.next();
                        }

                        let end = it.peek().map(|n| n.0).unwrap_or(s.len());
                        let pow = str::parse::<u32>(&s[start..end])?;
                        *map.entry(add).or_default() += pow;
                        unit = None;
                    }
                    (None, u) => {
                        unit = Some(u);
                    }
                    (Some(add), u) => {
                        *map.entry(add).or_default() += 1;
                        unit = Some(u);
                    }
                }
            }

            if let Some(unit) = unit {
                *map.entry(unit).or_default() += 1;
            }

            Ok(())
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let without_den = self.den.iter().all(|c| *c.1 == 0);

        if self.num.iter().all(|c| *c.1 == 0) {
            if without_den {
                return Ok(());
            }

            write!(f, "1")?;
        } else {
            for (unit, pow) in self.num.iter() {
                fmt_base(*unit, *pow, f)?;
            }
        }

        if without_den {
            return Ok(());
        }

        write!(f, "/")?;

        for (unit, pow) in self.den.iter() {
            fmt_base(*unit, *pow, f)?;
        }

        return Ok(());

        fn fmt_base(unit: char, pow: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", unit)?;

            if pow != 1 {
                if pow < 10 {
                    pow_into_char(pow).fmt(f)?;
                } else {
                    let mut chars = Vec::new();
                    let mut pow = pow;

                    while pow != 0 {
                        chars.push(pow_into_char(pow % 10));
                        pow /= 10;
                    }

                    for c in chars.into_iter().rev() {
                        c.fmt(f)?;
                    }
                }
            }

            Ok(())
        }
    }
}

fn pow_into_char(pow: u32) -> char {
    match pow {
        0 => '⁰',
        1 => '¹',
        2 => '²',
        3 => '³',
        4 => '⁴',
        5 => '⁵',
        6 => '⁶',
        7 => '⁷',
        8 => '⁸',
        _ => '⁹',
    }
}

#[cfg(test)]
mod tests {
    use super::{Base, Unit};

    #[test]
    fn display_unit() {
        let unit = Unit::div(Unit::meter(), Unit::pow(Base::Second, 2));
        assert_eq!("m/s²", unit.to_string());
    }

    #[test]
    fn test_normalize_div_div() {
        let unit = Unit::div(Unit::meter(), Unit::div(Unit::second(), Unit::second())).normalize();
        assert_eq!("m/s²", unit.to_string());

        let unit = Unit::div(
            Unit::meter(),
            Unit::div(Unit::second(), Unit::div(Unit::second(), Unit::second())),
        )
        .normalize();
        assert_eq!("m/s³", unit.to_string());

        let unit = Unit::div(
            Unit::meter(),
            Unit::div(Unit::div(Unit::second(), Unit::second()), Unit::second()),
        )
        .normalize();
        assert_eq!("m/s³", unit.to_string());
    }

    #[test]
    fn test_large_pow() {
        let unit = Unit::div(Unit::meter(), Unit::pow(Base::Second, 103));
        assert_eq!("m/s¹⁰³", unit.to_string());
    }

    #[test]
    fn test_normalize() {
        let unit = Unit::div(Unit::meter(), Unit::pow(Base::Second, 0)).normalize();
        assert_eq!(Unit::meter(), unit);
    }
}
