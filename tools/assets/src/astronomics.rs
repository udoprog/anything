use crate::db::Db;
use crate::helpers;
use crate::{cache, db::Sources};
use anyhow::Result;
use facts::{Constant, Source};
use rational::Rational;
use serde::Deserialize;

const SOURCE: u64 = 0x2cf06db8998f8888;

pub async fn download(db: &mut Db, sources: &mut Sources) -> Result<()> {
    sources.sources.push(Source {
        id: SOURCE,
        description: "NASA factsheet about planets and satellites".into(),
        url: Some("https://github.com/devstronomy/nasa-data-scraper".into()),
    });

    download_planets(db).await?;
    download_satellites(db).await?;
    Ok(())
}

const PLANETS_URL: &str =
    "https://raw.githubusercontent.com/devstronomy/nasa-data-scraper/master/data/json/planets.json";

#[derive(Debug, Deserialize)]
pub struct Planet {
    id: u64,
    name: String,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    mass: Rational,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    diameter: Rational,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    density: Rational,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    gravity: Rational,
    #[serde(rename = "escapeVelocity")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    escape_velocity: Rational,
    #[serde(rename = "rotationPeriod")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    rotation_period: Rational,
    #[serde(rename = "lengthOfDay")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    length_of_day: Rational,
    #[serde(rename = "distanceFromSun")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    distance_from_sun: Rational,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    perihelion: Rational,
    #[serde(deserialize_with = "helpers::deserialize_number")]
    aphelion: Rational,
    #[serde(rename = "orbitalPeriod")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    orbital_period: Rational,
    #[serde(rename = "orbitalVelocity")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    orbital_velocity: Rational,
    #[serde(rename = "orbitalInclination")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    orbital_inclination: Rational,
    #[serde(rename = "orbitalEccentricity")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    orbital_eccentricity: Rational,
    #[serde(rename = "obliquityToOrbit")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    obliquity_to_orbit: Rational,
    #[serde(rename = "meanTemperature")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    mean_temperature: Rational,
    #[serde(rename = "surfacePressure")]
    #[serde(deserialize_with = "helpers::deserialize_option_number")]
    surface_pressure: Option<Rational>,
    #[serde(rename = "numberOfMoons")]
    #[serde(deserialize_with = "helpers::deserialize_number")]
    number_of_moons: Rational,
    #[serde(rename = "hasRingSystem")]
    has_ring_system: bool,
    #[serde(rename = "hasGlobalMagneticField")]
    has_global_magnetic_field: bool,
}

/// Download and format planetary constants.
async fn download_planets(db: &mut Db) -> Result<()> {
    println!("Downloading planets");
    let planets = cache::get("planets", PLANETS_URL).await?;

    println!("Deserializing planets");
    let planets: Vec<Planet> = serde_json::from_slice(&planets)?;

    let mkm_in_au = Rational::new(1496u32, 10u32);
    let days_in_year = Rational::new(3652u32, 10u32);
    let mass_ratio = Rational::new(1000000000000000000000000u128, 1u32);
    let hours_in_day = Rational::new(24u32, 1u32);
    let two = Rational::new(2u32, 1u32);

    for p in planets {
        let search_name = Box::<str>::from(p.name.to_lowercase());

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "orbit".into(), "distance".into()],
            description: format!("Orbital distance of {}", p.name).into(),
            unit: str::parse("au")?,
            value: p.distance_from_sun / &mkm_in_au,
        });

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "orbital".into(), "period".into()],
            description: format!("Orbital period of {}", p.name).into(),
            unit: str::parse("yr")?,
            value: p.orbital_period / &days_in_year,
        });

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "mass".into()],
            description: format!("Mass of {}", p.name).into(),
            unit: str::parse("kg")?,
            value: p.mass * &mass_ratio,
        });

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![
                search_name.clone(),
                "solar".into(),
                "day".into(),
                "length".into(),
            ],
            description: format!("Length of a solar day on {}", p.name).into(),
            unit: str::parse("dy")?,
            value: p.length_of_day / &hours_in_day,
        });

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "diameter".into()],
            description: format!("Diameter of {}", p.name).into(),
            unit: str::parse("km")?,
            value: p.diameter.clone(),
        });

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "radius".into()],
            description: format!("Radius of {}", p.name).into(),
            unit: str::parse("km")?,
            value: &p.diameter / &two,
        });
    }

    Ok(())
}

const SATELLITES_URL: &str =
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
async fn download_satellites(db: &mut Db) -> Result<()> {
    let satellites = cache::get("satellites", SATELLITES_URL).await?;
    let satellites: Vec<Satellite> = serde_json::from_slice(&satellites)?;

    let big_g = str::parse::<Rational>("6.6743015e-11")?;
    let kmc_to_mc = str::parse::<Rational>("1e9")?;
    let two = Rational::new(2u32, 1u32);

    for s in satellites {
        let search_name = Box::<str>::from(s.name.to_lowercase());

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "mass".into()],
            description: format!("Mass of the satellite {}", s.name).into(),
            unit: str::parse("kg")?,
            value: s.gm * &kmc_to_mc / &big_g,
        });

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "radius".into()],
            description: format!("Radius of the satellite {}", s.name).into(),
            unit: str::parse("km")?,
            value: s.radius.clone(),
        });

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: vec![search_name.clone(), "diameter".into()],
            description: format!("Diameter of the satellite {}", s.name).into(),
            unit: str::parse("km")?,
            value: &s.radius * &two,
        });
    }

    Ok(())
}
