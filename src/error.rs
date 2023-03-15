use std::num::ParseIntError;
use std::ops::Range;

use syntree::Span;
use thiserror::Error;

use crate::compound::Compound;
use crate::db::LookupError;
use crate::rational::ParseRationalError;
use crate::syntax::parser::Syntax;

/// A facts error.
#[derive(Debug, Error)]
#[error("{kind}")]
pub struct Error {
    span: Span<u32>,
    kind: ErrorKind,
}

impl Error {
    /// Get the text range for the current error.
    pub fn range(&self) -> Range<usize> {
        self.span.range()
    }

    pub(crate) fn new(span: Span<u32>, kind: ErrorKind) -> Self {
        Self { span, kind }
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
    #[error("conversion from {from} to {to} is not possible")]
    ConversionNotPossible { from: Compound, to: Compound },
    #[error("cannot cast `{from}` to `{to}`")]
    IllegalCast { from: Compound, to: Compound },
    #[error("bad decimal number: {error}")]
    ParseRationalError { error: ParseRationalError },
    #[error("bad number: {error}")]
    BadNumber { error: ParseIntError },
    #[error("unexpected syntax `{kind:?}` (internal error)")]
    Unexpected { kind: Syntax },
    #[error("unexpected syntax `{actual:?}`, expected {expected:?} (internal error)")]
    Expected { actual: Syntax, expected: Syntax },
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
    #[error("non-finite number in calculation")]
    NonFinite,
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
    #[error("error when building tree")]
    TreeError {
        #[source]
        #[from]
        error: syntree::Error,
    },
}
