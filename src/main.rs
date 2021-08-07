mod calc;
mod data;
mod service;
use crate::calc::Next;
use calc::atr::ATR;
use data::StockParams;
use std::time::Duration;

pub async fn create_task() {
    let mut int = tokio::time::interval(Duration::from_millis(500));
    int.tick().await;

    loop {
        println!("hello world!");
        int.tick().await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stock_service = service::Stock::new();
    let list = stock_service
        .get_price(
            "000300.XSHG",
            Some(StockParams {
                count: 365,
                unit: "1d".to_string(),
                end_date: "2021-08-02".to_string(),
            }),
        )
        .await?;
    Ok(())
}
