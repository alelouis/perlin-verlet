use crate::model::Model;
use crate::{Particle, HEIGHT, WIDTH};
use nannou::noise::NoiseFn;
use nannou::prelude::*;

// https://youtu.be/lS_qeBy3aQI

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let dt = model.config.dt;
    model.framerates[0] = 1.0 / app.duration.since_prev_update.as_secs_f32();
    model.framerates.rotate_left(1);

    let sub_steps = 8;
    let sub_dt = dt / sub_steps as f32;
    if !model.paused {
        for i in 0..sub_steps {
            apply_gravity(&mut model.particles);
            update_positions(&mut model.particles, sub_dt);
            apply_constraints(&mut model.particles);
            solve_collisions(&mut model.particles);
        }
    }
}

fn apply_gravity(particles: &mut Vec<Particle>) {
    let gravity = vec2(0.0, -1000.0);
    for particle in particles {
        particle.accelerate(gravity);
    }
}

fn update_positions(particles: &mut Vec<Particle>, dt: f32) {
    for particle in particles {
        particle.update_position(dt);
    }
}

fn solve_collisions(particles: &mut Vec<Particle>) {
    for i in 0..particles.len() {
        for j in i + 1..particles.len() {
            let collision_axis = particles[i].position - particles[j].position;
            let dist = collision_axis.length();
            if dist < particles[i].radius + particles[j].radius {
                let n = collision_axis / dist;
                let delta = particles[i].radius + particles[j].radius - dist;
                particles[i].position += 0.5 * delta * n;
                particles[j].position -= 0.5 * delta * n;
            }
        }
    }
}

fn apply_constraints(particles: &mut Vec<Particle>) {
    let position = vec2(0.0, 0.0);
    let radius = 300f32;
    for particle in particles {
        // let to_particle = particle.position - position;
        // let dist = to_particle.length();
        // if dist > radius - particle.radius {
        //     let n = to_particle / dist;
        //     particle.position = position + n * (radius - particle.radius);
        // }
        let limit = -(WIDTH as f32) / 2.0 + particle.radius;
        if particle.position.x < limit {
            let d = particle.position.x - limit;
            particle.position.x = particle.position.x - d;
        }
        let limit = (WIDTH as f32) / 2.0 - particle.radius;
        if particle.position.x > limit {
            let d = particle.position.x - limit;
            particle.position.x = particle.position.x - d;
        }
        let limit = -(HEIGHT as f32) / 2.0 + particle.radius;
        if particle.position.y <= limit {
            let d = particle.position.y - limit;
            particle.position.y = particle.position.y - d;
        }
    }
}
