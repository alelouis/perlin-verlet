use crate::model::Model;
use crate::{generate_quad, HEIGHT, WIDTH};
use nannou::draw::theme::Primitive::Quad;
use nannou::geom::Tri;
use nannou::mesh::TexCoords;
use nannou::prelude::*;
use std::f64::consts::TAU;

pub fn view(app: &App, model: &Model, frame: Frame) {
    // Get draw
    let mut world_draw = app.draw();
    let ui_draw = app.draw();
    world_draw = world_draw.scale(model.scale);
    world_draw = world_draw.translate(vec3(model.center.x, model.center.y, 0.0));

    frame.clear(BLACK);
    let mut points_texture: Vec<(Point3, Point2)> = vec![];
    let mut points_color: Vec<(Point3, Hsva)> = vec![];
    for particle in &model.particles {
        let quad = generate_quad(particle.radius * 2.0, vec2(0.0, 0.0));
        let p: Vec<(Point3, Point2, Hsva)> = quad
            .triangles_iter()
            .flat_map(Tri::vertices)
            .map(|point| {
                (
                    pt3(
                        point.x + particle.position.x,
                        point.y + particle.position.y,
                        0.0,
                    ),
                    pt2(
                        point.x / (particle.radius * 2f32) + 0.5,
                        point.y / (particle.radius * 2f32) + 0.5,
                    ),
                    hsva(
                        map_range(particle.radius, 3.0, 6.0, 0.0, 1.0),
                        1.0,
                        1.0,
                        1.0,
                    ),
                )
            })
            .collect();
        for (p3, p2, hs) in p {
            points_texture.push((p3, p2));
            points_color.push((p3, hs));
        }
    }

    // world_draw.mesh().points_colored(points_color);

    world_draw
        .mesh()
        .points_textured(&model.texture, points_texture);

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
            "fps = {:.1}\nzoom = {:.1}\nparticles = {}\nscale = {}\n{}\n{}",
            framerate,
            model.scale,
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
