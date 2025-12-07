use num::bigint::Sign;
use num::{Signed, Zero};
use syntree::node::Children;
use syntree::{FlavorDefault, Node, Span};

use crate::compound::{Compound, CompoundError};
use crate::error::{Error, ErrorKind};
use crate::numeric::Numeric;
use crate::query::Description;
use crate::rational::Rational;
use crate::syntax::parser::Syntax;
use crate::unit_parser::UnitParser;
use crate::{db, Query};

use ErrorKind::*;
use Syntax::*;

type Result<T, E = Error> = std::result::Result<T, E>;

mod builtin;

/// Built-in function to use.
pub(crate) type BuiltIn = fn(Span<u32>, Vec<Numeric>) -> Result<Numeric>;

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

fn add(span: Span<u32>, a: Numeric, mut b: Numeric) -> Result<Numeric> {
    match a.unit.factor(&b.unit, &mut b.value) {
        Ok(true) => Ok(Numeric::new(a.value + b.value, a.unit)),
        Ok(false) => Err(Error::new(
            span,
            IllegalOperation {
                op: "+",
                lhs: a.unit,
                rhs: b.unit,
            },
        )),
        Err(CompoundError) => Err(Error::new(
            span,
            ConversionNotPossible {
                from: a.unit,
                to: b.unit,
            },
        )),
    }
}

fn sub(span: Span<u32>, a: Numeric, mut b: Numeric) -> Result<Numeric> {
    match a.unit.factor(&b.unit, &mut b.value) {
        Ok(true) => Ok(Numeric::new(a.value - b.value, a.unit)),
        Ok(false) => Err(Error::new(
            span,
            IllegalOperation {
                op: "-",
                lhs: a.unit,
                rhs: b.unit,
            },
        )),
        Err(CompoundError) => Err(Error::new(
            span,
            ConversionNotPossible {
                from: a.unit,
                to: b.unit,
            },
        )),
    }
}

fn div(span: Span<u32>, mut a: Numeric, mut b: Numeric) -> Result<Numeric> {
    let unit = match a.unit.mul(&b.unit, -1, &mut a.value, &mut b.value) {
        Ok(unit) => unit,
        Err(CompoundError) => {
            return Err(Error::new(
                span,
                ConversionNotPossible {
                    from: a.unit,
                    to: b.unit,
                },
            ))
        }
    };

    if a.value.denom().is_zero() || b.value.numer().is_zero() {
        return Err(Error::new(span, DivideByZero));
    }

    Ok(Numeric::new(a.value / b.value, unit))
}

fn mul(span: Span<u32>, mut a: Numeric, mut b: Numeric) -> Result<Numeric> {
    let unit = match a.unit.mul(&b.unit, 1, &mut a.value, &mut b.value) {
        Ok(unit) => unit,
        Err(CompoundError) => {
            return Err(Error::new(
                span,
                ConversionNotPossible {
                    from: a.unit,
                    to: b.unit,
                },
            ))
        }
    };

    if a.value.denom().is_zero() || b.value.denom().is_zero() {
        return Err(Error::new(span, DivideByZero));
    }

    Ok(Numeric::new(a.value * b.value, unit))
}

