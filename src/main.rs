mod fractal;

use fractal::Fractal;
// use std::env;

fn main() {

    let mut mandelbrot = Fractal::new(1920, 1080, 100);

    mandelbrot.iterate();
}
