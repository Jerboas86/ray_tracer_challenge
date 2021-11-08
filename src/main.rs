use crate::{
    math::{Point, Vector},
    sim::{Environment, Projectile, Simulator},
};

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
        v: Vector::new(1., 1., 0.).normalize(),
    };

    let mut canon = Simulator::new(env, proj);

    println!("Canon ball running...");

    loop {
        let new_proj = canon.tick();
        println!("Still flying...");

        if new_proj.pos.1 <= 0. {
            println!("Hit ground !!!");
            break;
        }
    }
}
