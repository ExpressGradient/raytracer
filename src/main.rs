use std::fs::File;
use std::io::{Result, Write};

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
            let r = (w as f32 / (width - 1) as f32) * 255.0;
            let g = 0;
            let b = (h as f32 / (height - 1) as f32) * 255.0;

            file.write_all(format!("{} {} {} ", r, g, b).as_bytes())?;
        }
        file.write_all(b"\n")?;
    }

    Ok(())
}
