use image::{ImageBuffer, Rgb};
use nalgebra::{Complex, Normed};
use rand::Rng;

fn julia(c: Complex<f64>, x: f64, y: f64) -> u8 {
    let mut z = Complex::new(x, y);

    for i in 20..255 {
        if z.norm_squared() > 2 as f64 {
            return i as u8;
        }
        z = z * z + c;
    }

    255
}

fn mandelbrot(x: f64, y: f64) -> u8 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..255 {
        if z.norm_squared() > 4.0 {
            return i as u8;
        }
        z = z * z + c;
    }

    255
}

fn tricorn(x: f64, y: f64) -> u8 {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..255 {
        if z.norm_squared() > 3.0 {
            return i as u8;
        }
        z = Complex::new(z.re, -z.im);
        z = z * z + c;
    }

    255
}

fn main() {
    let width = 800;
    let height = 800;

    let mut img = ImageBuffer::new(width, height);

    let c = Complex::new(-0.4, 0.6);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x = x as f64 / width as f64 * 3.1 - 1.5;
        let y = y as f64 / height as f64 * 2.9 - 1.5;

        let intensity = julia(c, x, y);
        // let intensity = mandelbrot(x, y);
        // let intensity = tricorn(x, y);
        // let red = rand::thread_rng().gen_range(10..20);
        // let green = intensity;
        // // let green = (intensity as f64 * 0.8) as u8;
        // let blue = intensity;
        // // let blue = (intensity as f64 * 0.6) as u8;

        let red = (intensity as f64 * 0.1) as u8;
        let green = (intensity as f64 * 0.8) as u8;
        let blue = (intensity as f64 * 0.9) as u8;

        *pixel = Rgb([red as u8, green as u8, blue]);
    }

    img.save("julia.png").unwrap();
}
