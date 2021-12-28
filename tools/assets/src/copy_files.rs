use crate::db::Db;
use anyhow::{anyhow, Result};
use facts::{Compound, Constant};
use rational::Rational;
use serde::de;
use serde::Deserialize;
use std::{borrow::Cow, path::PathBuf};
use tokio::fs;

#[derive(Debug, Deserialize)]
struct InnerConstant {
    tokens: Vec<Box<str>>,
    description: Box<str>,
    #[serde(deserialize_with = "deserialize_value")]
    value: Rational,
    #[serde(default, deserialize_with = "deserialize_unit")]
    unit: Option<Compound>,
}

#[derive(Debug, Deserialize)]
struct InnerDb {
    #[serde(default)]
    constants: Vec<InnerConstant>,
}

/// Copy files.
pub async fn copy_files(db: &mut Db) -> Result<()> {
    let dir = match std::env::var_os("CARGO_MANIFEST_DIR") {
        Some(dir) => dir,
        None => return Err(anyhow!("missing `CARGO_MANIFEST_DIR`")),
    };

    let root = PathBuf::from(dir);
    let db_path = root.join("db");

    for e in std::fs::read_dir(&db_path)? {
        let e = e?;
        let content = fs::read(e.path()).await?;
        let inner_db: InnerDb = toml::from_slice(&content)?;

        for c in inner_db.constants {
            db.constants.push(Constant {
                source: None,
                tokens: c.tokens,
                description: c.description,
                value: c.value,
                unit: c.unit.unwrap_or_default(),
            });
        }
    }

    Ok(())
}

fn deserialize_value<'de, D>(deserializer: D) -> Result<Rational, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = Cow::<'de, str>::deserialize(deserializer)?;
    str::parse::<Rational>(s.as_ref()).map_err(<D::Error as de::Error>::custom)
}

fn deserialize_unit<'de, D>(deserializer: D) -> Result<Option<Compound>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = match Option::<Cow<'de, str>>::deserialize(deserializer)? {
        Some(s) => s,
        None => return Ok(None),
    };

    Ok(Some(
        str::parse::<Compound>(s.as_ref()).map_err(<D::Error as de::Error>::custom)?,
    ))
}
