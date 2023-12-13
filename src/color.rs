use crate::vector::Vector3D;

pub type Color = Vector3D;

pub fn write_color(pixel_color: &Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    );
}
