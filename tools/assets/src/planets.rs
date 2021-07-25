use crate::cache;
use crate::db::{Constant, Db};
use crate::helpers;
use anyhow::Result;
use rational::Rational;
use serde::Deserialize;

const URL: &str =
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
pub async fn download(db: &mut Db) -> Result<()> {
    println!("Downloading planets");
    let planets = cache::get("planets", URL).await?;

    println!("Deserializing planets");
    let planets: Vec<Planet> = serde_json::from_slice(&planets)?;

    let mkm_in_au = Rational::new(1496u32, 10u32);
    let days_in_year = Rational::new(3652u32, 10u32);
    let mass_ratio = Rational::new(1000000000000000000000000u128, 1u32);
    let hours_in_day = Rational::new(24u32, 1u32);
    let two = Rational::new(2u32, 1u32);

    for p in planets {
        let name = p.name.to_lowercase();

        db.constants.push(Constant {
            names: vec![
                name.clone(),
                String::from("orbit"),
                String::from("distance"),
            ],
            unit: str::parse("au")?,
            value: p.distance_from_sun / &mkm_in_au,
        });

        db.constants.push(Constant {
            names: vec![
                name.clone(),
                String::from("orbital"),
                String::from("period"),
            ],
            unit: str::parse("yr")?,
            value: p.orbital_period / &days_in_year,
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("mass")],
            unit: str::parse("kg")?,
            value: p.mass * &mass_ratio,
        });

        db.constants.push(Constant {
            names: vec![
                name.clone(),
                String::from("solar"),
                String::from("day"),
                String::from("length"),
            ],
            unit: str::parse("dy")?,
            value: p.length_of_day / &hours_in_day,
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("diameter")],
            unit: str::parse("km")?,
            value: p.diameter.clone(),
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("radius")],
            unit: str::parse("km")?,
            value: &p.diameter / &two,
        });
    }

    Ok(())
}
