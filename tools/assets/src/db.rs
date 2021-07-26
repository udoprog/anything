use anyhow::Result;
use facts::Constant;
use serde::Serialize;
use std::io;
use std::path::Path;

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

    /// Write to the given path.
    pub fn to_path(self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        println!("Writing database to: {}", path.display());
        let mut f = std::fs::File::create(path)?;
        self.to_writer(&mut f)
    }

    /// Write to the given writer.
    pub fn to_writer<W>(self, out: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        let mut out = flate2::write::GzEncoder::new(out, Default::default());
        serde_cbor::to_writer(&mut out, &self)?;
        out.finish()?;
        Ok(())
    }
}
