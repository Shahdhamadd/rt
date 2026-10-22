use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cube {
    pub center: Vec3,
    pub size: f64,
    pub color: Vec3,
    pub reflect: f64,
    pub shine: f64,
}

fn enter_and_exit(start: f64, step: f64, low: f64, high: f64) -> (f64, f64) {
    let first = (low - start) / step;
    let second = (high - start) / step;
    if first < second {
        (first, second)
    } else {
        (second, first)
    }
}

impl Cube {
    pub fn with_shine(mut self, amount: f64) -> Cube {
        self.shine = amount;
        self
    }

    pub fn with_reflection(mut self, amount: f64) -> Cube {
        self.reflect = amount;
        self
    }

    pub fn new(center: Vec3, size: f64, color: Vec3) -> Cube {
        Cube {
            center,
            size,
            color,
            reflect: 0.0,
            shine: 0.0,
        }
    }

    pub fn hit(&self, ray: Ray) -> Option<f64> {
        let half = self.size / 2.0;
        let low = self.center - Vec3::new(half, half, half);
        let high = self.center + Vec3::new(half, half, half);

        let (x_enter, x_exit) = enter_and_exit(ray.origin.x, ray.direction.x, low.x, high.x);
        let (y_enter, y_exit) = enter_and_exit(ray.origin.y, ray.direction.y, low.y, high.y);
        let (z_enter, z_exit) = enter_and_exit(ray.origin.z, ray.direction.z, low.z, high.z);

        let enter = x_enter.max(y_enter).max(z_enter);
        let exit = x_exit.min(y_exit).min(z_exit);

        if enter > exit {
            return None;
        }

        if enter > 0.001 {
            Some(enter)
        } else if exit > 0.001 {
            Some(exit)
        } else {
            None
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        let from_center = point - self.center;
        let x = from_center.x.abs();
        let y = from_center.y.abs();
        let z = from_center.z.abs();

        if x > y && x > z {
            Vec3::new(from_center.x.signum(), 0.0, 0.0)
        } else if y > z {
            Vec3::new(0.0, from_center.y.signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, from_center.z.signum())
        }
    }
}
