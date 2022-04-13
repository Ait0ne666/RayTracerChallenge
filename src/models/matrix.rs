

use crate::prelude::equal_floats;

use super::*;
use std::ops;



#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Matrix<const R: usize, const C: usize> {
    pub elements: [[f64; C]; R],
    
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new() -> Self {
        Matrix {
            elements: [[0.0; C]; R],
            
        }
    }


    pub fn approx_equal(self, _rhs: Matrix<R,C>) -> bool {

        

        for row in 0..self.elements.len() {
            for col in 0..self.elements[row].len() {
                if !equal_floats(self.elements[row][col], _rhs.elements[row][col]) {
                    return  false;
                }
            }
        }

        return true;


    }

    pub fn setElement(&mut self, r: usize, c: usize, element: f64) {
        self.elements[r][c] = element
    }

    pub fn identity() -> Self {


        let mut result = Matrix {
            elements: [[0.0; C]; R],
        };


        for row in 0..result.elements.len() {
            for col in 0..result.elements[row].len() {
                if col == row {
                    result.setElement(row, col, 1.0)
                }
            }
        }


        result

    }


    pub fn transpose(self) -> Matrix<C,R> {
        
        let mut result = Matrix::<C,R>::new();

        for row in 0..self.elements.len() {
            for col in 0..self.elements[row].len() {
                
                result.setElement(col, row, self.elements[row][col])
            }
        }


        result
    }





   

}



impl Matrix<2,2> {
        pub fn determinant(self) -> f64 {


        return self.elements[0][0] * self.elements[1][1] - self.elements[1][0]* self.elements[0][1]
    }
}



impl Matrix<3,3> {


    pub fn submatrix(self, row: usize, col:usize) -> Matrix<2,2> {

        let mut result = Matrix::<2,2>::new();


        for r in 0..self.elements.len() {
            

            for c in 0..self.elements[row].len() {
                if r != row && c!=col {
                    let mut newRow = r;
                    let mut newCol = c;
                    
                    
                    if newRow > row {
                        newRow = newRow - 1
                    }


                    if newCol > col {
                        newCol = newCol - 1
                    }

                    result.setElement(newRow, newCol, self.elements[r][c])
                }
            }
        }


        return  result;


    }


    pub fn minor(self, row: usize, col:usize) -> f64 {

        return self.submatrix(row, col).determinant()
    }



    pub fn cofactor(self, row: usize, col:usize) -> f64 {
        if (row + col) % 2 == 0 {

            return self.minor(row, col)
        } 


        return -self.minor(row, col)



    }

    pub fn determinant(self) -> f64 {


        let mut determinant = 0.0;


        for col in 0..self.elements[0].len() {
            determinant = determinant + self.elements[0][col] * self.cofactor(0, col);
        }


        determinant

    }


    pub fn isInvertible(self) -> bool {


        return self.determinant() != 0.0
    }



    pub fn inverse(self) -> Option<Self> {

        if !self.isInvertible() {
            None
        } else {
            let mut inverted = Self::new();


            let determinant = self.determinant();

            for row in 0.. self.elements.len() {
                for col in 0..self.elements[row].len() {

                    let cofactor = self.cofactor(row, col);


                    inverted.setElement(col, row, cofactor/determinant)

                }
            }

            Some(inverted)
        }
    }

}



impl Matrix<4,4> {


    pub fn submatrix(self, row: usize, col:usize) -> Matrix<3,3> {

        let mut result = Matrix::<3,3>::new();


        for r in 0..self.elements.len() {
            

            for c in 0..self.elements[row].len() {
                if r != row && c!=col {
                    let mut newRow = r;
                    let mut newCol = c;
                    
                    
                    if newRow > row {
                        newRow = newRow - 1
                    }


                    if newCol > col {
                        newCol = newCol - 1
                    }

                    result.setElement(newRow, newCol, self.elements[r][c])
                }
            }
        }


        return  result;


    }


    pub fn minor(self, row: usize, col:usize) -> f64 {

        return self.submatrix(row, col).determinant()
    }



    pub fn cofactor(self, row: usize, col:usize) -> f64 {
        if (row + col) % 2 == 0 {

            return self.minor(row, col)
        } 


        return -self.minor(row, col)



    }

