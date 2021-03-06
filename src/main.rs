extern crate image;
extern crate num_complex;
extern crate rayon;

use num_complex::Complex;
use rayon::prelude::*;

fn main() {
    let max_iterations = 1000;
    let range_x = (-2.1, 0.5); // (-2.1, 0.5);
    let range_y = (-1.0, 1.0); // (-1.0, 1.0);
    let width = 4000;
    let height = 4000;
    let scale_x = (range_x.1 - range_x.0) / width as f32;
    let scale_y = (range_y.1 - range_y.0) / height as f32;

    let mut pixels = vec![0u8; (width * height) as usize];

    pixels.par_iter_mut().enumerate().for_each(|(idx, pixel)| {
        // Translate 1D array position to a (x, y) coordinate in a 2D array.
        let x = idx % width;
        let y = idx / height;

        // Translate the pixel position to a point in the mandelbrot set.
        let cx = range_x.0 + x as f32 * scale_x;
        let cy = range_y.0 + y as f32 * scale_y;

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0.0, 0.0);

        // Calculate z^2 + c.
        let mut iter = 0;
        while iter < max_iterations {
            if z.norm_sqr() > 4.0 {
                break;
            }
            z = z * z + c;
            iter += 1;
        }

        *pixel = -iter as u8;
    });

    let fout = std::fs::File::create("fractal.png").expect("Could not create outputfile");
    let encoder = image::png::PNGEncoder::new(fout);
    encoder
        .encode(
            &pixels,
            width as u32,
            height as u32,
            image::ColorType::Gray(8),
        ).expect("Failed to encoede image");
}
