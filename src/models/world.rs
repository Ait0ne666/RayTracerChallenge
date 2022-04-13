use std::cmp::Ordering;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub light_source: Light,
    pub objects: Vec<Object>,
}

impl World {
    pub fn default() -> Self {
        let mut sphere1 = Sphere::new();
        let material1 = Material::new(
            Some(Color::new(0.8, 1.0, 0.6)),
            None,
            Some(0.7),
            Some(0.2),
            None,
        );
        sphere1.setMaterial(material1);

        let mut sphere2 = Sphere::new();
        sphere2.setTransform(scale(0.6, 0.5, 0.5));

        let light = Light::point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

        World {
            light_source: light,
            objects: vec![Object::from(sphere1), Object::from(sphere2)],
        }
    }

    pub fn shade(&self, comps: Computation) -> Color {
        let shadowed = self.is_shadowed(comps.point);



        // println!("{}", shadowed);

        lighting(
            comps.object.material(),
            self.light_source,
            comps.point,
            comps.eye,
            comps.normal,
            shadowed
        )
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect(ray);
        let hit = hit(intersections);

        match hit {
            Some(inter) => {
                let compute = inter.compute(ray);

                self.shade(compute)
            }
            None => Color::black(),
        }
    }


    pub fn is_shadowed(&self, point: Vector) -> bool {

        let v = self.light_source.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);
        let h = hit(intersections);

        match h {
            Some(inter) => {
                if (inter.t < distance) {
                    return true
                }
                return false
            },
            None => {
                return false
            }
        }

    
    }
}

impl Intersect for World {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];

        for object in self.objects.iter() {
            let mut inter = object.intersect(ray);
            intersections.append(&mut inter)
        }

        intersections.sort_by(|a, b| -> Ordering {
            if a.t > b.t {
                return Ordering::Greater;
            }

            if (b.t > a.t) {
                return Ordering::Less;
            }

            return Ordering::Equal;
        });

        intersections
    }
}

pub fn view_transform(from: Vector, to: Vector, up: Vector) -> Matrix<4, 4> {
    let forward = (to - from).normalize();
    let left = cross(forward, up.normalize());
    let true_up = cross(left, forward);
    let orientation = Matrix {
        elements: [
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    orientation*translation(-from.x, -from.y, -from.z)
}