    pub fn determinant(self) -> f64 {


        let mut determinant = 0.0;


        for col in 0..self.elements[0].len() {
            determinant = determinant + self.elements[0][col] * self.cofactor(0, col);
        }


        determinant

    }


    pub fn isInvertible(self) -> bool {


        return self.determinant() != 0.0
    }



    pub fn inverse(self) -> Option<Self> {

        if !self.isInvertible() {
            None
        } else {
            let mut inverted = Self::new();


            let determinant = self.determinant();

            for row in 0.. self.elements.len() {
                for col in 0..self.elements[row].len() {

                    let cofactor = self.cofactor(row, col);


                    inverted.setElement(col, row, cofactor/determinant)

                }
            }

            Some(inverted)
        }
    }

}

pub fn matrix<const R: usize, const C: usize>(elements: [[f64; C]; R]) -> Matrix<R, C> {
    Matrix::<R, C> { elements: elements }
}


pub fn translation(x:f64,y:f64,z:f64) -> Matrix<4,4> {
    let mut matrix = Matrix::<4,4>::identity();

    matrix.setElement(0, 3, x);
    matrix.setElement(1, 3, y);
    matrix.setElement(2, 3, z);

    matrix
}


pub fn scale(x:f64,y:f64,z:f64) -> Matrix<4,4> {
    let mut matrix = Matrix::<4,4>::identity();

    matrix.setElement(0, 0, x);
    matrix.setElement(1, 1, y);
    matrix.setElement(2, 2, z);

    matrix
}


pub fn rotateX(deg: f64) -> Matrix<4,4> {
    let mut matrix = Matrix::<4,4>::identity();

    matrix.setElement(1, 1, deg.cos());
    matrix.setElement(2, 2, deg.cos());
    matrix.setElement(1, 2, -deg.sin());
    matrix.setElement(2, 1, deg.sin());
    matrix
}


pub fn rotateY(deg: f64) -> Matrix<4,4> {
    let mut matrix = Matrix::<4,4>::identity();
    matrix.setElement(0, 0, deg.cos());
    matrix.setElement(2, 2, deg.cos());
    matrix.setElement(0, 2, deg.sin());
    matrix.setElement(2, 0, -deg.sin());
    matrix
}


pub fn rotateZ(deg: f64) -> Matrix<4,4> {
    let mut matrix = Matrix::<4,4>::identity();

    matrix.setElement(1, 1, deg.cos());
    matrix.setElement(0, 0, deg.cos());
    matrix.setElement(0, 1, -deg.sin());
    matrix.setElement(1, 0, deg.sin());
    matrix
}



pub fn shearing(xy:f64, xz:f64, yx:f64, yz:f64, zx: f64, zy: f64) -> Matrix<4,4> {
    let mut matrix = Matrix::<4,4>::identity();

    matrix.setElement(0, 1, xy);
    matrix.setElement(0, 2, xz);
    matrix.setElement(1, 0, yx);
    matrix.setElement(1, 2, yz);
    matrix.setElement(2, 0, zx);
    matrix.setElement(2, 1, zy);



    matrix
}

impl<const R: usize, const C: usize, const X:usize, const Y:usize> ops::Mul<Matrix<X, Y>> for Matrix<R, C> {
    type Output = Matrix<X, Y>;

    fn mul(self, _rhs: Matrix<X, Y>) -> Matrix<X, Y> {
        let mut result = Matrix::<X, Y>::new();

        for row in 0..result.elements.len() {
            for col in 0..result.elements[row].len() {
                let mut cell = 0.0;

                for i in 0..result.elements.len() {
                    cell = cell + self.elements[row][i] * _rhs.elements[i][col]
                }

                result.setElement(row, col, cell)
            }
        }
       
        result
    }
}

impl<const R: usize, const C: usize> ops::Mul<Vector> for Matrix<R, C> {
    type Output = Vector;

    fn mul(self, _rhs: Vector) -> Vector {
        let res = matrix([[_rhs.x], [_rhs.y], [_rhs.z], [_rhs.w]]);


        let mult = self * res;


        Vector::new(mult.elements[0][0], mult.elements[1][0], mult.elements[2][0], mult.elements[3][0])
    }
}




