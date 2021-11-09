use crate::{
    canvas::Canvas,
    math::{Point, Vector},
    sim::{Environment, Projectile, Simulator},
};

mod canvas;
mod math;
mod sim;

fn main() {
    println!("Canon ball initialization...\n");

    let env = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    let proj = Projectile {
        pos: Point::new(0., 1., 0.),
        v: 11.25 * Vector::new(1., 1.8, 0.).normalize(),
    };

    let mut cv = Canvas::new(900, 550, None);

    let mut canon = Simulator::new(env, proj);

    let ppm = canon.draw(&mut cv);

    ppm.write_to_file("trajectory.ppm");
}
