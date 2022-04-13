use super::*;




#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn new(color: Option<Color>, ambient: Option<f64>, diffuse: Option<f64>, specular: Option<f64>, shininess: Option<f64>) -> Self {

        let mut material = Material::default();

        match color {
            Some(value) => {
                material.color = value
            }
            None => {},
        }


        match ambient {
            Some(value) => {
                material.ambient = value
            }
            None => {},
        }

        match diffuse {
            Some(value) => {
                material.diffuse = value
            }
            None => {},
        }

        match specular {
            Some(value) => {
                material.specular = value
            }
            None => {},
        }

        match shininess {
            Some(value) => {
                material.shininess = value
            }
            None => {},
        }


        material
    }
}
