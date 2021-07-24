use crate::db::{Constant, Db};
use anyhow::Result;
use rational::Rational;
use serde::Deserialize;

const URL: &str =
    "https://raw.githubusercontent.com/devstronomy/nasa-data-scraper/master/data/json/planets.json";

#[derive(Debug, Deserialize)]
pub struct Planet {
    id: u64,
    name: String,
    mass: Rational,
    diameter: Rational,
    density: Rational,
    gravity: Rational,
    #[serde(rename = "escapeVelocity")]
    escape_velocity: Rational,
    #[serde(rename = "rotationPeriod")]
    rotation_period: Rational,
    #[serde(rename = "lengthOfDay")]
    length_of_day: Rational,
    #[serde(rename = "distanceFromSun")]
    distance_from_sun: Rational,
    perihelion: Rational,
    aphelion: Rational,
    #[serde(rename = "orbitalPeriod")]
    orbital_period: Rational,
    #[serde(rename = "orbitalVelocity")]
    orbital_velocity: Rational,
    #[serde(rename = "orbitalInclination")]
    orbital_inclination: Rational,
    #[serde(rename = "orbitalEccentricity")]
    orbital_eccentricity: Rational,
    #[serde(rename = "obliquityToOrbit")]
    obliquity_to_orbit: Rational,
    #[serde(rename = "meanTemperature")]
    mean_temperature: Rational,
    #[serde(rename = "surfacePressure")]
    surface_pressure: Option<Rational>,
    #[serde(rename = "numberOfMoons")]
    number_of_moons: Rational,
    #[serde(rename = "hasRingSystem")]
    has_ring_system: bool,
    #[serde(rename = "hasGlobalMagneticField")]
    has_global_magnetic_field: bool,
}

/// Download and format planetary constants.
pub async fn download(db: &mut Db) -> Result<()> {
    let res = reqwest::get(URL).await?;
    let planets = res.bytes().await?;

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
            unit: Some(String::from("au")),
            value: p.distance_from_sun / &mkm_in_au,
        });

        db.constants.push(Constant {
            names: vec![
                name.clone(),
                String::from("orbital"),
                String::from("period"),
            ],
            unit: Some(String::from("yr")),
            value: p.orbital_period / &days_in_year,
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("mass")],
            unit: Some(String::from("kg")),
            value: p.mass * &mass_ratio,
        });

        db.constants.push(Constant {
            names: vec![
                name.clone(),
                String::from("solar"),
                String::from("day"),
                String::from("length"),
            ],
            unit: Some(String::from("dy")),
            value: p.length_of_day / &hours_in_day,
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("diameter")],
            unit: Some(String::from("km")),
            value: p.diameter.clone(),
        });

        db.constants.push(Constant {
            names: vec![name.clone(), String::from("radius")],
            unit: Some(String::from("km")),
            value: &p.diameter / &two,
        });
    }

    Ok(())
}
