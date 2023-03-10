use crate::model::Model;
use crate::{HEIGHT, WIDTH};
use nannou::noise::NoiseFn;
use nannou::prelude::*;

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let dt = model.config.dt;
    let scale = model.config.scale;
    let time = app.time / model.config.time_dilation;
    model.framerates[0] = 1.0 / app.duration.since_prev_update.as_secs_f32();
    model.framerates.rotate_left(1);
    if !model.paused {
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
            let new_speed = p.speed + 0.5 * (p.acceleration + new_acc) * dt;
            //new_speed = new_speed.clamp_length(0.0, 1.0);
            p.position = new_pos;
            p.speed = new_speed;
            let friction = 0.8f32;
            p.acceleration = new_acc - friction * new_speed;
            p.age += 1.0 / 60.0;
        }
        model.particles.retain(|p| {
            (p.position.x > -(WIDTH as f32) / 2.0)
                & (p.position.x < (WIDTH as f32) / 2.0)
                & (p.position.y > -(HEIGHT as f32) / 2.0)
                & (p.position.y < (HEIGHT as f32) / 2.0)
            //& (p.age < model.config.lifetime)
        });
    }
}
