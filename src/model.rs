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
    pub(crate) drawing: bool,
    pub(crate) clear: bool,
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

    Model {
        _window,
        particles,
        framerates,
        noise,
        config,
        paused: false,
        drawing: false,
        clear: false,
    }
}
