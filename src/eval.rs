use std::collections::BTreeMap;

use crate::db;
use crate::parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use crate::unit::BaseData;
use crate::{Base, Prefix};
use crate::{Numeric, Unit};
use anyhow::{anyhow, bail, Result};
use rowan::{NodeOrToken, TextRange};
use SyntaxKind::*;

fn add(mut a: Numeric, b: Numeric) -> Result<Numeric> {
    if let Some((a_factor, b_factor)) = a.unit.factor(&b.unit) {
        Ok(Numeric::new(
            a.value * a_factor + b.value * b_factor,
            a.unit,
        ))
    } else {
        bail!("cannot add the units `{} + {}`", a.unit, b.unit)
    }
}

fn sub(mut a: Numeric, b: Numeric) -> Result<Numeric> {
    if let Some((a_factor, b_factor)) = a.unit.factor(&b.unit) {
        Ok(Numeric::new(
            a.value * a_factor - b.value * b_factor,
            a.unit,
        ))
    } else {
        bail!("cannot subtract the units `{} - {}`", a.unit, b.unit)
    }
}

fn div(mut a: Numeric, b: Numeric) -> Result<Numeric> {
    let (a_factor, b_factor) = a.unit.mul(b.unit, -1);
    Ok(Numeric::new(
        (a.value * a_factor) / (b.value * b_factor),
        a.unit,
    ))
}

fn mul(mut a: Numeric, b: Numeric) -> Result<Numeric> {
    let (a_factor, b_factor) = a.unit.mul(b.unit, 1);
    Ok(Numeric::new(
        (a.value * a_factor) * (b.value * b_factor),
        a.unit,
    ))
}

pub fn unit(source: &str, node: SyntaxNode) -> Result<Unit> {
    match node.kind() {
        UNIT => {}
        kind => bail!("unsupported unit node: {:?}", kind),
    }

    let mut it = Tokens {
        source,
        iter: node.children_with_tokens().peekable(),
        next: None,
    };

    let mut bases = BTreeMap::new();

    inner(&mut it, &mut bases, 1)?;
    inner(&mut it, &mut bases, -1)?;

    return Ok(Unit::new(bases));

    struct Tokens<'a, T> {
        source: &'a str,
        iter: T,
        next: Option<SyntaxToken>,
    }

    impl<T> Tokens<'_, T>
    where
        T: Iterator<Item = NodeOrToken<SyntaxNode, SyntaxToken>>,
    {
        fn text(&self, range: TextRange) -> &str {
            &self.source[range]
        }

        fn next(&mut self) -> Option<SyntaxToken> {
            if let Some(token) = self.next.take() {
                return Some(token);
            }

            loop {
                return match self.iter.next()? {
                    NodeOrToken::Token(token) => Some(token),
                    _ => continue,
                };
            }
        }

        fn peek(&mut self) -> Option<SyntaxToken> {
            loop {
                if let Some(token) = &self.next {
                    return Some(token.clone());
                }

                self.next = Some(match self.iter.next()? {
                    NodeOrToken::Token(token) => token,
                    _ => continue,
                });
            }
        }
    }

    fn inner(
        p: &mut Tokens<impl Iterator<Item = NodeOrToken<SyntaxNode, SyntaxToken>>>,
        bases: &mut BTreeMap<Base, BaseData>,
        power_factor: i32,
    ) -> Result<()> {
        while let Some(t) = p.next() {
            let mut prefix = Prefix::None;

            match t.kind() {
                STAR => {
                    continue;
                }
                SLASH => {
                    break;
                }
                UNIT_LETTER | UNIT_ESCAPED_WORD => {
                    let unit = parse_unit(p, t, &mut prefix)?;

                    let (base, multiple) = match unit.kind() {
                        UNIT_ESCAPED_WORD => {
                            let s = p.text(unit.text_range());
                            let s = &s[1..(s.len() - 1)];

                            match s {
                                "minute" | "minutes" => (Base::Second, 60),
                                other => {
                                    bail!("not a supported unit `{}`", other)
                                }
                            }
                        }
                        UNIT_LETTER => {
                            let unit = match p.text(unit.text_range()).chars().next() {
                                Some(c) => c,
                                None => bail!("missing unit character"),
                            };

                            let base = match Base::parse(unit) {
                                Some(unit) => unit,
                                None => bail!("unsupported base unit `{}`", unit),
                            };

                            (base, 1)
                        }
                        kind => bail!("unexpected unit kind `{:?}`", kind),
                    };

                    let caret = p.peek().map(|t| t.kind()).unwrap_or(EOF);

                    let power = if caret == CARET {
                        p.next();

                        let number = match p.next() {
                            Some(t) if t.kind() == UNIT_NUMBER => t,
                            _ => bail!("expected unit number"),
                        };

                        let text = p.text(number.text_range());
                        str::parse::<i32>(text)?
                    } else {
                        1
                    };

                    let entry = bases.entry(base).or_insert_with(|| BaseData {
                        prefix,
                        power: 0,
                        multiple,
                    });

                    entry.power += power * power_factor;

                    if entry.prefix != prefix || entry.multiple != multiple {
                        bail!("unit `{}` must have the same kind in each unit spec", unit);
                    }
                }
                kind => bail!("unexpected token: {:?}", kind),
            }
        }

        Ok(())
    }

    fn parse_unit(
        p: &mut Tokens<impl Iterator<Item = NodeOrToken<SyntaxNode, SyntaxToken>>>,
        t: SyntaxToken,
        prefix: &mut Prefix,
    ) -> Result<SyntaxToken> {
        let unit = match t.kind() {
            UNIT_LETTER => match p.text(t.text_range()).chars().next() {
                Some(c) => c,
                None => bail!("missing unit character"),
            },
            _ => return Ok(t),
        };

        let new = match Prefix::parse(unit) {
            Some(new) => new,
            None => return Ok(t),
        };

        match p.peek().map(|t| t.kind()) {
            Some(UNIT_LETTER | UNIT_ESCAPED_WORD) => {
                *prefix = new;
                p.next().ok_or_else(|| anyhow!("missing required token"))
            }
            _ => {
                // we did not see a prefix, so "unparse" it.
                Ok(t)
            }
        }
    }
}

