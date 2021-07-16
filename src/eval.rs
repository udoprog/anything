use std::collections::BTreeMap;

use crate::compound::{Compound, State};
use crate::error::{Error, ErrorKind};
use crate::numeric::Numeric;
use crate::syntax::parser::{SyntaxKind, SyntaxNode, SyntaxToken};
use crate::unit::Unit;
use crate::unit_parser::{ParsedUnit, UnitParser};
use crate::{db, numeric};
use num::BigRational;
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

fn add(range: TextRange, a: Numeric, b: Numeric) -> Result<Numeric> {
    let (mut b, b_unit) = b.split();

    if a.unit().factor(&b_unit, &mut b) {
        let (value, unit) = a.split();
        Ok(Numeric::new(value + b, unit))
    } else {
        Err(Error::new(
            range,
            IllegalOperation {
                op: "+",
                lhs: a.unit().clone(),
                rhs: b_unit,
            },
        ))
    }
}

fn sub(range: TextRange, a: Numeric, b: Numeric) -> Result<Numeric> {
    let (mut b, b_unit) = b.split();

    if a.unit().factor(&b_unit, &mut b) {
        let (value, unit) = a.split();
        Ok(Numeric::new(value - b, unit))
    } else {
        Err(Error::new(
            range,
            IllegalOperation {
                op: "-",
                lhs: a.unit().clone(),
                rhs: b_unit,
            },
        ))
    }
}

fn div(range: TextRange, a: Numeric, b: Numeric) -> Result<Numeric> {
    use num::Zero;

    let (mut a, a_unit) = a.split();
    let (mut b, b_unit) = b.split();

    let unit = a_unit.mul(&b_unit, -1, &mut a, &mut b);

    if a.denom().is_zero() || b.numer().is_zero() {
        return Err(Error::message(range, "divide by zero"));
    }

    Ok(Numeric::new(a / b, unit))
}

fn mul(range: TextRange, a: Numeric, b: Numeric) -> Result<Numeric> {
    use num::Zero;

    let (mut a, a_unit) = a.split();
    let (mut b, b_unit) = b.split();
    let unit = a_unit.mul(&b_unit, 1, &mut a, &mut b);

    if a.denom().is_zero() || b.denom().is_zero() {
        return Err(Error::message(range, "divide by zero"));
    }

    Ok(Numeric::new(a * b, unit))
}

pub fn unit(source: &str, node: SyntaxNode, bias: Bias) -> Result<Compound> {
    match node.kind() {
        UNIT => {}
        kind => return Err(Error::expected(node.text_range(), UNIT, kind)),
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
                        kind => return Err(Error::unexpected(t.text_range(), kind)),
                    };

                    let mut parser = parser.with_acceleration_bias(bias.acceleration_bias);

                    let mut last = None;
                    let range = t.text_range();

                    while let Some(parsed) =
                        parser.next().map_err(|e| Error::illegal_unit(range, e))?
                    {
                        if let Some(parsed) = std::mem::replace(&mut last, Some(parsed)) {
                            populate_unit(p, bases, parsed, range, power_factor)?;
                        }
                    }

                    if let Some(parsed) = last {
                        let caret = p.peek().map(|t| t.kind()).unwrap_or(EOF);

                        let power = if caret == CARET {
                            p.next();

                            let number = match p.next() {
                                Some(t) => match t.kind() {
                                    UNIT_NUMBER => t,
                                    ERROR => {
                                        return Err(Error::message(
                                            t.text_range(),
                                            "expected number",
                                        ))
                                    }
                                    kind => return Err(Error::unexpected(t.text_range(), kind)),
                                },
                                _ => return Err(Error::expected_only(t.text_range(), UNIT_NUMBER)),
                            };

                            let text = p.text(number.text_range());

                            match str::parse::<i32>(text) {
                                Ok(n) => n,
                                Err(e) => return Err(Error::int(number.text_range(), e)),
                            }
                        } else {
                            1
                        };

                        populate_unit(p, bases, parsed, range, power * power_factor)?;
                    }
                }
                ERROR => return Err(Error::message(t.text_range(), "expected unit component")),
                kind => return Err(Error::unexpected(t.text_range(), kind)),
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
            return Err(Error::new(
                range,
                PrefixMismatch {
                    unit: p.text(range).into(),
                    expected: entry.prefix,
                    actual: parsed.prefix,
                },
            ));
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

/// Evaluate the given syntax node.
pub fn eval(node: SyntaxNode, source: &str, db: &db::Db, bias: Bias) -> Result<Numeric> {
    match node.kind() {
        OPERATION => {
            let mut it = node.children();

            let base = match it.next() {
                Some(base) => base,
                None => return Err(Error::message(node.text_range(), "expected base node")),
            };

            let start = base.text_range();

            let mut base = DelayedEval::Node(base);

            while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
                let op = match op.first_token() {
                    Some(op) => op,
                    None => {
                        return Err(Error::new(
                            op.text_range(),
                            Internal {
                                message: "missing operator",
                            },
                        ))
                    }
                };

                let op = match op.kind() {
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

                        let (mut lhs, lhs_unit) = b.split();

                        if !rhs.factor(&lhs_unit, &mut lhs) {
                            return Err(Error::new(
                                op.text_range(),
                                IllegalCast {
                                    from: lhs_unit,
                                    to: rhs.clone(),
                                },
                            ));
                        };

                        base = DelayedEval::Numeric(Numeric::new(lhs, rhs));
                        continue;
                    }
                    ERROR => return Err(Error::message(op.text_range(), "expected operator")),
                    kind => {
                        return Err(Error::unexpected(op.text_range(), kind));
                    }
                };

                let range = rhs.text_range();
                let rhs = eval(rhs, source, db, bias)?;
                let b = base.eval(source, db, bias)?;

                let range = TextRange::new(start.start(), range.end());
                base = DelayedEval::Numeric(op(range, b, rhs)?);
            }

            let numeric = base.eval(source, db, bias)?;
            Ok(numeric)
        }
        NUMBER => {
            let number = &source[node.text_range()];
            let number = match numeric::parse_decimal_big_rational(number) {
                Ok(number) => number,
                Err(error) => return Err(Error::parse(node.text_range(), error)),
            };
            Ok(Numeric::new(number, Compound::empty()))
        }
        NUMBER_WITH_UNIT => {
            let mut it = node.children();

            let number = it.next().unwrap();
            let number = &source[number.text_range()];
            let number = match numeric::parse_decimal_big_rational(number) {
                Ok(number) => number,
                Err(error) => return Err(Error::parse(node.text_range(), error)),
            };

            let node = it.next().unwrap();
            let unit = unit(source, node, bias)?;

            Ok(Numeric::new(number, unit))
        }
        SENTENCE => {
            let s = &source[node.text_range()];

            let m = match db.lookup(s) {
                Some(m) => m,
                None => return Err(Error::new(node.text_range(), Missing { query: s.into() })),
            };

            match m {
                db::Match::Constant(c) => Ok(Numeric::new(c.value.clone(), c.unit.clone())),
            }
        }
        PERCENTAGE => {
            let number = node.first_token().expect("number of percentage");
            let number = &source[number.text_range()];
            let number = match numeric::parse_decimal_big_rational(number) {
                Ok(number) => number,
                Err(error) => return Err(Error::parse(node.text_range(), error)),
            };
            let one_hundred = BigRational::new(100u32.into(), 1u32.into());

            Ok(Numeric::new(number / one_hundred, Compound::empty()))
        }
        ERROR => Err(Error::message(node.text_range(), "expected value")),
        kind => Err(Error::unexpected(node.text_range(), kind)),
    }
}
