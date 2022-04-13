use super::*;



#[derive(Clone, Copy, Debug)]
pub struct Environment {

    pub wind: Vector,
    pub gravity: Vector

}



impl Environment {
    

    pub fn new(wind: Vector, gravity: Vector) -> Self {


        Environment{wind: wind, gravity: gravity}
    }
}