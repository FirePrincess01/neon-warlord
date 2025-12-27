//! A sliding average using u32 for better performance

pub struct SlidingAverage {
    values: Vec<u32>,
    average: u32,

    index: usize,
}

impl SlidingAverage {
    pub fn new(size: usize) -> Self {
        let values = vec![0; size];

        let average = 0;
        let index = 0;

        Self {
            values,
            average,
            index,
        }
    }

    pub fn average(&self) -> u32 {
        self.average / self.values.len() as u32
    }

    pub fn push(&mut self, value: u32) {
        self.index = (self.index + 1) % self.values.len();

        let last_value = self.values[self.index];
        self.values[self.index] = value;

        self.average = self.average - last_value + value;
    }
}
