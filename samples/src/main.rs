use anim8::{Driver, Looper};
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line},
    style::{PrimitiveStyle, TextStyle},
};
use embedded_graphics_simulator::{SimulatorDisplay, SimulatorEvent, WindowBuilder};
use std::thread;
use std::time::{Duration, Instant};

fn polar(center: Point, angle: f32, radius: f32) -> Point {
    center + Point::new((angle.cos() * radius) as i32, (angle.sin() * radius) as i32)
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(129, 129));
    let mut window = WindowBuilder::new(&display).title("Anim8").scale(4).build();

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    let center = Point::new(64, 64);

    let start = Instant::now();
    let driver = Driver::new(
        0.0,
        2.0 * std::f32::consts::PI,
        Duration::from_millis(10000),
    );
    let looper = Looper::new(driver, None);

    'running: loop {
        display.clear(BinaryColor::Off)?;

        Line::new(center, polar(center, looper.step(start.elapsed()), 50.0))
            .into_styled(line_style)
            .draw(&mut display)?;

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }

        thread::sleep(Duration::from_millis(16));
    }
}
