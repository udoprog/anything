use crate::helpers;
use crate::{
    cache,
    db::{Constant, Db},
};
use anyhow::Result;
use rational::Rational;
use serde::Deserialize;

const URL: &str =
    "https://raw.githubusercontent.com/devstronomy/nasa-data-scraper/master/data/json/satellites.json";

#[derive(Debug, Deserialize)]
pub struct Satellite {
    id: u64,
    #[serde(rename = "planetId")]
    planet_id: u32,
    name: String,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    gm: Rational,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    radius: Rational,
    #[serde(default)]
    #[serde(deserialize_with = "helpers::deserialize_option_number")]
    density: Option<Rational>,
    #[serde(default)]
    #[serde(deserialize_with = "helpers::deserialize_option_number")]
    magnitude: Option<Rational>,
    #[serde(default)]
    #[serde(deserialize_with = "helpers::deserialize_option_number")]
    albedo: Option<Rational>,
}

/// Download and format planetary constants.
pub async fn download(db: &mut Db) -> Result<()> {
    let satellites = cache::get("satellites", URL).await?;
    let satellites: Vec<Satellite> = serde_json::from_slice(&satellites)?;

    let big_g = str::parse::<Rational>("6.6743015e-11")?;
    let kmc_to_mc = str::parse::<Rational>("1e9")?;
    let two = Rational::new(2u32, 1u32);

    for s in satellites {
        let name = s.name.to_lowercase();

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("mass")],
            unit: str::parse("kg")?,
            value: s.gm * &kmc_to_mc / &big_g,
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("radius")],
            unit: str::parse("km")?,
            value: s.radius.clone(),
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("diameter")],
            unit: str::parse("km")?,
            value: &s.radius * &two,
        });
    }

    Ok(())
}
