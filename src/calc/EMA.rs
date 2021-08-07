use super::Next;

pub struct EMA {
    period: usize,
    k: f64,
    current: f64,
    is_new: bool,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            k: 2.0 / (period + 1) as f64,
            current: 0.0,
            is_new: true,
        }
    }
}

impl Next<f64> for EMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.is_new {
            self.is_new = false;
            self.current = input;
        } else {
            self.current = self.k * input + (1.0 - self.k) * self.current;
        }
        self.current
    }
}
