use perlin_verlet::model::model;
use perlin_verlet::controller::update;

fn main() {
    nannou::app(model).update(update).run();
}