use crate::rational::Rational;
use num::{One, ToPrimitive};
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
    let first = one(range, arguments)?;

    let value = match first.value.to_f64() {
        Some(value) => value.sin(),
        None => return Err(Error::new(range, BadArgument { argument: 0 })),
    };

    let value = Rational::from_f64(value).ok_or_else(|| Error::new(range, NonFinite))?;
    Ok(Numeric::new(value, first.unit))
}

/// Calculate the cosine of a number.
pub(crate) fn cos(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let first = one(range, arguments)?;

    let value = match first.value.to_f64() {
        Some(value) => value.cos(),
        None => return Err(Error::new(range, BadArgument { argument: 0 })),
    };

    let value = Rational::from_f64(value).ok_or_else(|| Error::new(range, NonFinite))?;
    Ok(Numeric::new(value, first.unit))
}

/// Round a number with an optional power.
pub(crate) fn round(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let actual = arguments.len();
    let mut it = arguments.into_iter();

    let (mut first, second) = match (it.next(), it.next()) {
        (Some(first), None) if actual == 1 => (first, 0),
        (Some(first), Some(second)) if actual == 2 => (
            first,
            match second.value.to_i32() {
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

    let value = if second >= 0 && first.value.denom().is_one() {
        first.value
    } else {
        if second == 0 {
            first.value.round()
        } else {
            let ten = Rational::new(10u32, 1u32).pow(second);
            first.value *= &ten;
            let mut value = first.value.round();
            value /= &ten;
            value
        }
    };

    debug_assert!(value.denom().is_one());
    Ok(Numeric::new(value, first.unit))
}

/// Floor a number.
pub(crate) fn floor(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let first = one(range, arguments)?;
    let value = first.value.floor();
    debug_assert!(value.denom().is_one());
    Ok(Numeric::new(value, first.unit))
}

/// Ceil a number.
pub(crate) fn ceil(range: TextRange, arguments: Vec<Numeric>) -> Result<Numeric> {
    let first = one(range, arguments)?;
    let value = first.value.ceil();
    debug_assert!(value.denom().is_one());
    Ok(Numeric::new(value, first.unit))
}
