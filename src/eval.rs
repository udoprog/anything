use crate::compound::{Compound, CompoundError};
use crate::error::{Error, ErrorKind};
use crate::numeric::Numeric;
use crate::query::Description;
use crate::rational::Rational;
use crate::syntax::parser::{SyntaxKind, SyntaxNode};
use crate::unit_parser::UnitParser;
use crate::{db, Query};
use num::bigint::Sign;
use num::{Signed, Zero};
use rowan::TextRange;

use ErrorKind::*;
use SyntaxKind::*;

type Result<T, E = Error> = std::result::Result<T, E>;

mod builtin;

/// Built-in function to use.
pub(crate) type BuiltIn = fn(TextRange, Vec<Numeric>) -> Result<Numeric>;

/// Try to look up a built-in function.
pub(crate) fn builtin(name: &str) -> Option<BuiltIn> {
    let builtin: BuiltIn = match name {
        "sin" => builtin::sin,
        "cos" => builtin::cos,
        "round" => builtin::round,
        "floor" => builtin::floor,
        "ceil" => builtin::ceil,
        _ => return None,
    };

    Some(builtin)
}

/// A context.
pub struct Context {}

impl Context {
    /// Construct a new empty context.
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Bias {
    #[allow(unused)]
    acceleration_bias: bool,
}

impl Bias {
    /// Coerce the current bias to work with acceleration bias.
    fn with_acceleration_bias(self, acceleration_bias: bool) -> Self {
        Self { acceleration_bias }
    }
}

fn add(range: TextRange, a: Numeric, mut b: Numeric) -> Result<Numeric> {
    match a.unit.factor(&b.unit, &mut b.value) {
        Ok(true) => Ok(Numeric::new(a.value + b.value, a.unit)),
        Ok(false) => Err(Error::new(
            range,
            IllegalOperation {
                op: "+",
                lhs: a.unit,
                rhs: b.unit,
            },
        )),
        Err(CompoundError) => Err(Error::new(
            range,
            ConversionNotPossible {
                from: a.unit,
                to: b.unit,
            },
        )),
    }
}

fn sub(range: TextRange, a: Numeric, mut b: Numeric) -> Result<Numeric> {
    match a.unit.factor(&b.unit, &mut b.value) {
        Ok(true) => Ok(Numeric::new(a.value - b.value, a.unit)),
        Ok(false) => Err(Error::new(
            range,
            IllegalOperation {
                op: "-",
                lhs: a.unit,
                rhs: b.unit,
            },
        )),
        Err(CompoundError) => Err(Error::new(
            range,
            ConversionNotPossible {
                from: a.unit,
                to: b.unit,
            },
        )),
    }
}

fn div(range: TextRange, mut a: Numeric, mut b: Numeric) -> Result<Numeric> {
    let unit = match a.unit.mul(&b.unit, -1, &mut a.value, &mut b.value) {
        Ok(unit) => unit,
        Err(CompoundError) => {
            return Err(Error::new(
                range,
                ConversionNotPossible {
                    from: a.unit,
                    to: b.unit,
                },
            ))
        }
    };

    if a.value.denom().is_zero() || b.value.numer().is_zero() {
        return Err(Error::new(range, DivideByZero));
    }

    Ok(Numeric::new(a.value / b.value, unit))
}

fn mul(range: TextRange, mut a: Numeric, mut b: Numeric) -> Result<Numeric> {
    let unit = match a.unit.mul(&b.unit, 1, &mut a.value, &mut b.value) {
        Ok(unit) => unit,
        Err(CompoundError) => {
            return Err(Error::new(
                range,
                ConversionNotPossible {
                    from: a.unit,
                    to: b.unit,
                },
            ))
        }
    };

    if a.value.denom().is_zero() || b.value.denom().is_zero() {
        return Err(Error::new(range, DivideByZero));
    }

    Ok(Numeric::new(a.value * b.value, unit))
}

fn pow(range: TextRange, base: Numeric, pow: Numeric) -> Result<Numeric> {
    if !pow.unit.is_empty() {
        return Err(Error::new(range, IllegalPowerUnit));
    }

    if !pow.value.is_integer() {
        return Err(Error::new(range, IllegalPowerNonInteger));
    }

    if pow.value.is_zero() {
        return Ok(Numeric::new(Rational::new(1, 1), base.unit));
    }

    if base.value.is_zero() {
        return Ok(Numeric::new(base.value, base.unit));
    }

    let mut value = Rational::new(1, 1);
    let mut pow = pow.value.numer().clone();
    let sign = pow.signum();

    let b = match sign.sign() {
        Sign::Minus => base.value.recip(),
        _ => base.value,
    };

    while !pow.is_zero() {
        value *= &b;
        pow -= &sign;
    }

    Ok(Numeric::new(value, base.unit))
}

/// Parse a unit.
pub fn unit(source: &str, node: SyntaxNode, _bias: Bias) -> Result<Compound> {
    if node.kind() != UNIT {
        return Ok(Compound::default());
    }

    let mut n = 1;
    let mut compound = Compound::default();
    let mut last = None;
    let mut it = node.children_with_tokens();

    while let Some(node) = it.next() {
        match node.kind() {
            NUMBER => {
                let range = node.text_range();

                let power = match str::parse::<i32>(&source[range]) {
                    Ok(power) => power,
                    Err(error) => return Err(Error::new(range, ParseIntError { error })),
                };

                if power != 1 {
                    return Err(Error::new(range, IllegalUnitNumber));
                }
            }
            WORD => {
                let range = node.text_range();
                let unit = &source[range];
                let mut parser = UnitParser::new(unit);

                while let Some(result) = parser.next().transpose() {
                    let (prefix, name) = match result {
                        Ok(out) => out,
                        Err(unit) => {
                            return Err(Error::new(range, IllegalUnit { unit: unit.into() }));
                        }
                    };

                    if let Err(expected) = compound.update(name, n, prefix) {
                        return Err(Error::new(
                            range,
                            PrefixMismatch {
                                unit: unit.into(),
                                expected,
                                actual: prefix,
                            },
                        ));
                    }

                    last = Some(name);
                }
            }
            OP_POWER => {
                let (kind, range) = match (last.take(), it.next()) {
                    (Some(last), Some(node)) if node.kind() == NUMBER => {
                        let range = node.text_range();

                        let power = match str::parse::<i32>(&source[range]) {
                            Ok(power) => power,
                            Err(error) => return Err(Error::new(range, ParseIntError { error })),
                        };

                        compound.update_power(last, power * n);
                        continue;
                    }
                    (_, Some(node)) => (node.kind(), node.text_range()),
                    _ => (node.kind(), node.text_range()),
                };

                return Err(Error::new(range, Unexpected { kind }));
            }
            OP_MUL => {}
            OP_DIV => {
                n = n * -1;
            }
            WHITESPACE => {}
            kind => {
                return Err(Error::new(node.text_range(), Unexpected { kind }));
            }
        }
    }

    Ok(compound)
}

/// Helper to delay evaluation of a syntax node so that we can modify its bias.
enum DelayedEval {
    Node(SyntaxNode),
    Numeric(Numeric),
}

impl DelayedEval {
    fn eval(self, q: &mut Query<'_>, bias: Bias) -> Result<Numeric> {
        match self {
            DelayedEval::Node(node) => eval(q, node, bias),
            DelayedEval::Numeric(numeric) => Ok(numeric),
        }
    }
}

/// Evaluate the given syntax node.
pub fn eval(q: &mut Query<'_>, node: SyntaxNode, bias: Bias) -> Result<Numeric> {
    match node.kind() {
        OPERATION => {
            let mut it = node.children();

            let base = match it.next() {
                Some(base) => base,
                None => return Err(Error::new(node.text_range(), MissingNode)),
            };

            let mut base = DelayedEval::Node(base);

            while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
                let op = match op.kind() {
                    OP_ADD => add,
                    OP_SUB => sub,
                    OP_DIV => div,
                    OP_MUL | OP_IMPLICIT_MUL => mul,
                    OP_POWER => pow,
                    OP_CAST => {
                        let rhs = unit(q.source_as_str(), rhs, bias)?;

                        let mut lhs =
                            base.eval(q, bias.with_acceleration_bias(rhs.is_acceleration()))?;

                        match rhs.factor(&lhs.unit, &mut lhs.value) {
                            Ok(true) => {}
                            Ok(false) => {
                                return Err(Error::new(
                                    node.text_range(),
                                    IllegalCast {
                                        from: lhs.unit,
                                        to: rhs,
                                    },
                                ));
                            }
                            Err(CompoundError) => {
                                return Err(Error::new(
                                    node.text_range(),
                                    ConversionNotPossible {
                                        from: lhs.unit,
                                        to: rhs,
                                    },
                                ))
                            }
                        }

                        base = DelayedEval::Numeric(Numeric::new(lhs.value, rhs));
                        continue;
                    }
                    ERROR => return Err(Error::new(op.text_range(), SyntaxError)),
                    kind => {
                        return Err(Error::new(op.text_range(), Unexpected { kind }));
                    }
                };

                let rhs = eval(q, rhs, bias)?;
                let b = base.eval(q, bias)?;

                base = DelayedEval::Numeric(op(node.text_range(), b, rhs)?);
            }

            let numeric = base.eval(q, bias)?;
            Ok(numeric)
        }
        NUMBER => {
            let number = q.source(node.text_range());
            let number = match str::parse::<Rational>(number) {
                Ok(number) => number,
                Err(error) => {
                    return Err(Error::new(node.text_range(), ParseRationalError { error }))
                }
            };
            Ok(Numeric::new(number, Compound::empty()))
        }
        WITH_UNIT => {
            let mut it = node.children();

            let value_node = match it.next() {
                Some(number) => number,
                None => return Err(Error::new(node.text_range(), MissingNode)),
            };

            let unit_node = match it.next() {
                Some(unit) => unit,
                None => return Err(Error::new(node.text_range(), MissingNode)),
            };

            let value = eval(q, value_node, bias)?;
            let unit = unit(q.source_as_str(), unit_node, bias)?;
            Ok(Numeric::new(value.value, unit))
        }
        SENTENCE | WORD => {
            let s = q.source(node.text_range());

            let m = match q
                .db
                .lookup(s)
                .map_err(|error| Error::new(node.text_range(), LookupError { error }))?
            {
                Some(m) => m,
                None => return Err(Error::new(node.text_range(), Missing { query: s.into() })),
            };

            match m {
                db::Match::Constant(c) => {
                    if q.options.describe {
                        q.descriptions
                            .push(Description::Constant(s.into(), c.clone()));
                    }

                    Ok(Numeric::new(c.value.clone(), c.unit.clone()))
                }
            }
        }
        PERCENTAGE => {
            let number = match node.first_token() {
                Some(number) if number.kind() == NUMBER => number,
                Some(node) => {
                    return Err(Error::new(node.text_range(), Unexpected { kind: NUMBER }))
                }
                _ => return Err(Error::new(node.text_range(), Unexpected { kind: NUMBER })),
            };

            let number = q.source(number.text_range());
            let number = match str::parse::<Rational>(number) {
                Ok(number) => number,
                Err(error) => {
                    return Err(Error::new(node.text_range(), ParseRationalError { error }))
                }
            };
            let one_hundred = Rational::new(100u32, 1u32);

            Ok(Numeric::new(number / one_hundred, Compound::empty()))
        }
        FN_CALL => {
            let mut it = node.children();

            let name = match it.next() {
                Some(name) if name.kind() == FN_NAME => name,
                _ => return Err(Error::new(node.text_range(), Unexpected { kind: FN_NAME })),
            };
            let arguments = match it.next() {
                Some(arguments) if arguments.kind() == FN_ARGUMENTS => arguments,
                _ => {
                    return Err(Error::new(
                        node.text_range(),
                        Unexpected { kind: FN_ARGUMENTS },
                    ))
                }
            };
            let name = q.source(name.text_range());

            let mut args = Vec::new();

            for node in arguments.children() {
                args.push(eval(q, node, bias)?);
            }

            if let Some(builtin) = builtin(name) {
                return builtin(node.text_range(), args);
            }

            Err(Error::new(
                node.text_range(),
                MissingFunction { name: name.into() },
            ))
        }
        ERROR => Err(Error::new(node.text_range(), SyntaxError)),
        kind => Err(Error::new(node.text_range(), Unexpected { kind })),
    }
}
