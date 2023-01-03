use crate::events::event;
use crate::view::view;
use crate::{compute_ellipse, Config, Particle, HEIGHT, WIDTH};
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
    pub(crate) ellipse: [[Point3; 3]; 6],
    pub(crate) center: Vec2,
    pub(crate) previous_center: Vec2,
    pub(crate) scale: f32,
    pub(crate) mouse_pressed: bool,
    pub(crate) pressed_location: Point2,
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
    let particles: Vec<Particle> = vec![];
    let noise = Perlin::new();
    let ellipse = compute_ellipse(3.0);

    Model {
        _window,
        particles,
        framerates,
        noise,
        config,
        ellipse,
        paused: false,
        drawing: false,
        clear: false,
        scale: 1.0,
        center: vec2(0.0, 0.0),
        previous_center: vec2(0.0, 0.0),
        mouse_pressed: false,
        pressed_location: pt2(0.0, 0.0),
    }
}
