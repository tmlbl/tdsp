use num::Float;
use std::f64::consts::PI;

pub struct LFO<T: Float> {
    frequency: T,
    sample_rate: T,
    phase: T,
    inc: T,
}

impl<T: Float> LFO<T> {
    pub fn new(frequency: usize, sample_rate: usize) -> Self {
        let mut instance = Self {
            frequency: T::from(frequency).unwrap(),
            sample_rate: T::from(sample_rate).unwrap(),
            phase: T::zero(),
            inc: T::zero(),
        };
        instance.update();
        instance
    }

    fn update(&mut self) {
        self.inc = self.frequency / self.sample_rate;
    }

    pub fn set_frequency(&mut self, frequency: T) {
        self.frequency = frequency;
        self.update();
    }

    pub fn next(&mut self) -> T {
        self.phase = self.phase + self.inc;
        if self.phase > T::one() {
            self.phase = T::zero();
        }
        let pi = T::from(PI).unwrap();
        let angle = self.phase * T::from(2.0).unwrap() * pi - pi;
        angle.sin()
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_oscillates() {
        let mut lfo: LFO<f32> = LFO::new(30, 1000);
        for _i in 1..1000 {
            let y = lfo.next();
            assert!(y >= -1.0 && y <= 1.0);   
        }    
    }
}
