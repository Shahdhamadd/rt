use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
    pub reflect: f64,
    pub shine: f64,
}

impl Plane {
    pub fn with_shine(mut self, amount: f64) -> Plane {
        self.shine = amount;
        self
    }

    pub fn with_reflection(mut self, amount: f64) -> Plane {
        self.reflect = amount;
        self
    }

    pub fn new(point: Vec3, normal: Vec3, color: Vec3) -> Plane {
        Plane {
            point,
            normal: normal.unit(),
            color,
            reflect: 0.0,
            shine: 0.0,
        }
    }

    pub fn hit(&self, ray: Ray) -> Option<f64> {
        let facing = self.normal.dot(ray.direction);
        if facing.abs() < 0.000001 {
            return None;
        }

        let distance = (self.point - ray.origin).dot(self.normal) / facing;
        if distance > 0.001 {
            Some(distance)
        } else {
            None
        }
    }
}
