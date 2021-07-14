//! Facts is a flexible unit-aware arbitrary precision calculator.

pub mod db;
mod error;
mod eval;
mod grammar;
mod lexer;
mod numeric;
#[doc(hidden)]
pub mod parser;
mod query;
mod span;
mod unit;
mod unit_parser;

pub use self::error::Error;
pub use self::numeric::Numeric;
pub use self::query::{query, Query};
pub use self::unit::{CompoundUnit, Unit};
