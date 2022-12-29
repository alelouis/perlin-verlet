use crate::events::event;
use crate::view::view;
use crate::{Config, Particle, HEIGHT, WIDTH};
use nannou::noise::Perlin;
use nannou::prelude::*;

pub struct Model {
    _window: window::Id,
    pub(crate) particles: Vec<Particle>,
    pub(crate) framerates: Vec<f32>,
    pub(crate) noise: Perlin,
    pub(crate) config: Config,
    pub(crate) paused: bool,
}

pub fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .event(event)
        .title("dev")
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let config: Config = toml::from_str(include_str!("conf.toml")).unwrap();
    let framerates = vec![60.0; 120];
    let mut particles: Vec<Particle> = vec![];
    let noise = Perlin::new();
    for _ in 0..config.n_particles {
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
        framerates,
        noise,
        config,
        paused: false,
    }
}
