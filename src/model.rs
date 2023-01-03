use crate::events::event;
use crate::view::view;
use crate::{Config, Particle, HEIGHT, WIDTH};
use nannou::prelude::*;

pub struct Model {
    _window: window::Id,
    pub(crate) particles: Vec<Particle>,
    pub(crate) framerates: Vec<f32>,
    pub(crate) config: Config,
    pub(crate) paused: bool,
    pub(crate) drawing: bool,
    pub(crate) clear: bool,
    pub(crate) center: Vec2,
    pub(crate) previous_center: Vec2,
    pub(crate) scale: f32,
    pub(crate) mouse_pressed: bool,
    pub(crate) pressed_location: Point2,
    pub(crate) texture: wgpu::Texture,
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
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("circle.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        _window,
        particles,
        framerates,
        config,
        paused: false,
        drawing: false,
        clear: false,
        scale: 1.0,
        center: vec2(0.0, 0.0),
        previous_center: vec2(0.0, 0.0),
        mouse_pressed: false,
        pressed_location: pt2(0.0, 0.0),
        texture,
    }
}
