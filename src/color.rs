use crate::interval::Interval;
use crate::vector::Vector3D;

pub type Color = Vector3D;

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn write_color(pixel_color: &Color, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f64;

    let red = linear_to_gamma(pixel_color.x() * scale);
    let green = linear_to_gamma(pixel_color.y() * scale);
    let blue = linear_to_gamma(pixel_color.z() * scale);

    let intensity = Interval::new(0.000, 0.999);

    println!(
        "{} {} {}",
        (256.0 * intensity.clamp(red)) as i32,
        (256.0 * intensity.clamp(green)) as i32,
        (256.0 * intensity.clamp(blue)) as i32
    );
}
