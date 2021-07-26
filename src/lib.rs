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
#[doc(hidden)]
pub mod syntax;
mod unit;
mod unit_parser;
pub mod units;

pub use self::compound::Compound;
pub use self::db::{Constant, Db};
pub use self::error::Error;
pub use self::numeric::Numeric;
pub use self::powers::Powers;
pub use self::query::{query, Description, Options, Query};
pub use self::unit::Unit;
