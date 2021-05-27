mod data;
mod service;
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
async fn main() {
    // let stock_service = service::Stock::new();
    // let list = stock_service.get_price("000300.XSHG").await;
    // println!("{:?}", list.unwrap());
    create_task().await;
}
