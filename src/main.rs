use perlin_verlet::controller::update;
use perlin_verlet::model::model;

fn main() {
    let app = nannou::app(model).update(update);
    app.run();
}
