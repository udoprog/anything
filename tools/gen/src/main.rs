use anyhow::{anyhow, Result};
use genco::fmt;
use genco::lang::rust;
use std::fs;
use std::path::{Path, PathBuf};

/// Write a rust file to the given path.
fn write_rust_file(path: impl AsRef<Path>, tokens: rust::Tokens) -> Result<()> {
    let f = fs::File::create(path.as_ref())?;
    let mut w = fmt::IoWriter::new(f);
    let fmt = fmt::Config::from_lang::<rust::Rust>().with_indentation(fmt::Indentation::Space(4));
    let config = rust::Config::default();

    tokens.format_file(&mut w.as_formatter(&fmt), &config)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let root = match std::env::var_os("CARGO_MANIFEST_DIR") {
        Some(root) => root,
        None => return Err(anyhow!("Missing `CARGO_MANIFEST_DIR`")),
    };

    let units_path = PathBuf::from(root).join("data.toml");
    let units = fs::read(units_path)?;
    let doc: gen::units::Doc = toml::from_slice(&units)?;

    let g = Path::new("src").join("generated");

    write_rust_file(g.join("unit.rs"), gen::units::parser(&doc))?;
    write_rust_file(g.join("ids.rs"), gen::units::ids(&doc)?)?;
    Ok(())
}
