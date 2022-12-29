use crate::model::Model;
use nannou::prelude::*;
use nannou::winit::event::VirtualKeyCode;

pub fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            if key == VirtualKeyCode::Space {
                model.paused = !model.paused;
            }
        }
        _ => {}
    }
}
