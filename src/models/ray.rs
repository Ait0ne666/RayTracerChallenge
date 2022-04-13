use super::*;
use crate::prelude::*;



#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}



impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }


    pub fn position(self, time: f64) -> Vector {


        self.origin + self.direction * time
    }   
}



impl Transform for Ray {

    fn transform(self, transform: Matrix<4,4>) -> Self {
        let new_origin = transform * self.origin;
        let new_direction = transform * self.direction;


        return Ray::new(new_origin, new_direction)
    }
    

}




