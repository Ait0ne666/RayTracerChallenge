use crate::prelude::*;

pub fn equal_floats(a:f64, b: f64) -> bool {
 
    if (a-b).abs() < EPSILON {
        return true
    };


    return false


}





#[cfg(test)]
mod tests;