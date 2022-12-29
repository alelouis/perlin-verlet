use crate::model::Model;
use crate::{HEIGHT, WIDTH};
use nannou::noise::NoiseFn;
use nannou::prelude::*;

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let dt = model.config.dt;
    let scale = model.config.scale;
    let time = app.time / model.config.time_dilation;
    for p in model.particles.iter_mut() {
        let new_pos = p.position + p.speed * dt + 0.5 * p.acceleration * dt.powi(2);
        let acc_x = 1.0
            * &model.noise.get([
                (scale * p.position.x / WIDTH as f32) as f64,
                (scale * p.position.y / HEIGHT as f32) as f64,
                time as f64,
            ]);
        let acc_y = 1.0
            * &model.noise.get([
                (scale * p.position.x / WIDTH as f32) as f64,
                (scale * p.position.y / HEIGHT as f32) as f64,
                (time + 1.0) as f64,
            ]);
        let new_acc = Vec2::new(acc_x as f32, acc_y as f32);
        let mut new_speed = p.speed + 0.5 * (p.acceleration + new_acc) * dt;
        new_speed = new_speed.clamp_length(0.0, 0.1);
        p.position = new_pos;
        p.speed = new_speed;
        p.acceleration = new_acc;
    }
}
