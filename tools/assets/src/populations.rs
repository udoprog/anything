use crate::analyzer::Analyzer;
use crate::cache;
use crate::db::{Db, Sources};
use anyhow::{anyhow, Result};
use anything::{Constant, Rational, Source};
use calamine::{DataType, Reader, Xlsx};
use std::io::Cursor;

const SOURCE: u64 = 0x23afb9ae5087db93;

const URL: &str = "https://population.un.org/wpp/Download/Files/1_Indicators%20(Standard)/EXCEL_FILES/1_Population/WPP2019_POP_F01_1_TOTAL_POPULATION_BOTH_SEXES.xlsx";

/// Download and format planetary constants.
pub async fn download(analyzer: &Analyzer, db: &mut Db, sources: &mut Sources) -> Result<()> {
    sources.sources.push(Source {
        id: SOURCE,
        description: "Population data from the UN".into(),
        url: Some("https://population.un.org/wpp/Download/Standard/Population/".into()),
    });

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
            last_year = Some((year, n));
        }
    }

    let (last_year_number, last_year) = match last_year {
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
            Some(DataType::Float(population)) => {
                Rational::from_f64(*population).unwrap_or_else(|| Rational::new(1, 1)) * &thousand
            }
            Some(DataType::Int(population)) => Rational::new(*population, 1u32) * &thousand,
            _ => {
                continue;
            }
        };

        let mut names = Vec::new();
        names.push("population".into());
        names.extend(analyzer.filter(region));

        db.constants.push(Constant {
            source: Some(SOURCE),
            tokens: names,
            description: format!("Population of {region} in {last_year_number}").into(),
            unit: Default::default(),
            value: population,
        });
    }

    Ok(())
}
