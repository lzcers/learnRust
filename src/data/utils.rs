use serde::de::DeserializeOwned;
use std::{fs::File, path::Path};

use super::StockPrice;

pub fn parse_csv_from_file_path<T>(path: &Path) -> anyhow::Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    Ok(csv::Reader::from_reader(file)
        .deserialize::<T>()
        .map(|i| i.unwrap())
        .collect())
}

pub fn parse_csv_from_str<T>(str: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    csv::Reader::from_reader(str.as_bytes())
        .deserialize::<T>()
        .map(|i| i.unwrap())
        .collect()
}

pub fn write_csv(file_path: &Path, code: &str, list: Vec<StockPrice>) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_path(file_path)?;
    wtr.write_record(&[
        "code", "date", "open", "close", "high", "low", "volume", "money",
    ])?;
    for item in list {
        wtr.serialize((
            code,
            &item.date,
            &item.open,
            &item.close,
            &&item.high,
            &item.low,
            &item.volume,
            &item.money,
        ))?;
    }
    wtr.flush()?;
    Ok(())
}
