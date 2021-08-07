pub struct DataItem {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub money: f64,
}

pub trait Next<T> {
    type Output;
    fn next(&mut self, input: T) -> Self::Output;
}

mod ATR;
pub mod atr;
pub mod ema;
