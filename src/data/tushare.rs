use anyhow::{Error, Result};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

use super::{Stock, StockPrice};

#[derive(Debug, Deserialize)]
struct TushareResponse<T> {
    code: usize,
    msg: String,
    data: T,
}

struct TushareSource {
    token: String,
}

impl TushareSource {
    pub fn init(&mut self, token: &str) {
        self.token = token.to_owned();
    }
    pub async fn get(
        api_name: &str,
        fields: Vec<String>,
        mut params: HashMap<&str, String>,
    ) -> Result<Vec<StockPrice>, Error> {
        let client = Client::new();
        params.insert("api_name", api_name.to_owned());
        let resp = client
            .post("http://api.tushare.pro")
            .json(&params)
            .send()
            .await?;
        let data = resp.json::<TushareResponse<Vec<String>>>().await?;
        println!("{data:?}");
        todo!()
    }
}

impl Stock for TushareSource {
    fn get_Stock_price(
        code: &str,
        start_time: String,
        end_time: String,
        unit: String,
    ) -> Result<Vec<StockPrice>, Error> {
        todo!()
    }
}
