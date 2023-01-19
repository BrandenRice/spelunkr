use std::ops::{self};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub r: f64,
    pub i: f64
}

// impl Complex {
//     pub fn new (real: f64, imaginary: f64) -> Self {
//         Self { r: real, i: imaginary }
//     }
// }

impl ops::Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex {
            r: (self.r * rhs.r) - (self.i * rhs.i),
            i: (self.r * rhs.i) + (self.i * rhs.r)
        }
    }
}

impl ops::MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, rhs: Complex) {
        *self = *self * rhs
    }
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex { 
            r: self.r + rhs.r,
            i: self.i + rhs.i 
         }
    }
}