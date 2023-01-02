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

fn compute_ellipse(radius: f32) -> [Point3; 8] {
    let points: [Point3; 8] = (0..8)
        .map(|k| (2.0 * PI * k as f32 / 8.0))
        .map(|k| pt3(radius * (k as f32).cos(), radius * (k as f32).sin(), 0.0))
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong size iterator");
    points
}
