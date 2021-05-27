mod data;
mod service;
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
    // let stock_service = service::Stock::new();
    // let list = stock_service.get_price("000300.XSHG").await;
    // println!("{:?}", list.unwrap());
    // create_task().await;

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let mut conn = pool.acquire().await?;

    let res = sqlx::query!("select id from stock_price")
        .fetch_all(&mut conn)
        .await?;
    println!("{:?}", res);
    Ok(())
}
