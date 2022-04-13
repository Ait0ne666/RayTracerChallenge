use super::*;


pub struct Computation {
    pub point: Vector,
    pub eye: Vector,
    pub normal: Vector,
    pub object: Object,
    pub t: f64,
    pub inside: bool
}



#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub object: Object,
    pub t: f64,
   
}







impl Intersection {


    pub fn new(object: Object, t: f64) -> Self {
        

        Intersection { object: object, t: t}
    }


    pub fn compute(self, ray: Ray) -> Computation {
        let point  = ray.position(self.t);
        let eyev  = -ray.direction;
        let mut normalv = self.object.normal_at(point);



        let dot_product = dot(normalv, eyev);
        let mut inside = false;
        if (dot_product < 0.0) {
            inside = true;
            normalv = -normalv;
        }

        Computation { point, eye: eyev, normal: normalv, object: self.object, t: self.t, inside }
    }
}



pub fn hit(intersections: Vec<Intersection> )  -> Option<Intersection>   {

    let mut min_t = f64::MAX;
    let mut result = -1.0;
    let mut inter: Intersection = Intersection { object: Object::from(Sphere::new()), t: 0.0 };


    for intersection in intersections.iter() {
       
        if intersection.t < min_t && intersection.t >= 0.0 {
            min_t = intersection.t;
            result = intersection.t;
            inter = *intersection;
        }
    }

    if result > -1.0 {
        return  Some(inter);
    }

    return None
}