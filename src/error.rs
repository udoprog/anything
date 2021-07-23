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
}

/// En evaluation error.
#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    #[error("syntax error")]
    SyntaxError,
    #[error("divide by zero")]
    DivideByZero,
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
    #[error("unexpected syntax `{kind:?}` (internal error)")]
    Unexpected { kind: SyntaxKind },
    #[error("nothing matching `{query}` found in database")]
    Missing { query: Box<str> },
    #[error("unit `{unit}` is not a valid unit")]
    IllegalUnit { unit: Box<str> },
    #[error("missing function `{name}`")]
    MissingFunction { name: Box<str> },
    #[error("bad number of arguments, got {actual} but expected {expected}")]
    ArgumentMismatch { expected: usize, actual: usize },
    #[error("bad argument {argument}")]
    BadArgument { argument: usize },
    #[error("missing expected node")]
    MissingNode,
    #[error("mismatching prefix for unit `{unit}`; expected {expected} but got {actual}")]
    PrefixMismatch {
        unit: Box<str>,
        expected: i32,
        actual: i32,
    },
    #[error("unit numbers must be `1`")]
    IllegalUnitNumber,
    #[error("the power must not have a unit")]
    IllegalPowerUnit,
    #[error("the power of a number must be an integer")]
    IllegalPowerNonInteger,
}
