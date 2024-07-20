use crate::vec3::Vec3;

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
