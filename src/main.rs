mod color;
mod ray;
mod vec3;

use std::fs::File;
use std::io::{Result, Write};

use color::{ray_color, write_color};
use ray::Ray;
use vec3::Vec3;

fn main() -> Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as i32;
    let max_colors = 255;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width / height) as f64;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Viewport Vectors
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);

    // Pixel delta Vectors
    let pixel_delta_v = viewport_v / height as f64;
    let pixel_delta_u = viewport_u / width as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    let mut file = File::create("./assets/image.ppm")?;

    // Header
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", width, height).as_bytes())?;
    file.write_all(format!("{}\n", max_colors).as_bytes())?;

    for h in (0..height).rev() {
        for w in 0..width {
            let pixel_center =
                pixel_00_loc + (pixel_delta_u * w as f64) + (pixel_delta_v * h as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(ray);
            file.write_all(write_color(pixel_color).as_bytes())?;
        }
        file.write_all(b"\n")?;
    }

    Ok(())
}
