// #![no_std]
// #![deny(missing_docs)]

use core::time::Duration;

struct Driver<V> {
    initial_value: V,
    final_value: V,
    duration: Duration,
}

impl Driver<i32> {
    pub fn new(initial_value: i32, final_value: i32, duration: Duration) -> Self {
        Self {
            initial_value,
            final_value,
            duration,
        }
    }

    /// Get the animated value at a given time along the animation
    pub fn step(&self, time: Duration) -> Option<f32> {
        if time > self.duration || time < Duration::new(0, 0) {
            None
        } else {
            let time_norm = time.div_f32(self.duration.as_secs_f32());

            let delta = (self.final_value - self.initial_value) as f32;

            Some(self.initial_value as f32 + (delta * time_norm.as_secs_f32()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_step() {
        let steps = 20;

        let driver = Driver::new(10, 20, Duration::from_millis(steps));

        for i in 0..=steps {
            println!("{} ms: {:?}", i, driver.step(Duration::from_millis(i)));
        }
    }
}
