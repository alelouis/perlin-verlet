pub mod controller;
pub mod events;
pub mod model;
mod view;

use nannou::prelude::*;
use serde::Deserialize;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 700;

#[derive(Default, Debug)]
pub struct Particle {
    pub position: Vec2,
    pub speed: Vec2,
    pub acceleration: Vec2,
    pub age: f32,
}

impl Particle {
    pub fn new(position: Vec2, speed: Vec2, acceleration: Vec2) -> Self {
        Self {
            position,
            speed,
            acceleration,
            age: 0f32,
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    n_particles: u32,
    scale: f32,
    dt: f32,
    time_dilation: f32,
    fading: f32,
    lifetime: f32,
}