/// Evaluate the syntax node.
pub fn eval(source: &str, node: SyntaxNode, db: &db::Db) -> Result<Numeric> {
    match node.kind() {
        OPERATION => {
            let mut it = node.children();
            let mut base = eval(source, it.next().unwrap(), db)?;

            while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
                let op = op
                    .first_token()
                    .map(|t| t.kind())
                    .ok_or_else(|| anyhow!("missing op"))?;
                let rhs = eval(source, rhs, db)?;

                let op = match op {
                    PLUS => add,
                    DASH => sub,
                    SLASH => div,
                    STAR => mul,
                    kind => {
                        bail!("unsuported op: {:?}", kind);
                    }
                };

                base = op(base, rhs)?;
            }

            Ok(base)
        }
        NUMBER => {
            let s = &source[node.text_range()];
            let int = str::parse::<bigdecimal::BigDecimal>(s)?;
            Ok(Numeric::new(int, Unit::empty()))
        }
        NUMBER_WITH_UNIT => {
            let mut it = node.children();

            let number = it.next().unwrap();
            let number = &source[number.text_range()];
            let number = str::parse::<bigdecimal::BigDecimal>(number)?;

            let node = it.next().unwrap();
            let unit = unit(source, node)?;

            Ok(Numeric::new(number, unit))
        }
        SENTENCE => {
            let s = &source[node.text_range()];

            let m = match db.lookup(s) {
                Some(m) => m,
                None => bail!("found nothing matching `{}`", s),
            };

            match m {
                db::Match::Constant(c) => Ok(Numeric::new(c.value.clone(), c.unit.clone())),
            }
        }
        PERCENTAGE => {
            let number = node.first_token().expect("number of percentage");
            let number = &source[number.text_range()];
            let number = str::parse::<bigdecimal::BigDecimal>(number)?;
            let one_hundred = bigdecimal::BigDecimal::from(100);
            Ok(Numeric::new(number / one_hundred, Unit::empty()))
        }
        kind => {
            bail!("unsupported expression: {:?}", kind)
        }
    }
}
