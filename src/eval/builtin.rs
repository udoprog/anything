use num::{One, ToPrimitive};
use rational::Rational;
use rowan::TextRange;

use crate::error::ErrorKind::*;
use crate::{Error, Numeric};

type Result<T, E = Error> = std::result::Result<T, E>;

/// Expect end decode a single argument.
fn one(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let actual = arguments.len();
    let mut it = arguments.into_iter();

    match (it.next(), it.next()) {
        (Some(first), None) if actual == 1 => Ok(first),
        _ => Err(Error::new(
            range,
            ArgumentMismatch {
                expected: 1,
                actual,
            },
        )),
    }
}

/// Calculate the sine of a number.
pub(crate) fn sin(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let (value, unit) = one(range, arguments)?.split();

    let value = match value.to_f64() {
        Some(value) => value.sin(),
        None => return Err(Error::new(range, BadArgument { argument: 0 })),
    };

    let value = Rational::from_f64(value).ok_or_else(|| Error::new(range, NonFinite))?;
    Ok(Numeric::new(value, unit))
}

/// Calculate the cosine of a number.
pub(crate) fn cos(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let (value, unit) = one(range, arguments)?.split();

    let value = match value.to_f64() {
        Some(value) => value.cos(),
        None => return Err(Error::new(range, BadArgument { argument: 0 })),
    };

    let value = Rational::from_f64(value).ok_or_else(|| Error::new(range, NonFinite))?;
    Ok(Numeric::new(value, unit))
}

/// Round a number with an optional power.
pub(crate) fn round(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let actual = arguments.len();
    let mut it = arguments.into_iter();

    let (first, second) = match (it.next(), it.next()) {
        (Some(first), None) if actual == 1 => (first, 0),
        (Some(first), Some(second)) if actual == 2 => (
            first,
            match second.split().0.to_i32() {
                Some(second) => second,
                None => return Err(Error::new(range, BadArgument { argument: 0 })),
            },
        ),
        _ => {
            return Err(Error::new(
                range,
                ArgumentMismatch {
                    expected: if actual == 0 { 1 } else { 2 },
                    actual,
                },
            ));
        }
    };

    let (mut first, unit) = first.split();

    let first = if second >= 0 && first.denom().is_one() {
        first
    } else {
        if second == 0 {
            first.round()
        } else {
            let ten = Rational::new(10u32, 1u32).pow(second);
            first *= &ten;
            let mut first = first.round();
            first /= &ten;
            first
        }
    };

    debug_assert!(first.denom().is_one());
    Ok(Numeric::new(first, unit))
}

/// Floor a number.
pub(crate) fn floor(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let (first, unit) = one(range, arguments)?.split();
    let first = first.floor();
    debug_assert!(first.denom().is_one());
    Ok(Numeric::new(first, unit))
}

/// Ceil a number.
pub(crate) fn ceil(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let (first, unit) = one(range, arguments)?.split();
    let first = first.ceil();
    debug_assert!(first.denom().is_one());
    Ok(Numeric::new(first, unit))
}
