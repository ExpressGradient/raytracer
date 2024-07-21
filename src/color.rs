use crate::{ray::Ray, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(color: Color) -> String {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    format!("{} {} {} ", rbyte, gbyte, bbyte)
}

pub fn ray_color(ray: Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(ray.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;

    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.8, 0.7, 0.5) * a
}