fn pow(span: Span<u32>, base: Numeric, pow: Numeric) -> Result<Numeric> {
    if !pow.unit.is_empty() {
        return Err(Error::new(span, IllegalPowerUnit));
    }

    if !pow.value.is_integer() {
        return Err(Error::new(span, IllegalPowerNonInteger));
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
pub(crate) fn unit(
    source: &str,
    mut nodes: Children<'_, Syntax, FlavorDefault>,
    _bias: Bias,
) -> Result<Compound> {
    let mut current = 1;
    let mut compound = Compound::default();
    let mut last = None;

    while let Some(node) = nodes.next_node() {
        match node.value() {
            NUMBER => {
                let power = match str::parse::<i32>(&source[node.range()]) {
                    Ok(power) => power,
                    Err(error) => return Err(Error::new(*node.span(), BadNumber { error })),
                };

                if power != 1 {
                    return Err(Error::new(*node.span(), IllegalUnitNumber));
                }
            }
            WORD => {
                let unit = &source[node.range()];
                let mut parser = UnitParser::new(unit);

                while let Some(result) = parser.next().transpose() {
                    let (prefix, name) = match result {
                        Ok(out) => out,
                        Err(unit) => {
                            return Err(Error::new(
                                *node.span(),
                                IllegalUnit { unit: unit.into() },
                            ));
                        }
                    };

                    if let Err(expected) = compound.update(name, current, prefix) {
                        return Err(Error::new(
                            *node.span(),
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
                let (kind, span) = match (last.take(), nodes.next_node()) {
                    (Some(last), Some(node)) if node.value() == NUMBER => {
                        let span = node.span();

                        let power = match str::parse::<i32>(&source[span.range()]) {
                            Ok(power) => power,
                            Err(error) => return Err(Error::new(*span, BadNumber { error })),
                        };

                        compound.update_power(last, power * current);
                        continue;
                    }
                    (_, Some(node)) => (node.value(), *node.span()),
                    _ => (node.value(), *node.span()),
                };

                return Err(Error::new(span, Unexpected { kind }));
            }
            OP_DIV => {
                current = -current;
            }
            WHITESPACE | OP_MUL => {}
            kind => {
                return Err(Error::new(*node.span(), Unexpected { kind }));
            }
        }
    }

    Ok(compound)
}

/// Helper to delay evaluation of a syntax node so that we can modify its bias.
enum DelayedEval<'a> {
    Node(Node<'a, Syntax, FlavorDefault>),
    Numeric(Numeric),
}

impl DelayedEval<'_> {
    fn eval(self, q: &mut Query<'_>, bias: Bias) -> Result<Numeric> {
        match self {
            DelayedEval::Node(node) => eval(q, node, bias),
            DelayedEval::Numeric(numeric) => Ok(numeric),
        }
    }
}

/// Evaluate the given syntax node.
pub fn eval(
    q: &mut Query<'_>,
    node: Node<'_, Syntax, FlavorDefault>,
    bias: Bias,
) -> Result<Numeric> {
    match node.value() {
        OPERATION => {
            let mut it = node.children().skip_tokens();

            let base = match it.next() {
                Some(base) => base,
                None => return Err(Error::new(*node.span(), MissingNode)),
            };

            let mut base = DelayedEval::Node(base);

            while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
                let op = match op.value() {
                    OP_ADD => add,
                    OP_SUB => sub,
                    OP_DIV => div,
                    OP_MUL | OP_IMPLICIT_MUL => mul,
                    OP_POWER => pow,
                    OP_CAST => {
                        let rhs = unit(q.source_as_str(), rhs.children(), bias)?;

                        let mut lhs =
                            base.eval(q, bias.with_acceleration_bias(rhs.is_acceleration()))?;

                        match rhs.factor(&lhs.unit, &mut lhs.value) {
                            Ok(true) => {}
                            Ok(false) => {
                                return Err(Error::new(
                                    *node.span(),
                                    IllegalCast {
                                        from: lhs.unit,
                                        to: rhs,
                                    },
                                ));
                            }
                            Err(CompoundError) => {
                                return Err(Error::new(
                                    *node.span(),
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
                    ERROR => return Err(Error::new(*op.span(), SyntaxError)),
                    kind => {
                        return Err(Error::new(*op.span(), Unexpected { kind }));
                    }
                };

                let rhs = eval(q, rhs, bias)?;
                let b = base.eval(q, bias)?;

                base = DelayedEval::Numeric(op(*node.span(), b, rhs)?);
            }

            let numeric = base.eval(q, bias)?;
            Ok(numeric)
        }
        NUMBER => {
            let number = q.source(*node.span());
            let number = match str::parse::<Rational>(number) {
                Ok(number) => number,
                Err(error) => return Err(Error::new(*node.span(), ParseRationalError { error })),
            };
            Ok(Numeric::new(number, Compound::empty()))
        }
        WITH_UNIT => {
            let mut nodes = node.children();

            let value_node = match nodes.next() {
                Some(number) => number,
                None => return Err(Error::new(*node.span(), MissingNode)),
            };

            let unit_node = match nodes.next_node() {
                Some(unit) if unit.value() == UNIT => unit,
                Some(unit) => {
                    return Err(Error::new(
                        *unit.span(),
                        Expected {
                            expected: UNIT,
                            actual: unit.value(),
                        },
                    ))
                }
                None => return Err(Error::new(*node.span(), MissingNode)),
            };

            let value = eval(q, value_node, bias)?;
            let unit = unit(q.source_as_str(), unit_node.children(), bias)?;
            Ok(Numeric::new(value.value, unit))
        }
        SENTENCE | WORD => {
            let s = q.source(*node.span());

            let m = match q
                .db
                .lookup(s)
                .map_err(|error| Error::new(*node.span(), LookupError { error }))?
            {
                Some(m) => m,
                None => return Err(Error::new(*node.span(), Missing { query: s.into() })),
            };

            match m {
                db::Match::Constant(c) => {
                    if q.options.describe {
                        q.descriptions
                            .push(Description::Constant(s.into(), c.clone()));
                    }

                    Ok(Numeric::new(c.value.clone(), c.unit))
                }
            }
        }
        PERCENTAGE => {
            let number = match node.first() {
                Some(number) if number.value() == NUMBER => number,
                Some(node) => return Err(Error::new(*node.span(), Unexpected { kind: NUMBER })),
                _ => return Err(Error::new(*node.span(), Unexpected { kind: NUMBER })),
            };

            let number = q.source(*number.span());
            let number = match str::parse::<Rational>(number) {
                Ok(number) => number,
                Err(error) => return Err(Error::new(*node.span(), ParseRationalError { error })),
            };
            let one_hundred = Rational::new(100u32, 1u32);

            Ok(Numeric::new(number / one_hundred, Compound::empty()))
        }
        FN_CALL => {
            let mut it = node.children().skip_tokens();

            let name = match it.next() {
                Some(name) if name.value() == FN_NAME => name,
                _ => return Err(Error::new(*node.span(), Unexpected { kind: FN_NAME })),
            };

            let arguments = match it.next() {
                Some(arguments) if arguments.value() == FN_ARGUMENTS => arguments,
                _ => return Err(Error::new(*node.span(), Unexpected { kind: FN_ARGUMENTS })),
            };

            let name = q.source(*name.span());

            let mut args = Vec::new();

            for node in arguments.children().skip_tokens() {
                args.push(eval(q, node, bias)?);
            }

            if let Some(builtin) = builtin(name) {
                return builtin(*node.span(), args);
            }

            Err(Error::new(
                *node.span(),
                MissingFunction { name: name.into() },
            ))
        }
        ERROR => Err(Error::new(*node.span(), SyntaxError)),
        kind => Err(Error::new(*node.span(), Unexpected { kind })),
    }
}
