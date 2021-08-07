use super::{ema::EMA, DataItem, Next};

struct TR {
    pre_close: Option<f64>,
}

impl TR {
    pub fn new() -> Self {
        Self { pre_close: None }
    }
}

impl Next<DataItem> for TR {
    type Output = f64;

    fn next(&mut self, bar: DataItem) -> Self::Output {
        let max_dist = match self.pre_close {
            Some(prev_close) => {
                let dist1 = bar.high - bar.low;
                let dist2 = (bar.high - prev_close).abs();
                let dist3 = (bar.low - prev_close).abs();
                dist1.max(dist2).max(dist3)
            }
            None => bar.high - bar.low,
        };
        self.pre_close = Some(bar.close);
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
        let tr = self.tr.next(input);
        self.ema.next(tr)
    }
}
