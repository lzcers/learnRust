mod calc;
mod data;
mod service;
use calc::{atr::ATR, ema::EMA, DataItem, Next};
use data::utils::parse_csv_from_file_path;
use serde::Deserialize;
use std::{path::Path, time::Duration};

pub async fn create_task() {
    let mut int = tokio::time::interval(Duration::from_millis(500));
    int.tick().await;

    loop {
        println!("hello world!");
        int.tick().await;
    }
}

#[derive(Debug, Deserialize)]
struct CSVItem {
    code: String,
    date: String,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
    money: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let list = parse_csv_from_file_path::<CSVItem>(Path::new("./300.csv"))?;
    let mut atr = ATR::new(14);
    let mut ema = EMA::new(12);
    for item in &list[..] {
        println!(
            "{} atr14: {} ema12: {}",
            item.date,
            atr.next(DataItem {
                open: item.open,
                close: item.close,
                high: item.high,
                low: item.low,
                volume: item.volume,
                money: item.money
            }),
            ema.next(item.close),
        )
    }
    Ok(())
}
