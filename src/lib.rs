//! Facts is a flexible unit-aware arbitrary precision calculator.

#![deny(missing_docs)]
#![allow(clippy::neg_multiply)]

mod compound;
mod db;
mod error;
mod eval;
mod format;
mod numeric;
mod powers;
mod prefix;
mod query;
#[doc(hidden)]
pub mod syntax;
mod unit;
mod unit_parser;
pub mod units;

pub use self::compound::Compound;
pub use self::db::Db;
pub use self::error::Error;
pub use self::numeric::Numeric;
pub use self::powers::Powers;
pub use self::query::{query, Query};
pub use self::unit::Unit;
