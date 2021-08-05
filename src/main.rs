mod calc;
mod data;
mod service;
use data::StockParams;
use sqlx::sqlite::SqlitePool;
use std::env;
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
    dotenv::dotenv().ok();
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
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let mut conn = pool.begin().await?;
    for item in list {
        sqlx::query!("INSERT INTO stock_price (code, date, open, close, high, low, volume, money) VALUES (?,?,?,?,?,?,?,?)", "000300.XSHG", item.date, item.open, item.close, item.high, item.low, item.volume, item.money)
            .execute(&mut conn)
            .await?;
    }
    conn.commit().await?;
    Ok(())
}
