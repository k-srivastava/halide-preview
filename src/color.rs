use crate::interval::Interval;
use crate::vector::Vector3D;

pub type Color = Vector3D;

pub fn write_color(pixel_color: &Color, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f64;

    let red = pixel_color.x() * scale;
    let green = pixel_color.y() * scale;
    let blue = pixel_color.z() * scale;

    let intensity = Interval::new(0.000, 0.999);

    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(red)) as i32,
        (256.0 * intensity.clamp(green)) as i32,
        (256.0 * intensity.clamp(blue)) as i32
    );
}
