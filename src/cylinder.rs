use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cylinder {
    pub bottom: Vec3,
    pub radius: f64,
    pub height: f64,
    pub color: Vec3,
    pub reflect: f64,
    pub shine: f64,
}

impl Cylinder {
    pub fn with_shine(mut self, amount: f64) -> Cylinder {
        self.shine = amount;
        self
    }

    pub fn with_reflection(mut self, amount: f64) -> Cylinder {
        self.reflect = amount;
        self
    }

    pub fn new(bottom: Vec3, radius: f64, height: f64, color: Vec3) -> Cylinder {
        Cylinder {
            bottom,
            radius,
            height,
            color,
            reflect: 0.0,
            shine: 0.0,
        }
    }

    fn top_y(&self) -> f64 {
        self.bottom.y + self.height
    }

    fn side_hits(&self, ray: Ray) -> Vec<f64> {
        let from_center_x = ray.origin.x - self.bottom.x;
        let from_center_z = ray.origin.z - self.bottom.z;

        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        let b = 2.0 * (from_center_x * ray.direction.x + from_center_z * ray.direction.z);
        let c = from_center_x * from_center_x + from_center_z * from_center_z
            - self.radius * self.radius;

        if a < 0.000001 {
            return vec![];
        }

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        let near = (-b - discriminant.sqrt()) / (2.0 * a);
        let far = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![near, far]
    }

    fn cap_hit(&self, ray: Ray, cap_y: f64) -> Option<f64> {
        if ray.direction.y.abs() < 0.000001 {
            return None;
        }

        let distance = (cap_y - ray.origin.y) / ray.direction.y;
        let point = ray.at(distance);
        let dx = point.x - self.bottom.x;
        let dz = point.z - self.bottom.z;

        if dx * dx + dz * dz <= self.radius * self.radius {
            Some(distance)
        } else {
            None
        }
    }

    pub fn hit(&self, ray: Ray) -> Option<f64> {
        let mut closest = f64::INFINITY;

        for distance in self.side_hits(ray) {
            let y = ray.at(distance).y;
            let inside_height = y >= self.bottom.y && y <= self.top_y();
            if distance > 0.001 && distance < closest && inside_height {
                closest = distance;
            }
        }

        for cap_y in [self.bottom.y, self.top_y()] {
            if let Some(distance) = self.cap_hit(ray, cap_y)
                && distance > 0.001
                && distance < closest
            {
                closest = distance;
            }
        }

        if closest == f64::INFINITY {
            None
        } else {
            Some(closest)
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        if (point.y - self.top_y()).abs() < 0.001 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (point.y - self.bottom.y).abs() < 0.001 {
            Vec3::new(0.0, -1.0, 0.0)
        } else {
            Vec3::new(point.x - self.bottom.x, 0.0, point.z - self.bottom.z).unit()
        }
    }
}
