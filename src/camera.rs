use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    position: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    screen_width: f64,
    screen_height: f64,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, width: i32, height: i32) -> Camera {
        let world_up = Vec3::new(0.0, 1.0, 0.0);

        let forward = (look_at - position).unit();
        let right = forward.cross(world_up).unit();
        let up = right.cross(forward);

        let screen_height = 2.0;
        let screen_width = screen_height * (width as f64 / height as f64);

        Camera {
            position,
            forward,
            right,
            up,
            screen_width,
            screen_height,
        }
    }

    pub fn ray(&self, right_amount: f64, down_amount: f64) -> Ray {
        let sideways = (right_amount - 0.5) * self.screen_width;
        let upwards = (0.5 - down_amount) * self.screen_height;

        let direction = self.forward + self.right * sideways + self.up * upwards;
        Ray::new(self.position, direction)
    }
}
