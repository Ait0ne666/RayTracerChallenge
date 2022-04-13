

mod projectile;
mod gravity;
mod vector;
mod color;
mod canvas;
mod matrix;
mod ray;
mod sphere;
mod intersection;
mod object;
mod transform;
mod light;
mod material;
mod world;
mod camera;


pub mod prelude {
    pub use crate::models::projectile::*;
    pub use crate::models::vector::*;
    pub use crate::models::gravity::*;
    pub use crate::models::color::*;
    pub use crate::models::canvas::*;
    pub use crate::models::matrix::*;
    pub use crate::models::ray::*;
    pub use crate::models::sphere::*;
    pub use crate::models::intersection::*;
    pub use crate::models::object::*;
    pub use crate::models::transform::*;
    pub use crate::models::light::*;
    pub use crate::models::material::*;
    pub use crate::models::world::*;
    pub use crate::models::camera::*;
}

use crate::models::prelude::*;



pub fn point(x: f64, y: f64, z: f64) -> Vector {
    return Vector::new(x, y, z, 1.0);
}

pub fn vector(x: f64, y: f64, z: f64) -> Vector {
    return Vector::new(x, y, z, 0.0);
}


pub fn dot(a:Vector,b:Vector) -> f64 {

    a.x*b.x + a.y*b.y + a.z*b.z + a.w*b.w 
}

pub fn cross(a:Vector, b:Vector) -> Vector {

    vector(a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x, )
}







#[cfg(test)]
mod tests;
