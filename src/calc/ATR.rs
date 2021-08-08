use super::{ema::EMA, DataItem, Next};

struct TR {
    prev_close: Option<f64>,
}

impl TR {
    pub fn new() -> Self {
        Self { prev_close: None }
    }
}

impl Next<DataItem> for TR {
    type Output = f64;

    fn next(&mut self, bar: DataItem) -> Self::Output {
        let max_dist = match self.prev_close {
            Some(prev_close) => {
                let dist1 = bar.high - bar.low;
                let dist2 = (bar.high - prev_close).abs();
                let dist3 = (bar.low - prev_close).abs();
                dist1.max(dist2).max(dist3)
            }
            None => bar.high - bar.low,
        };
        self.prev_close = Some(bar.close);
        max_dist
    }
}

pub struct ATR {
    tr: TR,
    ema: EMA,
}

impl ATR {
    pub fn new(period: usize) -> Self {
        Self {
            tr: TR::new(),
            ema: EMA::new(period),
        }
    }
}

impl Next<DataItem> for ATR {
    type Output = f64;
    fn next(&mut self, input: DataItem) -> Self::Output {
        self.ema.next(self.tr.next(input))
    }
}
