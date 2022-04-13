use super::*;

pub trait Transform {
    
    fn transform(self, transform: Matrix<4,4>) -> Self;

}



pub trait Normal {

    fn normal_at(&self, point: Vector) -> Vector;
}


