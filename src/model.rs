use crate::{Particle, WIDTH, HEIGHT};
use nannou::prelude::*;
use nannou::noise::Perlin;
use crate::view::view;

pub struct Model {
    _window: window::Id,
    pub(crate) particles: Vec<Particle>,
    pub(crate) noise: Perlin,
}

pub fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    let mut particles: Vec<Particle> = vec![];
    let noise = Perlin::new();
    let n_particles = 10000;
    for _ in 0..n_particles {
        particles.push(Particle::new(
            vec2(
                random_range(-(WIDTH as f32) / 2.0, (WIDTH as f32) / 2.0),
                random_range(-(HEIGHT as f32) / 2.0, (HEIGHT as f32) / 2.0),
            ),
            Default::default(),
            Default::default(),
        ));
    }

    Model {
        _window,
        particles,
        noise,
    }
}
