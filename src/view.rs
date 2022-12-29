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
    let side = 30.0;
    let top_left = pt2(-(WIDTH as f32 / 2.0), HEIGHT as f32 / 2.0);
    let offset = vec2(side, -side / 2.0);
    let xy = top_left + offset;
    draw.rect().xy(xy).w_h(side * 2.0, side).color(BLACK);

    // Framerate
    draw.text(&*format!(
        "fps = {:.1?}",
        1.0 / app.duration.since_prev_update.as_secs_f32()
    ))
    .font_size(12)
    .xy(xy);

    // Frame update
    draw.to_frame(app, &frame).unwrap();
}
