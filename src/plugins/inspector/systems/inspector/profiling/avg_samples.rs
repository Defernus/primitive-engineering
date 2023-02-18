use std::collections::LinkedList;

pub struct AvgSamples {
    size: usize,
    samples: LinkedList<f32>,
}

impl Default for AvgSamples {
    fn default() -> Self {
        Self {
            samples: LinkedList::new(),
            size: 100,
        }
    }
}

impl AvgSamples {
    pub fn new(size: usize) -> Self {
        Self {
            samples: LinkedList::new(),
            size,
        }
    }

    pub fn update(&mut self, new_sample: f32) {
        self.samples.push_back(new_sample);
        if self.samples.len() > self.size {
            self.samples.pop_front();
        }
    }

    pub fn avg(&self) -> f32 {
        self.samples.iter().sum::<f32>() / self.samples.len() as f32
    }

    pub fn min(&self) -> f32 {
        *self
            .samples
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}
