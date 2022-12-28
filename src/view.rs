use nannou::prelude::*;
use crate::{WIDTH, HEIGHT};
use crate::model::Model;

pub fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() < 1 {
        frame.clear(BLACK);
    }
    let draw = app.draw();
    for particle in &model.particles {
        draw.ellipse()
            .resolution(4.0)
            .x_y(particle.position.x, particle.position.y)
            .w(2.0)
            .h(2.0)
            .color(WHITE);
    }
    draw.rect()
        .width(WIDTH as f32)
        .h(HEIGHT as f32)
        .color(srgba(0.0, 0.0, 0.0, 0.01));
    draw.to_frame(app, &frame).unwrap();
}
