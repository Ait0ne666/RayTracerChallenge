
use super::*;
use std::fs::File;
use std::io::prelude::*;

use super::prelude::Color;

#[derive(Debug,Clone, Copy, PartialEq, PartialOrd)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Color
}


impl Pixel {
    pub fn new(x: usize, y:usize, color:Color) -> Self {
        Pixel{x:x, y:y, color: color}
    }
}


#[derive(Debug,Clone,  PartialEq, PartialOrd)]
pub struct Canvas {
    pub width: usize,
    pub heigh: usize,
    
    pub pixels: Vec<Vec<Pixel>>
}




impl Canvas {



    pub fn new(width: usize, height: usize) -> Self {


        let mut pixels: Vec<Vec<Pixel>> = vec![];


        for y in 0..width {
            let mut row:Vec<Pixel> = vec![];
            for x in 0..height {
                row.push( Pixel::new(x, y, color(0.0,0.0,0.0)))
            }

            pixels.push(row)
        }


        Canvas { width: width, heigh: height, pixels }
    }


    pub fn write_pixel(&mut self, x:usize, y:usize, color: Color) {
        if x<= self.width-1 && y<= self.heigh -1 {

            self.pixels[x][y] = Pixel::new(x, y, color)       
        }

    }


    pub fn pixel_at(self, x: usize, y: usize) -> Pixel {
        self.pixels[x][y]
    }


    pub fn to_ppm(self) -> Vec<String> {
        let mut result: Vec<String> = vec!["P3".to_string(), self.width.to_string() + " " +  &self.heigh.to_string(), "255".to_string() ];
        

        let mut row = "".to_string();


        let mut pixels: Vec<Pixel> = vec![];
        
        
        for r in 0..self.pixels[0].len() {
            for p in 0..self.pixels.len() {
                pixels.push(self.pixels[p][r])
            }
        }

        // let pixels: Vec<Pixel> = self.pixels.iter().cloned().flatten().collect::<Vec<Pixel>>();


        
       

        for pixel in pixels {

            let mut red: usize = (pixel.color.red * 255.0).floor() as usize; 

            if red > 255 {
                red = 255;
            } 

            if red < 0 {
                red = 0;
            }


            let mut green: usize = (pixel.color.green * 255.0).floor() as usize; 

            if green > 255 {
                green = 255;
            } 

            if green < 0 {
                green = 0;
            }


            let mut blue: usize = (pixel.color.blue * 255.0).floor() as usize; 

            if blue > 255 {
                blue = 255;
            } 

            if blue < 0 {
                blue = 0;
            }




            if row.len() > 66 {
                result.push(row);
                row = "".to_string() 
            }

            row = row + &red.to_string() + " ";

            if row.len() > 67 {
                result.push(row);
                row = "".to_string()
            }

            row = row + &green.to_string() + " ";

            if row.len() > 67 {
                result.push(row);
                row = "".to_string()
            }

            row = row + &blue.to_string() + " ";

        }


        if (row != "".to_string()) {
            result.push(row)
        }

        let data = result.join("\n");


        let mut file = File::create("canvas.ppm").expect("an error creating file");


        file.write_all(data.as_bytes()).expect("an error writing to  file");
        
        


        result
    }
}