use std::io;
use std::io::Write;
use halide::color;
use halide::color::Color;

fn main() {
    let image_width: usize = 256;
    let image_height: usize = 256;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprint!("\rLines Remaining: {}", image_height - j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0
            );

            color::write_color(&pixel_color);
        }
    }

    eprintln!("\n\rDone.")
}
