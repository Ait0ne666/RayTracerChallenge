use super::*;
use crate::prelude::*;










pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Vec<Intersection>; 
}




#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sphere {
    pub origin: Vector,
    pub radius: f64,
    pub transform: Matrix<4,4>,
    pub material: Material
}



impl Sphere {

    pub fn new() -> Self {
        Sphere { origin: point(0.0,0.0,0.0), radius: 1.0, transform: Matrix::<4,4>::identity(), material: Material::default()}
    }


    pub fn setRadius(&mut self, radius: f64) {
        self.radius = radius;
    }


    pub fn setTransform(&mut self, transform: Matrix<4,4>) {
        self.transform = transform
    }


    pub fn setMaterial(&mut self, material: Material) {
        self.material = material;
    }

}


impl Intersect for Sphere {
    fn intersect(&self, pre_transformed_ray: Ray) -> Vec<Intersection> {

            let ray = pre_transformed_ray.transform(self.transform.inverse().unwrap());

            let sphere_to_ray = ray.origin - self.origin;


            let a = dot(ray.direction, ray.direction);
            let b = 2.0 * dot(ray.direction, sphere_to_ray);
            let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;


            let discriminant = b.powi(2) - 4.0 * a * c;



            if discriminant < 0.0 {
                return vec![]
            } 


            let t1 = (-b - discriminant.sqrt())/ (2.0 * a);
            let t2 = (-b + discriminant.sqrt())/ (2.0 * a);
            
            
            return  vec![Intersection::new(Object::from(*self), t1), Intersection::new(Object::from(*self), t2)];

    } 
}




impl Normal for Sphere {
    fn normal_at(&self, world_point: Vector) -> Vector {
        
        let point = self.transform.inverse().unwrap() * world_point;

        let object_normal = point - self.origin;

        let mut world_normal = self.transform.transpose().inverse().unwrap() * object_normal ;
        
        world_normal.w = 0.0;

        world_normal.normalize()
    }
}