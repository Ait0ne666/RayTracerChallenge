use super::*;


#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub position: Vector,
    pub velocity: Vector
}




impl Projectile {
    

    pub fn new(position: Vector, velocity: Vector) -> Self {


        Projectile{position: position, velocity: velocity}
    }
}