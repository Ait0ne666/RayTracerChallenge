use crate::prelude::*;
use std::ops;

#[derive(Copy, Debug, PartialEq, PartialOrd, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}


impl Vector {
    pub fn new(x:f64, y:f64, z: f64, w: f64) -> Self {
        Self{x, y, z, w}
    }


    pub fn magnitude(self) -> f64 {
        let magnitude: f64 = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) +self.w.powi(2)).sqrt();

        return magnitude;
    }


    pub fn normalize(self) -> Vector {


        Vector::new(self.x/self.magnitude(), self.y/self.magnitude(),self.z/self.magnitude(),self.w/self.magnitude())

    }


    pub fn appr_equal(self, vector:Vector) ->bool {

        equal_floats(self.x, vector.x) && equal_floats(self.y, vector.y) && equal_floats(self.z, vector.z) && equal_floats(self.w, vector.w)
    }


    pub fn reflect(&self, normal: Self) -> Self {

        *self - normal * 2.0 * dot(*self, normal)


    } 
}




impl ops::Add<Vector> for Vector {
    type Output = Vector;


    fn add(self, _rhs: Vector) -> Vector {

        Vector::new(self.x+_rhs.x, self.y+_rhs.y, self.z+_rhs.z, self.w+_rhs.w)
    }
}


impl ops::Sub<Vector> for Vector {
    type Output = Vector;


    fn sub(self, _rhs: Vector) -> Vector {

        Vector::new(self.x-_rhs.x, self.y-_rhs.y, self.z-_rhs.z, self.w-_rhs.w)
    }
}


impl ops::Neg for Vector {
    type Output = Self;


    fn neg(self) -> Self {

        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}


impl ops::Mul<f64> for Vector {
    type Output = Vector;


    fn mul(self, _rhs: f64) -> Vector {

        Vector::new(self.x*_rhs, self.y*_rhs, self.z*_rhs, self.w*_rhs)
    }
}


impl ops::Div<f64> for Vector {
    type Output = Vector;


    fn div(self, _rhs: f64) -> Vector {

        Vector::new(self.x/_rhs, self.y/_rhs, self.z/_rhs, self.w/_rhs)
    }
}


impl ops::Mul<Vector> for f64 {
    type Output = Vector;


    fn mul(self, _rhs: Vector) -> Vector {

        Vector::new(self*_rhs.x, self*_rhs.y, self*_rhs.z, self*_rhs.w)
    }
}

