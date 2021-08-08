use super::Next;

pub struct MA {
    period: usize,
    index: usize,
    count: usize,
    sum: f64,
    deque: Box<[f64]>,
}

impl MA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            index: 0,
            count: 0,
            sum: 0.0,
            deque: vec![0.0; period].into_boxed_slice(),
        }
    }
}

impl Next<f64> for MA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let old_val = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period {
            self.index + 1
        } else {
            0
        };

        if self.count < self.period {
            self.count += 1;
        }
        self.sum = self.sum - old_val + input;
        self.sum / (self.count as f64)
    }
}
