use anyhow::Result;
use rational::Rational;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

#[derive(Serialize)]
pub struct Constant {
    /// Names of the constant.
    pub names: Vec<String>,
    /// The unit associated with the constant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// The value of a constant.
    pub value: Rational,
}

#[derive(Serialize)]
/// A database with constants.
pub struct Db {
    /// Constant in the database.
    pub constants: Vec<Constant>,
}

impl Db {
    /// Write to the given path.
    pub fn write_to(self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        let string = toml::to_string_pretty(&self)?;

        println!("Writing database to: {}", path.display());
        let mut f = std::fs::File::create(path)?;
        f.write_all(string.as_bytes())?;
        Ok(())
    }
}
