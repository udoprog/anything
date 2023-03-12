use anyhow::{anyhow, Result};
use rust_embed::{EmbeddedFile, RustEmbed};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(RustEmbed)]
#[folder = "db"]
struct Asset;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub database_hash: Option<String>,
}

/// The configuration of the project.
pub struct Config {
    pub this_version: &'static str,
    pub meta: Meta,
    pub index_path: PathBuf,
    pub meta_path: PathBuf,
}

impl Config {
    /// Write the given metadata.
    pub fn write_meta(&self) -> Result<()> {
        let f = fs::File::create(&self.meta_path)?;
        serde_json::to_writer(f, &self.meta)?;
        Ok(())
    }

    /// Iterate over available asset names.
    pub fn assets(&self) -> impl Iterator<Item = Cow<'static, str>> {
        Asset::iter()
    }

    /// Get content for the given asset.
    pub fn get_asset(&self, name: &str) -> Option<EmbeddedFile> {
        Asset::get(name)
    }

    /// Hash all available assets so we can determine if we need to rebuild or not.
    pub fn hash_assets(&self) -> String {
        use std::hash::Hasher;
        use twox_hash::xxh3::HasherExt;

        const SEED: u64 = 0x9a7f42b11904b426;

        let mut hash = twox_hash::xxh3::Hash128::with_seed(SEED);

        hash.write_usize(self.this_version.as_bytes().len());
        hash.write(self.this_version.as_bytes());

        for name in Asset::iter() {
            if let Some(content) = Asset::get(name.as_ref()) {
                let name = name.as_ref().as_bytes();

                hash.write_usize(name.len());
                hash.write(name);
                hash.write_usize(content.data.len());
                hash.write(&content.metadata.sha256_hash()[..]);
            }
        }

        format!("{:x}", hash.finish_ext())
    }
}

pub fn open() -> Result<Config> {
    let dirs = directories::ProjectDirs::from("se.tedro", "tedro", "facts")
        .ok_or_else(|| anyhow!("project directories not supported"))?;
    let data = dirs.data_dir();
    let index_path = data.join("index");
    let meta_path = data.join("meta.json");

    let meta: Option<Meta> = if meta_path.is_file() {
        try_read(&meta_path)
    } else {
        None
    };

    let meta = meta.unwrap_or(Meta {
        version: None,
        database_hash: None,
    });

    Ok(Config {
        this_version: VERSION,
        meta,
        index_path,
        meta_path,
    })
}

fn try_read(path: &Path) -> Option<Meta> {
    let f = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("failed to open meta file: {e}");
            return None;
        }
    };

    match serde_json::from_reader(f) {
        Ok(meta) => Some(meta),
        Err(e) => {
            log::error!("failed to open meta: {e}");
            None
        }
    }
}
