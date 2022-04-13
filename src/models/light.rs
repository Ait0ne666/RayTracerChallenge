use super::*;




#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Light {  
    pub intensity: Color,
    pub position: Vector
}




impl Light {

    pub fn point_light(point: Vector, color: Color) -> Self {


        Light {intensity: color, position: point}

    }


}



 

pub fn lighting(material: Material, light:Light, point:Vector, eyev:Vector, normalv:Vector, shadow: bool) -> Color {
    // # combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;
    // # find the direction to the light source
    let lightv = (light.position - point).normalize();
    // # compute the ambient contribution
    let ambient = effective_color * material.ambient;
    // # light_dot_normal represents the cosine of the angle between the
    // # light vector and the normal vector. A negative number means the
    // # light is on the other side of the surface.
    let light_dot_normal = dot(lightv, normalv);
    
    let black = Color::new(0.0, 0.0, 0.0);
    let diffuse:Color;
    let specular:Color;
    
    if light_dot_normal < 0.0 {
        diffuse = black;
        specular = black;
    }    else {
        // # compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // # reflect_dot_eye represents the cosine of the angle between the
        // # reflection vector and the eye vector. A negative number means the
        // # light reflects away from the eye.
        let reflectv = (-lightv).reflect(normalv);
        let reflect_dot_eye = dot(reflectv, eyev);
        if reflect_dot_eye <= 0.0 {
            specular = black
    
        }    else {
            // # compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor
    
        }
    }

    if shadow {
        return  ambient;
    }
    // # Add the three contributions together to get the final shading
    return ambient + diffuse + specular

}