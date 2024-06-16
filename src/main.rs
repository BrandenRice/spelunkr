extern crate hsv;
extern crate num_complex;

use hsv::hsv_to_rgb;
use num_complex::Complex;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let max_iterations = 256u32;
    let img_side = 1000u32;
    let cxmin = -2f32;
    let cxmax = 1f32;
    let cymin = -1.5f32;
    let cymax = 1.5f32;
    let scalex = (cxmax - cxmin) / img_side as f32;
    let scaley = (cymax - cymin) / img_side as f32;

    // Create a buffer
    let mut buffer: Vec<u32> = vec![0; (img_side * img_side).try_into().unwrap()];
    
    let mut window = Window::new(
        "Spelunkr",
        img_side.try_into().unwrap(),
        img_side.try_into().unwrap(),
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    }); 

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_image(&mut buffer, cxmin, cxmax, cymin, cymax, scalex, scaley, img_side, max_iterations);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, img_side.try_into().unwrap(), img_side.try_into().unwrap())
            .unwrap();
    }
}


fn update_image(buffer: &mut [u32], cxmin: f32, cxmax: f32, cymin :f32, cymax :f32, scalex :f32, scaley :f32, img_side: u32, max_iterations: u32) {
    // Calculate for each pixel
    for (i, pixel) in buffer.iter_mut().enumerate() {
        let cx = cxmin + ((i as u32 % img_side) as f32 * scalex);
        let cy = cymin + ((i as u32 / img_side) as f32 * scaley);

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);

        let mut i = 0;
        while i < max_iterations {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + c;
            i += 1;
        }

        *pixel = colour(i, max_iterations);
    }
}


fn colour(iterations: u32, max_iterations: u32) -> u32 {
    let rgb = hsv_to_rgb((360 * iterations / max_iterations) as f64, 1.0, (1 - iterations / max_iterations).into());

    return u32::from_be_bytes([0, rgb.0, rgb.1, rgb.2])
}
