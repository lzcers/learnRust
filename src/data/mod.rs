pub mod join_quant;
mod tushare;
pub mod utils;
use anyhow::{Error, Result};
pub use join_quant::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StockPrice {
    pub date: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub money: f64,
}

pub trait Stock {
    fn get_Stock_price(
        code: &str,
        start_time: String,
        end_time: String,
        unit: String,
    ) -> Result<Vec<StockPrice>, Error>;
}
