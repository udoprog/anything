use crate::analyzer::Analyzer;
use crate::cache;
use crate::db::{Constant, Db};
use anyhow::{anyhow, Result};
use calamine::{DataType, Reader, Xlsx};
use rational::Rational;
use std::io::Cursor;

const URL: &str = "https://population.un.org/wpp/Download/Files/1_Indicators%20(Standard)/EXCEL_FILES/1_Population/WPP2019_POP_F01_1_TOTAL_POPULATION_BOTH_SEXES.xlsx";

/// Download and format planetary constants.
pub async fn download(analyzer: &Analyzer, db: &mut Db) -> Result<()> {
    let bytes = cache::get("populations", URL).await?;
    let bytes = Cursor::new(&bytes[..]);

    let mut doc = Xlsx::new(bytes)?;

    let sheet = match doc.worksheet_range("ESTIMATES").transpose()? {
        Some(sheet) => sheet,
        None => return Err(anyhow!("didn't find sheet `ESTIMATES`")),
    };

    let mut it = sheet.rows().skip(12);

    let first = match it.next() {
        Some(first) => first,
        None => return Err(anyhow!("couldn't find first row")),
    };

    let mut years = Vec::new();
    let mut last_year = None;

    for (n, col) in first.iter().enumerate().skip(7) {
        if let DataType::String(year) = col {
            let year = str::parse::<u32>(year)?;
            years.push((year, n));
            last_year = Some(n);
        }
    }

    let last_year = match last_year {
        Some(last_year) => last_year,
        None => return Err(anyhow!("missing last year")),
    };

    let thousand = Rational::new(1000u32, 1u32);

    for row in it {
        let region = if let [DataType::Float(..), DataType::String(..), DataType::String(region), ..] =
            row
        {
            region
        } else {
            continue;
        };

        let population = match row.get(last_year) {
            Some(DataType::Float(population)) => Rational::from_f64(*population) * &thousand,
            Some(DataType::Int(population)) => Rational::new(*population, 1u32) * &thousand,
            _ => {
                continue;
            }
        };

        let mut names = Vec::new();
        names.push(String::from("population"));
        names.extend(analyzer.filter(region));

        db.constants.push(Constant {
            names,
            unit: None,
            value: population,
        });
    }

    Ok(())
}
