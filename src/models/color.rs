
use crate::prelude::equal_floats;


use std::ops;




#[derive(Debug, Clone, Copy , PartialEq, PartialOrd)]
pub struct Color {
    pub red: f64,
    pub green: f64, 
    pub blue: f64
}



impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color{red: red, green: green, blue: blue}
    }


    pub fn appr_equal(self, color:Color) ->bool {

        equal_floats(self.red, color.red) && equal_floats(self.blue, color.blue) && equal_floats(self.green, color.green) 
    }


    pub fn black() -> Self {
        Color {red: 0.0,green:0.0, blue:0.0}
    }
}


pub fn color(red: f64, green: f64, blue: f64) -> Color {

    Color::new(red, green, blue)
}



impl ops::Add<Color> for Color {
    type Output = Color;


    fn add(self, _rhs: Color) -> Color {

        Color::new(self.red+_rhs.red, self.green+_rhs.green, self.blue+_rhs.blue)
    }
}


impl ops::Sub<Color> for Color {
    type Output = Color;


    fn sub(self, _rhs: Color) -> Color {

        Color::new(self.red-_rhs.red, self.green-_rhs.green, self.blue-_rhs.blue)
    }
}



impl ops::Mul<f64> for Color {
    type Output = Color;


    fn mul(self, _rhs: f64) -> Color {

        Color::new(self.red*_rhs, self.green*_rhs, self.blue*_rhs)
    }
}


impl ops::Mul<Color> for Color {
    type Output = Color;


    fn mul(self, _rhs: Color) -> Color {

        Color::new(self.red*_rhs.red, self.green*_rhs.green, self.blue*_rhs.blue)
    }
}