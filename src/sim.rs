use crate::{
    canvas::{Canvas, Color, Ppm},
    math::{Point, Vector},
};

pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

pub struct Projectile {
    pub pos: Point,
    pub v: Vector,
}

pub struct Simulator {
    env: Environment,
    proj: Projectile,
}

impl Simulator {
    pub fn new(env: Environment, proj: Projectile) -> Self {
        Self { env, proj }
    }
    pub fn tick(&mut self) -> &Projectile {
        self.proj.pos += &self.proj.v;
        self.proj.v += &self.env.gravity + &self.env.wind;

        &self.proj
    }

    pub fn draw(&mut self, canvas: &mut Canvas) -> Ppm {
        println!("Canon ball running...");

        let c = Color::new(1., 0., 0.);
        let height = canvas.get_height();

        loop {
            let new_proj = self.tick();
            println!("Still flying...");

            canvas.write_pixel(
                new_proj.pos.0 as usize,
                height - new_proj.pos.1 as usize,
                &c,
            );

            if new_proj.pos.1 <= 0. {
                println!("Hit ground !!!");
                break;
            }
        }

        canvas.to_ppm()
    }
}
