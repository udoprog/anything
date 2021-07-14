use std::collections::BTreeMap;

use crate::compound::{Compound, State};
use crate::db;
use crate::error::{Error, ErrorKind};
use crate::numeric::Numeric;
use crate::parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use crate::unit::Unit;
use crate::unit_parser::{ParsedUnit, UnitParser};
use bigdecimal::BigDecimal;
use rowan::{NodeOrToken, TextRange};

use ErrorKind::*;
use SyntaxKind::*;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Default, Debug, Clone, Copy)]
pub struct Bias {
    acceleration_bias: bool,
}

impl Bias {
    /// Coerce the current bias to work with acceleration bias.
    fn with_acceleration_bias(self, acceleration_bias: bool) -> Self {
        Self {
            acceleration_bias,
            ..self
        }
    }
}

fn add(a: Numeric, b: Numeric) -> Result<Numeric> {
    if let Some(factor) = a.unit().factor(b.unit()) {
        let (value, unit) = a.split();
        Ok(Numeric::new(value + b.into_value() / factor, unit))
    } else {
        Err(Error::new(IllegalOperation {
            op: "+",
            lhs: a.unit().clone(),
            rhs: b.unit().clone(),
        }))
    }
}

fn sub(a: Numeric, b: Numeric) -> Result<Numeric> {
    if let Some(factor) = a.unit().factor(b.unit()) {
        let (value, unit) = a.split();
        Ok(Numeric::new(value - b.into_value() / factor, unit))
    } else {
        Err(Error::new(IllegalOperation {
            op: "-",
            lhs: a.unit().clone(),
            rhs: b.unit().clone(),
        }))
    }
}

fn div(a: Numeric, b: Numeric) -> Result<Numeric> {
    let (a_fac, b_fac, unit) = a.unit().mul(b.unit(), -1);
    Ok(Numeric::new(
        (a.into_value() * a_fac) / (b.into_value() * b_fac),
        unit,
    ))
}

fn mul(a: Numeric, b: Numeric) -> Result<Numeric> {
    let (a_fac, b_fac, unit) = a.unit().mul(b.unit(), 1);
    Ok(Numeric::new(
        (a.into_value() * a_fac) * (b.into_value() * b_fac),
        unit,
    ))
}

pub fn unit(source: &str, node: SyntaxNode, bias: Bias) -> Result<Compound> {
    match node.kind() {
        UNIT => {}
        kind => return Err(Error::expected(UNIT, kind)),
    }

    let mut it = Tokens {
        source,
        iter: node.children_with_tokens().peekable(),
        next: None,
    };

    let mut bases = BTreeMap::new();

    inner(&mut it, &mut bases, 1, bias)?;
    inner(&mut it, &mut bases, -1, bias)?;

    return Ok(Compound::new(bases));

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
        bases: &mut BTreeMap<Unit, State>,
        power_factor: i32,
        bias: Bias,
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
                    let parser = match t.kind() {
                        UNIT_ESCAPED_WORD => {
                            let s = p.text(t.text_range());
                            let s = &s[1..(s.len() - 1)];
                            UnitParser::new(s)
                        }
                        UNIT_WORD => UnitParser::new(p.text(t.text_range())),
                        kind => return Err(Error::unexpected(kind)),
                    };

                    let mut parser = parser.with_acceleration_bias(bias.acceleration_bias);

                    let mut last = None;
                    let range = t.text_range();

                    while let Some(parsed) = parser.next().map_err(Error::illegal_unit)? {
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
                                _ => return Err(Error::expected_only(UNIT_NUMBER)),
                            };

                            let text = p.text(number.text_range());
                            str::parse::<i32>(text).map_err(Error::int)?
                        } else {
                            1
                        };

                        populate_unit(p, bases, parsed, range, power * power_factor)?;
                    }
                }
                kind => return Err(Error::unexpected(kind)),
            }
        }

        Ok(())
    }

    fn populate_unit(
        p: &Tokens<impl Iterator<Item = NodeOrToken<SyntaxNode, SyntaxToken>>>,
        bases: &mut BTreeMap<Unit, State>,
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
            return Err(Error::new(PrefixMismatch {
                unit: p.text(range).into(),
                expected: entry.prefix,
                actual: parsed.prefix,
            }));
        }

        Ok(())
    }
}

/// Helper to delay evaluation of a syntax node so that we can modify its bias.
enum DelayedEval {
    Node(SyntaxNode),
    Numeric(Numeric),
}

impl DelayedEval {
    fn eval(self, source: &str, db: &db::Db, bias: Bias) -> Result<Numeric> {
        match self {
            DelayedEval::Node(node) => eval(node, source, db, bias),
            DelayedEval::Numeric(numeric) => Ok(numeric),
        }
    }
}

/// Evaluate the syntax node.
pub fn eval(node: SyntaxNode, source: &str, db: &db::Db, bias: Bias) -> Result<Numeric> {
    match node.kind() {
        OPERATION => {
            let mut it = node.children();
            let mut base = DelayedEval::Node(it.next().unwrap());

            while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
                let op = op.first_token().map(|t| t.kind()).ok_or_else(|| Internal {
                    message: "missing operator",
                })?;

                let op = match op {
                    PLUS => add,
                    DASH => sub,
                    SLASH => div,
                    STAR => mul,
                    AS | TO => {
                        let rhs = unit(source, rhs, bias)?;
                        let b = base.eval(
                            source,
                            db,
                            bias.with_acceleration_bias(rhs.is_acceleration()),
                        )?;

                        let factor = match b.unit().factor(&rhs) {
                            Some(factor) => factor,
                            None => {
                                return Err(Error::new(IllegalCast {
                                    from: b.unit().clone(),
                                    to: rhs.clone(),
                                }))
                            }
                        };

                        let value = b.into_value();
                        base = DelayedEval::Numeric(Numeric::new(value * factor, rhs));
                        continue;
                    }
                    kind => {
                        return Err(Error::unexpected(kind));
                    }
                };

                let rhs = eval(rhs, source, db, bias)?;
                let b = base.eval(source, db, bias)?;
                base = DelayedEval::Numeric(op(b, rhs)?);
            }

            Ok(base.eval(source, db, bias)?)
        }
        NUMBER => {
            let s = &source[node.text_range()];
            let int = str::parse::<BigDecimal>(s).map_err(Error::big_decimal)?;
            Ok(Numeric::new(int, Compound::empty()))
        }
        NUMBER_WITH_UNIT => {
            let mut it = node.children();

            let number = it.next().unwrap();
            let number = &source[number.text_range()];
            let number = str::parse::<BigDecimal>(number).map_err(Error::big_decimal)?;

            let node = it.next().unwrap();
            let unit = unit(source, node, bias)?;

            Ok(Numeric::new(number, unit))
        }
        SENTENCE => {
            let s = &source[node.text_range()];

            let m = match db.lookup(s) {
                Some(m) => m,
                None => return Err(Error::new(Missing { query: s.into() })),
            };

            match m {
                db::Match::Constant(c) => Ok(Numeric::new(c.value.clone(), c.unit.clone())),
            }
        }
        PERCENTAGE => {
            let number = node.first_token().expect("number of percentage");
            let number = &source[number.text_range()];
            let number = str::parse::<BigDecimal>(number).map_err(Error::big_decimal)?;
            let one_hundred = BigDecimal::from(100);
            Ok(Numeric::new(number / one_hundred, Compound::empty()))
        }
        kind => Err(Error::unexpected(kind)),
    }
}
