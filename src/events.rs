use crate::model::Model;
use crate::{Particle, HEIGHT, WIDTH};
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

pub fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => match key {
            VirtualKeyCode::Space => {
                model.paused = !model.paused;
            }
            VirtualKeyCode::A => {
                model.drawing = true;
            }
            VirtualKeyCode::G => {
                for _ in 0..model.config.n_particles {
                    model.particles.push(Particle::new(
                        vec2(
                            random_range(-(WIDTH as f32) / 2.0, (WIDTH as f32) / 2.0),
                            random_range(-(HEIGHT as f32) / 2.0, (HEIGHT as f32) / 2.0),
                        ),
                        Default::default(),
                        Default::default(),
                    ));
                }
            }
            VirtualKeyCode::C => {
                model.particles.clear();
                model.clear = true;
            }
            VirtualKeyCode::Escape => {
                std::process::exit(0);
            }
            _ => {}
        },
        KeyReleased(key) => match key {
            VirtualKeyCode::A => {
                model.drawing = false;
            }
            VirtualKeyCode::C => {
                model.clear = false;
            }
            _ => {}
        },
        MouseMoved(point) => {
            if model.drawing {
                model.particles.push(Particle::new(
                    vec2(point.x, point.y),
                    Default::default(),
                    Default::default(),
                ));
            }
        }
        _ => {}
    }
}
