use crate::data::{self, JoinQuant, StockParams, StockPrice};
use reqwest::Error;

pub struct Stock {
    data_source: JoinQuant,
}

impl Stock {
    pub fn new() -> Self {
        Stock {
            data_source: data::JoinQuant::new(),
        }
    }
    pub async fn get_list(&self) -> Result<String, Error> {
        Ok(self.data_source.get_all_stock().await?)
    }
    pub async fn get_query_count(&self) -> Result<u64, Error> {
        Ok(self.data_source.get_query_count().await?)
    }

    pub async fn get_price(
        &self,
        code: &str,
        params: Option<StockParams>,
    ) -> Result<Vec<StockPrice>, Error> {
        Ok(self.data_source.get_price(code, params).await?)
    }
}
