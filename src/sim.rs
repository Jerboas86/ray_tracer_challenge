use crate::math::{Point, Vector};

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
}
