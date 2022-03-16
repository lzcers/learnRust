pub mod csvdb;
pub mod sina_source;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StockDataItem {
    pub datetime: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub money: f64,
}

// 五档买卖数据, a1.0 卖价 a1.1 卖量

#[derive(Debug, Deserialize)]
pub struct Level5thData {
    a1: (f64, f64),
    a2: (f64, f64),
    a3: (f64, f64),
    a4: (f64, f64),
    a5: (f64, f64),
    b1: (f64, f64),
    b2: (f64, f64),
    b3: (f64, f64),
    b4: (f64, f64),
    b5: (f64, f64),
}

#[derive(Debug, Deserialize)]
pub struct StockPrice {
    code: String,
    name: String,
    data: StockDataItem,
    level5hData: Level5thData,
}

pub trait StockDB {
    fn insert_stock_data_items(&mut self, code: &str, items: Vec<StockDataItem>) -> Result<()>;
    fn get_stock_data_items(
        &self,
        code: &str,
        start_time: &str,
        end_time: &str,
        unit: &str,
    ) -> Result<Vec<StockDataItem>>;
}
