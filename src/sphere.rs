use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Vec3,
    pub reflect: f64,
    pub shine: f64,
}

impl Sphere {
    pub fn with_shine(mut self, amount: f64) -> Sphere {
        self.shine = amount;
        self
    }

    pub fn with_reflection(mut self, amount: f64) -> Sphere {
        self.reflect = amount;
        self
    }

    pub fn new(center: Vec3, radius: f64, color: Vec3) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            reflect: 0.0,
            shine: 0.0,
        }
    }

    pub fn hit(&self, ray: Ray) -> Option<f64> {
        let from_center = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * from_center.dot(ray.direction);
        let c = from_center.dot(from_center) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let near = (-b - discriminant.sqrt()) / (2.0 * a);
        let far = (-b + discriminant.sqrt()) / (2.0 * a);

        if near > 0.001 {
            Some(near)
        } else if far > 0.001 {
            Some(far)
        } else {
            None
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.center).unit()
    }
}
