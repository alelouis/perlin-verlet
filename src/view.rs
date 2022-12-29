use crate::model::Model;
use crate::{HEIGHT, WIDTH};
use nannou::prelude::*;

pub fn view(app: &App, model: &Model, frame: Frame) {
    // Get draw
    let draw = app.draw();

    // First clear
    if app.elapsed_frames() < 1 {
        frame.clear(BLACK);
    }

    // Draw particles
    for particle in &model.particles {
        draw.ellipse()
            .resolution(4.0)
            .x_y(particle.position.x, particle.position.y)
            .w(2.0)
            .h(2.0)
            .color(WHITE);
    }

    // Clear fade
    draw.rect()
        .width(WIDTH as f32)
        .h(HEIGHT as f32)
        .color(srgba(0.0, 0.0, 0.0, model.config.fading));

    // Info display
    let side = 120.0;
    let height = 45.0;
    let top_left = pt2(-(WIDTH as f32 / 2.0), HEIGHT as f32 / 2.0);
    let offset = vec2(side, -20.0);
    let xy = top_left + offset;
    draw.rect()
        .xy(top_left + vec2(side / 2.0, -height / 2.0))
        .w_h(side, height)
        .color(BLACK);

    // Info
    let framerate: f32 = model.framerates.iter().sum::<f32>() / model.framerates.len() as f32;
    draw.text(&*format!(
        "fps = {:.1}\nparticles = {}",
        framerate,
        model.particles.len()
    ))
    .font_size(12)
    .xy(xy)
    .left_justify()
    .line_spacing(0.01);

    // Frame update
    draw.to_frame(app, &frame).unwrap();
}
