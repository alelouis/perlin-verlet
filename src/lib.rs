pub mod controller;
pub mod events;
pub mod model;
mod view;

use nannou::geom::{Quad, Tri};
use nannou::prelude::*;
use serde::Deserialize;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 700;

#[derive(Default, Debug)]
pub struct Particle {
    pub position: Vec2,
    pub position_old: Vec2,
    pub acceleration: Vec2,
    pub age: f32,
    pub radius: f32,
}

impl Particle {
    pub fn new(position: Vec2, acceleration: Vec2, radius: f32) -> Self {
        Self {
            position,
            position_old: position,
            acceleration,
            age: 0f32,
            radius,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.position - self.position_old;
        self.position_old = self.position;
        self.position = self.position + velocity + self.acceleration * dt * dt;
        self.acceleration = vec2(0.0, 0.0);
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration += acc;
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

fn generate_quad(width: f32, offset: Vec2) -> Quad<Point2> {
    let halfw = width;
    let quad = geom::Quad::from([
        pt2(-0.5 * halfw + offset.x, 0.5 * halfw + offset.y),
        pt2(0.5 * halfw + offset.x, 0.5 * halfw + offset.y),
        pt2(0.5 * halfw + offset.x, -0.5 * halfw + offset.y),
        pt2(-0.5 * halfw + offset.x, -0.5 * halfw + offset.y),
    ]);
    quad
}
