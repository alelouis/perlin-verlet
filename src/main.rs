use perlin_verlet::controller::update;
use perlin_verlet::model::model;

fn main() {
    nannou::app(model).update(update).run();
}
