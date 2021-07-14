use std::collections::BTreeMap;

use crate::db;
use crate::numeric::Numeric;
use crate::parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use crate::unit::{Name, State, Unit};
use crate::unit_parser::{ParsedUnit, UnitParser};
use anyhow::{anyhow, bail, Result};
use rowan::{NodeOrToken, TextRange};
use SyntaxKind::*;

fn add(a: Numeric, b: Numeric) -> Result<Numeric> {
    if let Some(factor) = a.unit.factor(&b.unit) {
        Ok(Numeric::new(a.value + b.value / factor, a.unit))
    } else {
        bail!("cannot add the units `{} + {}`", a.unit, b.unit)
    }
}

fn sub(a: Numeric, b: Numeric) -> Result<Numeric> {
    if let Some(factor) = a.unit.factor(&b.unit) {
        Ok(Numeric::new(a.value - b.value / factor, a.unit))
    } else {
        bail!("cannot subtract the units `{} - {}`", a.unit, b.unit)
    }
}

fn div(a: Numeric, b: Numeric) -> Result<Numeric> {
    let (a_fac, b_fac, unit) = a.unit.mul(b.unit, -1);
    Ok(Numeric::new((a.value * a_fac) / (b.value * b_fac), unit))
}

fn mul(a: Numeric, b: Numeric) -> Result<Numeric> {
    let (a_fac, b_fac, unit) = a.unit.mul(b.unit, 1);
    Ok(Numeric::new((a.value * a_fac) * (b.value * b_fac), unit))
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
        bases: &mut BTreeMap<Name, State>,
        power_factor: i32,
    ) -> Result<()> {
        while let Some(t) = p.next() {
            match t.kind() {
                STAR => {
                    continue;
                }
                SLASH => {
                    break;
                }
                UNIT_WORD | UNIT_ESCAPED_WORD => {
                    let mut parser = match t.kind() {
                        UNIT_ESCAPED_WORD => {
                            let s = p.text(t.text_range());
                            let s = &s[1..(s.len() - 1)];
                            UnitParser::new(s)
                        }
                        UNIT_WORD => UnitParser::new(p.text(t.text_range())),
                        kind => bail!("unexpected unit kind `{:?}`", kind),
                    };

                    let mut last = None;
                    let range = t.text_range();

                    while let Some(parsed) = parser.next()? {
                        if let Some(parsed) = std::mem::replace(&mut last, Some(parsed)) {
                            populate_unit(p, bases, parsed, range, power_factor)?;
                        }
                    }

                    if let Some(parsed) = last {
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

                        populate_unit(p, bases, parsed, range, power * power_factor)?;
                    }
                }
                kind => bail!("unexpected token: {:?}", kind),
            }
        }

        Ok(())
    }

    fn populate_unit(
        p: &Tokens<impl Iterator<Item = NodeOrToken<SyntaxNode, SyntaxToken>>>,
        bases: &mut BTreeMap<Name, State>,
        parsed: ParsedUnit,
        range: TextRange,
        power_factor: i32,
    ) -> Result<()> {
        let entry = bases.entry(parsed.name).or_insert_with(|| State {
            prefix: parsed.prefix,
            power: 0,
        });

        entry.power += power_factor;

        if entry.prefix != parsed.prefix {
            bail!(
                "unit `{}` must have the same kind in each unit spec",
                p.text(range)
            );
        }

        Ok(())
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

                let op = match op {
                    PLUS => add,
                    DASH => sub,
                    SLASH => div,
                    STAR => mul,
                    AS | TO => {
                        let rhs = unit(source, rhs)?;

                        let factor = match base.unit.factor(&rhs) {
                            Some(factor) => factor,
                            None => bail!("{} cannot be cast to {}", base, rhs),
                        };

                        base.value *= factor;
                        base.unit = rhs;
                        continue;
                    }
                    kind => {
                        bail!("unsuported op: {:?}", kind);
                    }
                };

                let rhs = eval(source, rhs, db)?;
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
