use std::num::ParseIntError;

use crate::compound::Compound;
use crate::parser::SyntaxKind;
use bigdecimal::ParseBigDecimalError;
use thiserror::Error;

/// A facts error.
#[derive(Debug, Error)]
#[error("{kind}")]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }

    pub(crate) fn expected(kind: SyntaxKind, actual: SyntaxKind) -> Self {
        Self::new(ErrorKind::Expected { kind, actual })
    }

    pub(crate) fn expected_only(kind: SyntaxKind) -> Self {
        Self::new(ErrorKind::ExpectedOnly { kind })
    }

    pub(crate) fn unexpected(kind: SyntaxKind) -> Self {
        Self::new(ErrorKind::Unexpected { kind })
    }

    pub(crate) fn int(error: ParseIntError) -> Self {
        Self::new(ErrorKind::ParseIntError { error })
    }

    pub(crate) fn big_decimal(error: ParseBigDecimalError) -> Self {
        Self::new(ErrorKind::ParseBigDecimalError { error })
    }

    pub(crate) fn illegal_unit(unit: &str) -> Self {
        Self::new(ErrorKind::IllegalUnit { unit: unit.into() })
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

/// En evaluation error.
#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    #[error("illegal operation: {lhs} {op} {rhs}")]
    IllegalOperation {
        op: &'static str,
        lhs: Compound,
        rhs: Compound,
    },
    #[error("cannot cast `{from}` to `{to}`")]
    IllegalCast { from: Compound, to: Compound },
    #[error("bad decimal number: {error}")]
    ParseBigDecimalError { error: ParseBigDecimalError },
    #[error("bad number: {error}")]
    ParseIntError { error: ParseIntError },
    #[error("expected syntax `{kind:?}` (internal error)")]
    ExpectedOnly { kind: SyntaxKind },
    #[error("expected syntax `{kind:?}` but got `{actual:?}` (internal error)")]
    Expected {
        kind: SyntaxKind,
        actual: SyntaxKind,
    },
    #[error("unexpected syntax `{kind:?}` (internal error)")]
    Unexpected { kind: SyntaxKind },
    #[error("nothing matching `{query}` found in database")]
    Missing { query: Box<str> },
    #[error("{message} (internal error)")]
    Internal { message: &'static str },
    #[error(
        "unit `{unit}` contains multiple mismatching prefixes; expected {expected} got {actual}"
    )]
    PrefixMismatch {
        unit: Box<str>,
        expected: i32,
        actual: i32,
    },
    #[error("unit `{unit}` is not a valid unit")]
    IllegalUnit { unit: Box<str> },
}
