use crate::model::Model;
use crate::{HEIGHT, WIDTH};
use nannou::geom::Tri;
use nannou::prelude::*;
use std::f64::consts::TAU;

pub fn view(app: &App, model: &Model, frame: Frame) {
    // Get draw
    let mut world_draw = app.draw();
    let ui_draw = app.draw();
    world_draw = world_draw.scale(model.scale);
    world_draw = world_draw.translate(vec3(model.center.x, model.center.y, 0.0));

    let mut tris: Vec<Tri> =
        vec![Tri([pt3(0.0, 0.0, 0.0), pt3(0.0, 0.0, 0.0), pt3(0.0, 0.0, 0.0),]); 100_000];

    // Draw particles
    frame.clear(BLACK);
    let mut tri_len = 0;
    for (idx_part, particle) in model.particles.iter().enumerate() {
        for (idx_tri, ell_tri) in model.ellipse.iter().enumerate() {
            let mut ell_tri_trans = [pt3(0.0, 0.0, 0.0); 3];
            for i in 0..3 {
                ell_tri_trans[i] = ell_tri[i] + particle.position.extend(0.0);
            }
            tris[idx_part * 6 + idx_tri] = Tri(ell_tri_trans);
            tri_len += 1;
        }
    }

    world_draw.mesh().tris(tris);

    // Info
    let side = 120f32;
    let height = 100f32;
    let top_left = pt2(-(WIDTH as f32 / 2.0), HEIGHT as f32 / 2.0);
    let offset = vec2(side, -50.0);
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
            "fps = {:.1}\nzoom = {:.1}\nparticles = {}\ntriangles = {}\nscale = {}\n{}\n{}",
            framerate,
            model.scale,
            model.particles.len(),
            tri_len,
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
