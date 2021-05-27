use super::utils;
use reqwest::{self, Error, Response};
use serde::Deserialize;
use serde_json::json;
use std::{cell::RefCell, env};

pub struct JoinQuant {
    api_url: String,
    mob: String,
    pwd: String,
    token: RefCell<Option<String>>,
    client: reqwest::Client,
}
#[derive(Debug, Deserialize)]
pub struct StockPrice {
    date: String,
    open: f64,
    close: f64,
    high: f64,
    low: f64,
    volume: f64,
    money: f64,
}

impl JoinQuant {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        JoinQuant {
            api_url: "https://dataapi.joinquant.com/apis".into(),
            mob: env::var("mob").unwrap(),
            pwd: env::var("pwd").unwrap(),
            token: RefCell::new(None),
            client: reqwest::Client::new(),
        }
    }

    async fn fetch(&self, mut data: serde_json::Value) -> Result<Response, Error> {
        if *self.token.borrow() == None {
            self.get_token().await?;
        }

        if let (Some(obj), Some(token)) = (&mut data.as_object_mut(), &*self.token.borrow()) {
            obj.insert("token".into(), serde_json::Value::String(token.into()));
        }
        self.client.post(&self.api_url).json(&data).send().await
    }

    pub async fn get_token(&self) -> Result<(), Error> {
        let req_data = json!({
          "method": "get_token",
          "mob": self.mob,
          "pwd": self.pwd,
        });
        let token = self
            .client
            .post(&self.api_url)
            .json(&req_data)
            .send()
            .await?
            .text()
            .await?;

        println!("joinQuant token is {:?}", &token);
        *self.token.borrow_mut() = Some(token);
        Ok(())
    }

    pub async fn get_price(&self, code: &str) -> Result<Vec<StockPrice>, Error> {
        let res = self
            .fetch(json!({
                "method": "get_price",
                "code": code,
                "count": 1,
                "unit": "1m",
            }))
            .await?
            .text()
            .await?;
        Ok(utils::parse_csv(res))
    }

    pub async fn get_query_count(&self) -> Result<u64, Error> {
        let count = self
            .fetch(json!({
              "method": "get_query_count",
            }))
            .await?
            .text()
            .await?
            .parse::<u64>();
        Ok(count.unwrap())
    }

    pub async fn get_all_stock(&self) -> Result<String, Error> {
        let stocks = self
            .fetch(json!({
              "method": "get_all_securities",
              "code": "stock",
              "date": "2021-05-24",
            }))
            .await?
            .text()
            .await?;
        Ok(stocks)
    }
}
