use crate::compound::Compound;
use crate::error::{Error, ErrorKind};
use crate::numeric::Numeric;
use crate::query::Description;
use crate::syntax::parser::{SyntaxKind, SyntaxNode};
use crate::unit::Unit;
use crate::unit_parser::UnitParser;
use crate::{db, Query};
use num::bigint::Sign;
use num::{Signed, Zero};
use rational::Rational;
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
    let (mut a, a_unit) = a.split();
    let (mut b, b_unit) = b.split();

    let unit = a_unit.mul(&b_unit, -1, &mut a, &mut b);

    if a.denom().is_zero() || b.numer().is_zero() {
        return Err(Error::new(range, DivideByZero));
    }

    Ok(Numeric::new(a / b, unit))
}

fn mul(range: TextRange, a: Numeric, b: Numeric) -> Result<Numeric> {
    let (mut a, a_unit) = a.split();
    let (mut b, b_unit) = b.split();
    let unit = a_unit.mul(&b_unit, 1, &mut a, &mut b);

    if a.denom().is_zero() || b.denom().is_zero() {
        return Err(Error::new(range, DivideByZero));
    }

    Ok(Numeric::new(a * b, unit))
}

fn pow(range: TextRange, a: Numeric, b: Numeric) -> Result<Numeric> {
    let (base, unit) = a.split();
    let (pow, pow_unit) = b.split();

    if !pow_unit.is_empty() {
        return Err(Error::new(range, IllegalPowerUnit));
    }

    if !pow.is_integer() {
        return Err(Error::new(range, IllegalPowerNonInteger));
    }

    if pow.is_zero() {
        return Ok(Numeric::new(Rational::new(1, 1), unit));
    }

    if base.is_zero() {
        return Ok(Numeric::new(base, unit));
    }

    let mut value = Rational::new(1, 1);
    let mut pow = pow.numer().clone();
    let sign = pow.signum();

    let base = match sign.sign() {
        Sign::Minus => base.recip(),
        _ => base,
    };

    while !pow.is_zero() {
        value *= &base;
        pow -= &sign;
    }

    Ok(Numeric::new(value, unit))
}

pub fn unit(source: &str, node: SyntaxNode, bias: Bias) -> Result<Compound> {
    let mut compound = Compound::default();
    inner_unit(source, node, bias, &mut compound, 1)?;
    Ok(compound)
}

fn inner_unit(
    source: &str,
    node: SyntaxNode,
    bias: Bias,
    compound: &mut Compound,
    n: i32,
) -> Result<Option<Unit>> {
    let last = match node.kind() {
        NUMBER => {
            let range = node.text_range();

            let number = match str::parse::<i32>(&source[range]) {
                Ok(number) => number,
                Err(error) => return Err(Error::new(range, ParseIntError { error })),
            };

            if number != 1 {
                return Err(Error::new(range, IllegalUnitNumber));
            }

            None
        }
        OPERATION => {
            let range = node.text_range();
            let mut it = node.children();

            let base = match it.next() {
                Some(base) => base,
                None => return Err(Error::new(range, MissingNode)),
            };
            let last = inner_unit(source, base, bias, compound, n)?;

            while let (Some(op), Some(arg)) = (it.next(), it.next()) {
                match (last, op.kind()) {
                    (Some(last), OP_POWER) => {
                        let power = match arg.kind() {
                            NUMBER => match str::parse::<i32>(&source[arg.text_range()]) {
                                Ok(power) => power,
                                Err(error) => {
                                    return Err(Error::new(
                                        arg.text_range(),
                                        ParseIntError { error },
                                    ))
                                }
                            },
                            _ => {
                                return Err(Error::new(
                                    arg.text_range(),
                                    Unexpected { kind: NUMBER },
                                ))
                            }
                        };

                        compound.update_power(last, power * n);
                    }
                    (_, OP_MUL | OP_IMPLICIT_MUL) => {
                        inner_unit(source, arg, bias, compound, 1)?;
                    }
                    (_, OP_DIV) => {
                        inner_unit(source, arg, bias, compound, -1)?;
                    }
                    (_, kind) => {
                        return Err(Error::new(op.text_range(), Unexpected { kind }));
                    }
                }
            }

            None
        }
        WORD => {
            let range = node.text_range();
            let unit = &source[range];
            let mut parser = UnitParser::new(unit);

            let mut last = None;

            while let Some((prefix, name)) = parser
                .next()
                .map_err(|unit| Error::new(range, IllegalUnit { unit: unit.into() }))?
            {
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

            last
        }
        kind => {
            return Err(Error::new(node.text_range(), Unexpected { kind }));
        }
    };

    Ok(last)
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

            let start = base.text_range();

            let mut base = DelayedEval::Node(base);

            while let (Some(op), Some(rhs)) = (it.next(), it.next()) {
                let op = match op.kind() {
                    OP_ADD => add,
                    OP_SUB => sub,
                    OP_DIV => div,
                    OP_MUL | OP_IMPLICIT_MUL => mul,
                    OP_POWER => pow,
                    OP_CAST => {
                        let rhs = unit(q.source, rhs, bias)?;

                        let b = base.eval(q, bias.with_acceleration_bias(rhs.is_acceleration()))?;

                        let (mut lhs, lhs_unit) = b.split();

                        if !rhs.factor(&lhs_unit, &mut lhs) {
                            return Err(Error::new(
                                op.text_range(),
                                IllegalCast {
                                    from: lhs_unit,
                                    to: rhs,
                                },
                            ));
                        };

                        base = DelayedEval::Numeric(Numeric::new(lhs, rhs));
                        continue;
                    }
                    ERROR => return Err(Error::new(op.text_range(), SyntaxError)),
                    kind => {
                        return Err(Error::new(op.text_range(), Unexpected { kind }));
                    }
                };

                let range = rhs.text_range();
                let rhs = eval(q, rhs, bias)?;
                let b = base.eval(q, bias)?;

                let range = TextRange::new(start.start(), range.end());
                base = DelayedEval::Numeric(op(range, b, rhs)?);
            }

            let numeric = base.eval(q, bias)?;
            Ok(numeric)
        }
        NUMBER => {
            let number = &q.source[node.text_range()];
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
            let unit = unit(q.source, unit_node, bias)?;
            Ok(Numeric::new(value.into_value(), unit))
        }
        SENTENCE | WORD => {
            let s = &q.source[node.text_range()];

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
            let number = node.first_token().expect("number of percentage");
            let number = &q.source[number.text_range()];
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
            let name = &q.source[name.text_range()];

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
