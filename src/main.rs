use std::f64::consts::PI;
use std::time::Instant;

use models::point;
use models::prelude::hit;
use models::prelude::lighting;
use models::prelude::rotateY;
use models::prelude::rotateZ;
use models::prelude::scale;
use models::prelude::shearing;
use models::prelude::translation;
use models::prelude::Intersect;
use models::prelude::Light;
use models::prelude::Normal;
use models::prelude::Ray;
use models::prelude::Sphere;
use models::vector;
use playground::init;
use playground::tick;
use prelude::prelude::color;
use prelude::prelude::rotateX;
use prelude::prelude::view_transform;
use prelude::prelude::Camera;
use prelude::prelude::Canvas;
use prelude::prelude::Color;
use prelude::prelude::Material;
use prelude::prelude::Object;
use prelude::prelude::World;

mod constants;
mod models;
mod playground;
mod utils;

mod prelude {
    pub use crate::constants::*;
    pub use crate::models::*;
    pub use crate::playground::*;
    pub use crate::utils::*;
}

fn main() {

    let start = Instant::now();


    let mut floor = Sphere::new();

    floor.setTransform(scale(10.0, 0.01, 10.0));
    let material = Material::new(Some(color(1.0, 0.9, 0.9)), None, None, Some(0.0), None);
    floor.setMaterial(material);

    let mut left_wall = Sphere::new();
    let transform = translation(0.0, 0.0, 5.0)
        * rotateY(-PI / 4.0)
        * rotateX(PI / 2.0)
        * scale(10.0, 0.01, 10.0);
    left_wall.setTransform(transform);
    left_wall.setMaterial(material);

    let mut right_wall = Sphere::new();
    let transform = translation(0.0, 0.0, 5.0)
        * rotateY(PI / 4.0)
        * rotateX(PI / 2.0)
        * scale(10.0, 0.01, 10.0);
    right_wall.setTransform(transform);
    right_wall.setMaterial(material);

    let mut middle = Sphere::new();
    middle.setTransform(translation(-0.5, 1.0, 0.5));
    let material = Material::new(Some(color(0.1, 1.0, 0.5)), None, Some(0.7), Some(0.3), None);
    middle.setMaterial(material);

    let mut right_sphere = Sphere::new();
    right_sphere.setTransform(translation(1.5, 0.5, -0.5) * scale(0.5, 0.5, 0.5));
    let material = Material::new(Some(color(0.5, 1.0, 0.1)), None, Some(0.7), Some(0.3), None);
    right_sphere.setMaterial(material);

    let mut left_sphere = Sphere::new();
    left_sphere.setTransform(translation(-1.5, 0.33, -0.75) * scale(0.33, 0.33, 0.33));
    let material = Material::new(Some(color(1.0, 0.8, 0.1)), None, Some(0.7), Some(0.3), None);
    left_sphere.setMaterial(material);

    let light_source = Light::point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));

    let world = World {
        light_source,
        objects: vec![
            Object::from(floor),
            Object::from(left_wall),
            Object::from(right_wall),
            Object::from(middle),
            Object::from(left_sphere),
            Object::from(right_sphere),
        ],
    };

    let mut camera = Camera::new(1000, 1000, PI / 3.0);
    camera.transform(view_transform(
        point(0.0, 1.0, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(world);

    let ppm = canvas.to_ppm();


    let duration = start.elapsed();


    println!("the function run for {:?}", duration)
}
