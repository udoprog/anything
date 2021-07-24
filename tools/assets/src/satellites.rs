use crate::db::{Constant, Db};
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
    gm: Rational,
    radius: Rational,
    #[serde(default)]
    density: Option<Rational>,
    #[serde(default)]
    magnitude: Option<Rational>,
    #[serde(default)]
    albedo: Option<Rational>,
}

/// Download and format planetary constants.
pub async fn download(db: &mut Db) -> Result<()> {
    let res = reqwest::get(URL).await?;
    let satellites = res.bytes().await?;

    let satellites: Vec<Satellite> = serde_json::from_slice(&satellites)?;

    let big_g = str::parse::<Rational>("6.6743015e-11")?;
    let kmc_to_mc = str::parse::<Rational>("1e9")?;
    let two = Rational::new(2u32, 1u32);

    for s in satellites {
        let name = s.name.to_lowercase();

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("mass")],
            unit: Some(String::from("kg")),
            value: s.gm * &kmc_to_mc / &big_g,
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("radius")],
            unit: Some(String::from("km")),
            value: s.radius.clone(),
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("diameter")],
            unit: Some(String::from("km")),
            value: &s.radius * &two,
        });
    }

    Ok(())
}
