use std::collections::HashMap;
use std::io::Cursor;

use anyhow::{anyhow, ensure, Context, Result};
use anything::{Constant, Rational, Source};
use calamine::{Data, DataType, Reader, Xlsx};

use crate::analyzer::Analyzer;
use crate::cache;
use crate::db::{Db, Sources};

const SOURCE: u64 = 0x23afb9ae5087db93;

const URL: &str = "https://population.un.org/wpp/assets/Excel%20Files/1_Indicator%20(Standard)/EXCEL_FILES/2_Population/WPP2024_POP_F01_1_POPULATION_SINGLE_AGE_BOTH_SEXES.xlsx";

/// Download and format planetary constants.
pub async fn download(analyzer: &Analyzer, db: &mut Db, sources: &mut Sources) -> Result<()> {
    sources.sources.push(Source {
        id: SOURCE,
        description: "Population data from the UN".into(),
        url: Some("https://population.un.org/wpp/".into()),
    });

    let bytes = cache::get("populations", URL).await?;
    let bytes = Cursor::new(&bytes[..]);

    let mut doc = Xlsx::new(bytes)?;

    let sheet = doc.worksheet_range("Estimates")?;

    let mut it = sheet.rows();

    let Some(row) = it.by_ref().skip(12).next() else {
        return Err(anyhow!("couldn't find header row"));
    };

    let region_index = row
        .iter()
        .position(
            |cell| matches!(cell, Data::String(s) if s == "Region, subregion, country or area *"),
        )
        .context("couldn't find region column")?;
    let year_index = row
        .iter()
        .position(|cell| matches!(cell, Data::String(s) if s == "Year"))
        .context("couldn't find year column")?;
    let first_index = row
        .iter()
        .position(|cell| matches!(cell, Data::Float(v) if *v == 0.0))
        .context("couldn't find first index")?;
    let last_index = row
        .iter()
        .position(|cell| matches!(cell, Data::String(s) if s == "100+"))
        .context("couldn't find last index")?;

    ensure!(
        first_index < last_index,
        "first index must be less than last index"
    );

    let thousand = Rational::new(1000u32, 1u32);

    let mut last_years = HashMap::<String, (u32, Rational)>::new();

    for row in it {
        let region = row
            .get(region_index)
            .context("missing_region")?
            .as_string()
            .context("region is not a string")?;

        let Some(year) = row
            .get(year_index)
            .and_then(|cell| u32::try_from(cell.as_i64()?).ok())
        else {
            continue;
        };

        let mut population = Rational::new(0u32, 1u32);

        for index in first_index..=last_index {
            let Some(Data::Float(value)) = row.get(index) else {
                continue;
            };

            population += Rational::new(*value as u64, 1u32) * &thousand;
        }

        let mut names = Vec::new();
        names.push("population".into());
        names.extend(analyzer.filter(&region));
        names.push(year.to_string().into());

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: names,
            description: format!("Population of {region} in {year}").into(),
            unit: Default::default(),
            value: population.clone(),
        });

        let last_year = last_years
            .entry(region.to_string())
            .or_insert_with(|| (0, Rational::new(0u32, 1u32)));

        if year > last_year.0 {
            *last_year = (year, population);
        }
    }

    for (region, (year, population)) in last_years {
        let mut names = Vec::new();

        names.push("population".into());
        names.extend(analyzer.filter(&region));

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: names,
            description: format!("Population of {region} in {year}").into(),
            unit: Default::default(),
            value: population,
        });
    }

    Ok(())
}
