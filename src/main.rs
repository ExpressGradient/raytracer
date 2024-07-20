mod color;
mod vec3;

use std::fs::File;
use std::io::{Result, Write};

use color::write_color;
use vec3::Vec3;

fn main() -> Result<()> {
    // Config
    let width = 256;
    let height = 256;
    let max_colors = 255;

    let mut file = File::create("./assets/image.ppm")?;

    // Header
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", width, height).as_bytes())?;
    file.write_all(format!("{}\n", max_colors).as_bytes())?;

    for h in (0..height).rev() {
        for w in 0..width {
            let r = w as f64 / (width - 1) as f64;
            let g = 0.0;
            let b = h as f64 / (height - 1) as f64;

            let pixel_color = Vec3::new(r, g, b);
            file.write_all(write_color(pixel_color).as_bytes())?;
        }
        file.write_all(b"\n")?;
    }

    Ok(())
}
