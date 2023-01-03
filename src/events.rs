use crate::model::Model;
use crate::{Particle, HEIGHT, WIDTH};
use nannou::event::ElementState::{Pressed, Released};
use nannou::event::MouseScrollDelta::PixelDelta;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;
use nannou::winit::event::WindowEvent::MouseInput;

pub fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => match key {
            VirtualKeyCode::Space => {
                model.paused = !model.paused;
            }
            VirtualKeyCode::A => {
                model.drawing = true;
            }
            VirtualKeyCode::Z => {
                model.center = vec2(0.0, 0.0);
                model.scale = 1.0;
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
                model.center = vec2(0.0, 0.0);
                model.scale = 1.0;
            }
            VirtualKeyCode::P => {
                model.config.scale += 0.5;
            }
            VirtualKeyCode::O => {
                model.config.scale -= 0.5;
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
            if model.mouse_pressed {
                model.center =
                    model.previous_center + (point - model.pressed_location) / model.scale;
            }
        }
        MousePressed(_) => {
            model.mouse_pressed = true;
            model.pressed_location = app.mouse.position();
            model.previous_center = model.center;
        }
        MouseReleased(_) => {
            model.mouse_pressed = false;
            model.previous_center = model.center
        }
        MouseWheel(scroll, phase) => {
            model.scale += match scroll {
                PixelDelta(p) => p.y as f32 / 100.0,
                _ => 0.0,
            };
            if model.scale < 0.5 {
                model.scale = 0.5;
            }
        }
        _ => {}
    }
}
