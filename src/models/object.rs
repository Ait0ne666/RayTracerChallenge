use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}


impl Object {
    pub fn material(self) -> Material {
        match self {
            Self::Sphere(ref c) => {
                c.material
            }
        }
    }


    pub fn set_ambient(&mut self, ambient: f64) {
        match self {
            Self::Sphere(ref mut c) => {
                c.material.ambient = ambient;
            }
        }
    }


}

impl From<Sphere> for Object {
    fn from(sphere: Sphere) -> Self {
        Object::Sphere(sphere)
    }
}


impl Intersect for Object {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        match self {
            Object::Sphere(c) => {
                c.intersect(ray)
            }
        }
    }

}



impl Normal for Object {
    fn normal_at(&self, world_point: Vector) -> Vector {
        

        match *self {
            Object::Sphere(ref c) => {

                let point = c.transform.inverse().unwrap() * world_point;
        
                let object_normal = point - c.origin;
        
                let mut world_normal = c.transform.transpose().inverse().unwrap() * object_normal ;
                
                world_normal.w = 0.0;
        
                world_normal.normalize()
            }
        }

    }
}