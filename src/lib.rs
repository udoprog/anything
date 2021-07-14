//! Facts is a flexible unit-aware arbitrary precision calculator.

mod compound_unit;
pub mod db;
mod error;
mod eval;
mod grammar;
mod lexer;
mod numeric;
#[doc(hidden)]
pub mod parser;
mod prefix;
mod query;
mod span;
mod unit;
mod unit_parser;

pub use self::compound_unit::CompoundUnit;
pub use self::error::Error;
pub use self::numeric::Numeric;
pub use self::query::{query, Query};
pub use self::unit::Unit;
