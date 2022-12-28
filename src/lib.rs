pub mod model;
pub mod controller;
mod view;

use nannou::prelude::*;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 700;

#[derive(Default, Debug)]
pub struct Particle {
    pub position: Vec2,
    pub speed: Vec2,
    pub acceleration: Vec2,
}

impl Particle {
    pub fn new(position: Vec2, speed: Vec2, acceleration: Vec2) -> Self {
        Self {
            position,
            speed,
            acceleration,
        }
    }
}
