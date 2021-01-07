#![no_std]

use core::marker::PhantomData;
use embedded_time::duration::Milliseconds;

// ---

pub trait Easing {
    fn duration(&self) -> Milliseconds;

    fn value_at(&self, time: Milliseconds) -> f32;
}

pub struct LinearEasing {
    duration: Milliseconds,
}

impl Easing for LinearEasing {
    fn duration(&self) -> Milliseconds {
        self.duration
    }

    fn value_at(&self, time: Milliseconds) -> f32 {
        time.0 as f32 / self.duration.0 as f32
    }
}

// ---

pub enum Mode {
    /// Loop the animation forever.
    Loop,
    // /// Run the animation once, leaving it in its final state once complete.
    // OneShot,

    // // Run the animation once, then reset to beginning state once complete.
    // Reset,
}

struct EasingDriver<E> {
    easing: E,
    mode: Mode,
}

impl<E> EasingDriver<E>
where
    E: Easing,
{
    pub fn new(easing: E, mode: Mode) -> Self {
        Self { easing, mode }
    }

    pub fn drive(&self, time: Milliseconds) -> f32 {
        let duration = self.easing.duration();

        let time = match self.mode {
            Mode::Loop => time % duration,
        };

        self.easing.value_at(time)
    }
}
