mod complex;
use complex::Complex;

const STARTING_RANGE_X: (f64, f64) = (-2.0, 0.47);
const STARTING_RANGE_Y: (f64, f64) = (-1.12, 1.12);

#[derive(Debug)]
pub struct Fractal {
    iterations: u32,
    size_x: usize,
    size_y: usize,
    range_x: (f64, f64),
    range_y: (f64, f64),
    points: Vec<Complex>
}

impl Fractal {
    pub fn new (size_x: usize, size_y: usize, iterations: u32) -> Self {
        Self { 
            iterations, 
            size_x, 
            size_y,
            range_x: STARTING_RANGE_X,
            range_y: STARTING_RANGE_Y,
            points: Vec::with_capacity(size_x * size_y)
        }
    }

    fn iterate_point(iterations: u32, point: &mut Complex) {                                                                                 
        let mut p = *point;

        for i in 0..iterations {
            p *= p;

            if false { break; } 
        }
    }

    pub fn iterate(&mut self) {
        for point in &mut self.points {
            Fractal::iterate_point(self.iterations, point);
        }
    }
}
