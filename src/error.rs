use crate::compound::Compound;
use crate::db::LookupError;
use crate::numeric::ParseNumericError;
use crate::syntax::parser::SyntaxKind;
use rowan::TextRange;
use std::num::ParseIntError;
use std::ops::Range;
use thiserror::Error;

/// A facts error.
#[derive(Debug, Error)]
#[error("{kind}")]
pub struct Error {
    range: TextRange,
    kind: ErrorKind,
}

impl Error {
    /// Get the text range for the current error.
    pub fn range(&self) -> Range<usize> {
        usize::from(self.range.start())..usize::from(self.range.end())
    }

    pub(crate) fn new(range: TextRange, kind: ErrorKind) -> Self {
        Self { range, kind }
    }

    pub(crate) fn message(range: TextRange, message: &'static str) -> Self {
        Self::new(range, ErrorKind::Message { message })
    }

    pub(crate) fn expected(range: TextRange, kind: SyntaxKind, actual: SyntaxKind) -> Self {
        Self::new(range, ErrorKind::Expected { kind, actual })
    }

    pub(crate) fn expected_only(range: TextRange, kind: SyntaxKind) -> Self {
        Self::new(range, ErrorKind::ExpectedOnly { kind })
    }

    pub(crate) fn unexpected(range: TextRange, kind: SyntaxKind) -> Self {
        Self::new(range, ErrorKind::Unexpected { kind })
    }

    pub(crate) fn int(range: TextRange, error: ParseIntError) -> Self {
        Self::new(range, ErrorKind::ParseIntError { error })
    }

    pub(crate) fn parse(range: TextRange, error: ParseNumericError) -> Self {
        Self::new(range, ErrorKind::ParseNumericError { error })
    }

    pub(crate) fn illegal_unit(range: TextRange, unit: &str) -> Self {
        Self::new(range, ErrorKind::IllegalUnit { unit: unit.into() })
    }
}

/// En evaluation error.
#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    #[error("failed to look up constant: {error}")]
    LookupError {
        #[source]
        error: LookupError,
    },
    #[error("illegal operation: {lhs} {op} {rhs}")]
    IllegalOperation {
        op: &'static str,
        lhs: Compound,
        rhs: Compound,
    },
    #[error("cannot cast `{from}` to `{to}`")]
    IllegalCast { from: Compound, to: Compound },
    #[error("bad decimal number: {error}")]
    ParseNumericError { error: ParseNumericError },
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
    #[error("{message}")]
    Message { message: &'static str },
}
