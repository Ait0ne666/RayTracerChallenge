use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix<4, 4>,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64) / (vsize as f64);
        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::<4, 4>::identity(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn transform(&mut self, transform: Matrix<4, 4>) {
        self.transform = transform;
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        
        //  # the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        
        //  # the untransformed coordinates of the pixel in world space.
        //  # (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        //  # using the self matrix, transform the canvas point and the origin,
        //  # and then compute the ray's direction vector.
        //  # (remember that the canvas is at z=-1)
        let pixel = self.transform.inverse().unwrap() * point(world_x, world_y, -1.0);
        let origin = self.transform.inverse().unwrap() * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }


    pub fn render(&self, w: World) -> Canvas {

        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let ray = self.ray_for_pixel( x, y);
                let color = w.color_at(ray);
                image.write_pixel(x, y, color);

            }

        }
        
        
        image

    }
}
