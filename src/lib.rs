//! Facts is a flexible unit-aware arbitrary precision calculator.

#![deny(missing_docs)]
#![allow(clippy::neg_multiply)]

mod compound;
mod config;
mod db;
mod error;
mod eval;
mod generated;
mod numeric;
mod powers;
mod prefix;
mod query;
pub mod rational;
#[doc(hidden)]
pub mod syntax;
mod unit;
mod unit_parser;
pub mod units;

pub use self::compound::Compound;
pub use self::db::{Constant, Db, Source};
pub use self::error::Error;
pub use self::numeric::Numeric;
pub use self::powers::Powers;
pub use self::query::{parse, query, Description, Options, Query};
pub use self::rational::Rational;
pub use self::unit::Unit;
