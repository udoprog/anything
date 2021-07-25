use anyhow::Result;
use bytes::Bytes;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use tokio::fs;

const CACHE: &str = "cache";

/// Get a cached value.
pub async fn get(name: &str, url: &str) -> Result<Bytes> {
    let cache = Path::new(CACHE);

    if !cache.is_dir() {
        fs::create_dir_all(cache).await?;
    }

    let cache_path = cache.join(name).with_extension("gz");

    if cache_path.is_file() {
        let bytes = fs::read(cache_path).await?;
        let mut reader = GzDecoder::new(Cursor::new(bytes));
        let mut out = Vec::new();
        reader.read_to_end(&mut out)?;
        return Ok(out.into());
    }

    let res = reqwest::get(url).await?;
    let bytes = res.bytes().await?;

    let options = Default::default();
    let mut writer = GzEncoder::new(Vec::new(), options);
    writer.write_all(&bytes[..])?;
    let encoded = writer.finish()?;

    fs::write(cache_path, encoded).await?;
    Ok(bytes)
}
