use crate::vec3::Vec3;

pub struct Light {
    pub position: Vec3,
    pub brightness: f64,
    pub size: f64,
}

impl Light {
    pub fn new(position: Vec3, brightness: f64) -> Light {
        Light {
            position,
            brightness,
            size: 1.5,
        }
    }
}
