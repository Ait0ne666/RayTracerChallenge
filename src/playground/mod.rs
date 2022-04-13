use crate::prelude::{prelude::*, point, vector};

pub fn tick(env: Environment, proj: Projectile) -> Projectile {
    let new_position = proj.position + proj.velocity;
    let new_velocity = proj.velocity + env.gravity + env.wind;

    Projectile::new(new_position, new_velocity)
}

pub fn init() -> (Environment, Projectile) {
    let p = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.8, 0.0).normalize()*12.25);

    let e = Environment::new(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));

    return (e, p);
}
