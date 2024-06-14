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

fn main() {
    let width = 800;
    let height = 800;

    let mut img = ImageBuffer::new(width, height);

    let c = Complex::new(-0.4, 0.6);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let x = x as f64 / width as f64 * 3.1 - 1.5;
        let y = y as f64 / height as f64 * 2.9 - 1.5;

        let intensity = julia(c, x, y);
        let red = rand::thread_rng().gen_range(1..10);
        let green = intensity;
        let blue = intensity;

        *pixel = Rgb([red as u8, green as u8, blue]);
    }

    img.save("julia.png").unwrap();
}
