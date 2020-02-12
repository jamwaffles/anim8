// #![no_std]
// #![deny(missing_docs)]

use core::time::Duration;

struct LinearEasing {}

impl LinearEasing {
    pub fn new() -> Self {
        Self {}
    }

    pub fn map_norm(&self, norm: f32) -> f32 {
        norm
    }
}

pub struct Driver {
    start: f32,
    end: f32,
    duration: Duration,
    easing: LinearEasing,
}

impl Driver {
    pub fn new(start: f32, end: f32, duration: Duration) -> Self {
        Self {
            start,
            end,
            duration,
            easing: LinearEasing::new(),
        }
    }

    pub fn step(&self, position: Duration) -> f32 {
        let norm = position.as_secs_f32() / self.duration.as_secs_f32();

        self.norm(norm)
    }

    fn norm(&self, norm: f32) -> f32 {
        let eased = self.easing.map_norm(norm);

        dbg!(self.start as f32);
        dbg!(self.end as f32);
        dbg!(eased);
        dbg!(norm);

        self.start as f32 + ((self.end as f32 - self.start as f32) * eased)
    }
}

pub struct Looper {
    driver: Driver,
    iterations: Option<usize>,
}

impl Looper {
    pub fn new(driver: Driver, iterations: Option<usize>) -> Self {
        Self { driver, iterations }
    }

    pub fn step(&self, position: Duration) -> f32 {
        let num_iterations = position.as_secs_f32() / self.driver.duration.as_secs_f32();

        // let whole_iters = num_iterations.trunc();
        let whole_iters = num_iterations;

        // if let Some(iterations) = self.iterations {
        //     if whole_iters > iterations as f32 {
        //         return None;
        //     }
        // }

        // let ass = num_iterations.fract();

        let ass = if whole_iters > 1.0 {
            whole_iters - whole_iters.trunc()
        } else {
            whole_iters
        };

        // println!(
        //     "Step {}, pos {}, whole_iters {}",
        //     num_iterations,
        //     whole_iters
        // );

        self.driver.norm(ass)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_step() {
        let steps = 20;

        let driver = Driver::new(10.0, 20.0, Duration::from_millis(steps));

        for i in 0..=steps {
            println!("{} ms: {:?}", i, driver.step(Duration::from_millis(i)));
        }
    }

    #[test]
    fn looper() {
        let steps = 60;

        let driver = Driver::new(10.0, 20.0, Duration::from_millis(20));

        let looper = Looper::new(driver, Some(5));

        for i in 0..=steps {
            println!("{} ms: {:?}\n", i, looper.step(Duration::from_millis(i)));
            // looper.step(Duration::from_millis(i));
        }
    }
}
