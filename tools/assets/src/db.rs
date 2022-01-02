use anyhow::Result;
use anything::{Constant, Source};
use serde::Serialize;
use std::io;
use std::path::Path;

/// Write to the given path.
pub fn to_path<T>(path: impl AsRef<Path>, value: &T) -> Result<()>
where
    T: Serialize,
{
    let path = path.as_ref();
    println!("Writing to: {}", path.display());
    let mut f = std::fs::File::create(path)?;
    to_writer(&mut f, value)
}

/// Write to the given writer.
pub fn to_writer<W, T>(out: &mut W, value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let mut out = flate2::write::GzEncoder::new(out, Default::default());
    serde_cbor::to_writer(&mut out, value)?;
    out.finish()?;
    Ok(())
}

#[derive(Serialize)]
/// A database with constants.
pub struct Db {
    /// Constant in the database.
    pub constants: Vec<Constant>,
}

impl Db {
    /// Construct a new empty database file.
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
        }
    }
}

#[derive(Serialize)]
/// Data sources.
pub struct Sources {
    /// Sources in the database.
    pub sources: Vec<Source>,
}

impl Sources {
    /// Construct a new sources container.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }
}
