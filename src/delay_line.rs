// A multi-channel delay line with linear interpolation
pub struct DelayLine<T> {
    buf_size: usize,
    data: Vec<T>,
    write_indexes: Vec<usize>,
}

impl<T: num::Float> DelayLine<T> {
    pub fn new(buf_size: usize, num_channels: usize) -> Self {
        let data: Vec<T> = vec![T::zero(); buf_size * num_channels];
        let write_indexes = vec![0; num_channels];
        Self {
            buf_size,
            data,
            write_indexes,
        }
    }

    #[inline]
    pub fn write(&mut self, channel: usize, sample: T) {
        let ix = self.write_indexes[channel] + (channel * self.buf_size);
        self.data[ix] = sample;
        self.write_indexes[channel] += 1;
        self.write_indexes[channel] %= self.buf_size;
    }

    #[inline]
    pub fn read(&self, channel: usize, mut delay_samples: usize) -> &T {
        if delay_samples > self.data.len() {
            delay_samples %= self.data.len()
        }
        let mut index: isize = (self.write_indexes[channel] as isize) -
            (delay_samples as isize) - 1;
        if index < 0 {
            index += self.data.len() as isize;
        }
        self.data.get(index as usize).unwrap()
    }

    #[inline]
    pub fn read_interpolated(&self, channel: usize, delay: T) -> T {
        // Read from the int index
        let floored = delay.floor().to_usize().unwrap();
        let y1 = *self.read(channel, floored);
        let y2 = *self.read(channel, floored + 1);
        let fraction = delay - T::from(floored).unwrap();
        y1 + fraction * (y2 - y1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_delays() {
        let mut delay: DelayLine<f32> = DelayLine::new(3, 1);
        assert_eq!(delay.read(0, 11).to_owned(), 0.0);
        delay.write(0, 1.0);
        delay.write(0, 2.0);
        assert_eq!(delay.read(0, 1).to_owned(), 1.0);
        assert_eq!(delay.read(0, 0).to_owned(), 2.0);
    }

    #[test]
    fn it_is_circular() {
        let mut delay: DelayLine<f32> = DelayLine::new(3, 1);
        for i in 1..11 {
            delay.write(0, i as f32);
        }
        assert_eq!(delay.read(0, 1).to_owned(), 9.0);
    }

    #[test]
    fn it_interpolates() {
        let mut delay: DelayLine<f32> = DelayLine::new(3, 1);
        delay.write(0, 1.0);
        delay.write(0, 2.0);
        assert_eq!(delay.read_interpolated(0, 0.5), 1.5);
    }
}
