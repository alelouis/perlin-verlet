use crate::model::Model;
use crate::{HEIGHT, WIDTH};
use nannou::geom::Tri;
use nannou::prelude::*;
use std::f64::consts::TAU;

pub fn view(app: &App, model: &Model, frame: Frame) {
    // Get draw
    let mut world_draw = app.draw();
    let ui_draw = app.draw();
    world_draw = world_draw.scale(10.0);

    // First clear
    if (app.elapsed_frames() < 1) | (model.clear) {
        frame.clear(BLACK);
    }
    let mut tris: Vec<Tri> = vec![];
    let mut colors: Vec<Hsva> = vec![];

    // Draw particles
    if !model.paused {
        for particle in &model.particles {
            for i in 0..6 {
                tris.push(Tri([
                    model.ellipse[0] + particle.position.extend(0.0),
                    model.ellipse[i + 1] + particle.position.extend(0.0),
                    model.ellipse[i + 2] + particle.position.extend(0.0),
                ]));
            }
        }

        for particle in &model.particles {
            for _ in 0..6 {
                colors.push(hsva(
                    (TAU as f32 * particle.age / model.config.lifetime).sin(),
                    1.0,
                    0.8,
                    1.0,
                ))
            }
        }

        let tris_col = tris
            .iter()
            .zip(colors)
            .map(|(tri, color)| tri.map_vertices(|v| (v, color)));

        // Draw colors
        world_draw.mesh().tris_colored(tris_col);

        // Clear fade
        world_draw
            .rect()
            .width(WIDTH as f32)
            .h(HEIGHT as f32)
            .color(srgba(0.0, 0.0, 0.0, model.config.fading));
    }

    // Info
    let side = 120f32;
    let height = 70f32;
    let top_left = pt2(-(WIDTH as f32 / 2.0), HEIGHT as f32 / 2.0);
    let offset = vec2(side, -30.0);
    let xy = top_left + offset;
    ui_draw
        .rect()
        .xy(top_left + vec2(side / 2.0, -height / 2.0))
        .w_h(side, height)
        .color(BLACK);

    let framerate: f32 = model.framerates.iter().sum::<f32>() / model.framerates.len() as f32;
    let drawing = if model.drawing {
        "drawing"
    } else {
        "not drawing"
    };
    let paused = if model.paused { "paused" } else { "running" };
    ui_draw
        .text(&*format!(
            "fps = {:.1}\nparticles = {}\nscale = {}\n{}\n{}",
            framerate,
            model.particles.len(),
            model.config.scale,
            drawing,
            paused
        ))
        .font_size(12)
        .xy(xy)
        .left_justify()
        .line_spacing(0.01);

    // Frame update
    ui_draw.to_frame(app, &frame).unwrap();
    world_draw.to_frame(app, &frame).unwrap();
}
