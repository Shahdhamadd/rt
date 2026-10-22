use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::plane::Plane;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
}

impl Object {
    pub fn hit(&self, ray: Ray) -> Option<f64> {
        match self {
            Object::Sphere(sphere) => sphere.hit(ray),
            Object::Plane(plane) => plane.hit(ray),
            Object::Cube(cube) => cube.hit(ray),
            Object::Cylinder(cylinder) => cylinder.hit(ray),
        }
    }

    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        match self {
            Object::Sphere(sphere) => sphere.normal_at(point),
            Object::Plane(plane) => plane.normal,
            Object::Cube(cube) => cube.normal_at(point),
            Object::Cylinder(cylinder) => cylinder.normal_at(point),
        }
    }

    pub fn color(&self) -> Vec3 {
        match self {
            Object::Sphere(sphere) => sphere.color,
            Object::Plane(plane) => plane.color,
            Object::Cube(cube) => cube.color,
            Object::Cylinder(cylinder) => cylinder.color,
        }
    }

    pub fn reflect(&self) -> f64 {
        match self {
            Object::Sphere(sphere) => sphere.reflect,
            Object::Plane(plane) => plane.reflect,
            Object::Cube(cube) => cube.reflect,
            Object::Cylinder(cylinder) => cylinder.reflect,
        }
    }

    pub fn shine(&self) -> f64 {
        match self {
            Object::Sphere(sphere) => sphere.shine,
            Object::Plane(plane) => plane.shine,
            Object::Cube(cube) => cube.shine,
            Object::Cylinder(cylinder) => cylinder.shine,
        }
    }
}
