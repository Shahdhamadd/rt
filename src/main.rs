mod camera;
mod cube;
mod cylinder;
mod light;
mod object;
mod plane;
mod ray;
mod scene;
mod sphere;
mod vec3;

use camera::Camera;
use light::Light;
use object::Object;
use ray::Ray;
use scene::Scene;
use std::env;
use std::fs;
use std::process;
//for using all CPU cores
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use vec3::Vec3;

const SAMPLES_PER_SIDE: i32 = 6;
const SHININESS: f64 = 40.0;

fn random(seed: &mut u64) -> f64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    (*seed >> 11) as f64 / (1u64 << 53) as f64
}

fn color_line(color: Vec3) -> String {
    let color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
    let red = (color.x.clamp(0.0, 1.0) * 255.0).round() as i32;
    let green = (color.y.clamp(0.0, 1.0) * 255.0).round() as i32;
    let blue = (color.z.clamp(0.0, 1.0) * 255.0).round() as i32;
    format!("{} {} {}\n", red, green, blue)
}

fn sky_color(direction: Vec3) -> Vec3 {
    let unit_direction = direction.unit();
    let up_amount = (unit_direction.y + 1.0) * 0.5;
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white * (1.0 - up_amount) + blue * up_amount
}

fn closest_hit(ray: Ray, objects: &[Object]) -> Option<(f64, &Object)> {
    let mut closest_distance = f64::INFINITY;
    let mut closest_object = None;

    for object in objects {
        if let Some(distance) = object.hit(ray)
            && distance < closest_distance
        {
            closest_distance = distance;
            closest_object = Some(object);
        }
    }

    match closest_object {
        Some(object) => Some((closest_distance, object)),
        None => None,
    }
}

fn bounce(direction: Vec3, normal: Vec3) -> Vec3 {
    let direction = direction.unit();
    direction - normal * (2.0 * direction.dot(normal))
}

fn light_amount(
    point: Vec3,
    normal: Vec3,
    view_direction: Vec3,
    shine: f64,
    objects: &[Object],
    light: &Light,
    light_offset: Vec3,
) -> (f64, f64) {
    let ambient = 0.1;

    let to_light = light.position + light_offset - point;
    let light_distance = to_light.length();
    let light_direction = to_light.unit();

    let shadow_ray = Ray::new(point, light_direction);
    if let Some((blocker_distance, _)) = closest_hit(shadow_ray, objects)
        && blocker_distance < light_distance
    {
        return (ambient, 0.0);
    }

    let facing = normal.dot(light_direction).max(0.0);
    let diffuse = ambient + facing * light.brightness;

    let bounced_light = bounce(light_direction * -1.0, normal);
    let to_eye = (view_direction * -1.0).unit();
    let highlight = bounced_light.dot(to_eye).max(0.0).powf(SHININESS) * shine * light.brightness;

    (diffuse, highlight)
}

fn ray_color(
    ray: Ray,
    objects: &[Object],
    light: &Light,
    light_offset: Vec3,
    bounces_left: u32,
) -> Vec3 {
    match closest_hit(ray, objects) {
        None => sky_color(ray.direction),
        Some((distance, object)) => {
            let point = ray.at(distance);
            let normal = object.normal_at(point);
            let (diffuse, highlight) = light_amount(
                point,
                normal,
                ray.direction,
                object.shine(),
                objects,
                light,
                light_offset,
            );
            let white = Vec3::new(1.0, 1.0, 1.0);
            let own_color = object.color() * diffuse + white * highlight;

            let reflect = object.reflect();
            if reflect == 0.0 || bounces_left == 0 {
                return own_color;
            }

            let mirror_ray = Ray::new(point, bounce(ray.direction, normal));
            let mirror_color =
                ray_color(mirror_ray, objects, light, light_offset, bounces_left - 1);
            own_color * (1.0 - reflect) + mirror_color * reflect
        }
    }
}

fn render_row(
    y: i32,
    width: i32,
    height: i32,
    scene: &Scene,
    camera: &Camera,
    bounces: u32,
) -> String {
    let light = &scene.light;
    let mut seed: u64 = 12345 + y as u64 * 7919;
    let mut row = String::new();
    for x in 0..width {
        let mut total = Vec3::new(0.0, 0.0, 0.0);
        for sample_y in 0..SAMPLES_PER_SIDE {
            for sample_x in 0..SAMPLES_PER_SIDE {
                let step_x = (sample_x as f64 + 0.5) / SAMPLES_PER_SIDE as f64;
                let step_y = (sample_y as f64 + 0.5) / SAMPLES_PER_SIDE as f64;

                let right_amount = (x as f64 + step_x) / width as f64;
                let down_amount = (y as f64 + step_y) / height as f64;
                let ray = camera.ray(right_amount, down_amount);

                let light_offset = Vec3::new(
                    ((sample_y as f64 + random(&mut seed)) / SAMPLES_PER_SIDE as f64 - 0.5)
                        * light.size,
                    0.0,
                    ((sample_x as f64 + random(&mut seed)) / SAMPLES_PER_SIDE as f64 - 0.5)
                        * light.size,
                );
                total = total + ray_color(ray, &scene.objects, light, light_offset, bounces);
            }
        }
        let sample_count = (SAMPLES_PER_SIDE * SAMPLES_PER_SIDE) as f64;
        row.push_str(&color_line(total * (1.0 / sample_count)));
    }
    row
}

fn print_usage_and_quit() -> ! {
    eprintln!("Usage: cargo run --release -- <scene file> [width] [height] [-r] [-o image.ppm]");
    eprintln!("Scene files are in the scenes/ folder, for example scenes/all.txt");
    eprintln!("-r turns on reflections (bonus)");
    eprintln!("-o saves the picture to a file");
    eprintln!("Example: cargo run --release -- scenes/all.txt 800 600 -o all.ppm");
    process::exit(1);
}

fn read_number(args: &[String], index: usize, default: i32) -> i32 {
    if index >= args.len() {
        return default;
    }
    match args[index].parse::<i32>() {
        Ok(number) if number > 1 => number,
        _ => {
            eprintln!("'{}' is not a valid size", args[index]);
            print_usage_and_quit();
        }
    }
}

fn main() {
    let all_args: Vec<String> = env::args().collect();
    let mut args: Vec<String> = Vec::new();
    let mut reflections = false;
    let mut output: Option<String> = None;
    let mut index = 0;
    while index < all_args.len() {
        if all_args[index] == "-r" {
            reflections = true;
        } else if all_args[index] == "-o" {
            if index + 1 >= all_args.len() {
                eprintln!("-o needs a file name");
                print_usage_and_quit();
            }
            output = Some(all_args[index + 1].clone());
            index += 1;
        } else {
            args.push(all_args[index].clone());
        }
        index += 1;
    }

    let bounces = if reflections { 5 } else { 0 };

    let scene_path = if args.len() > 1 {
        args[1].as_str()
    } else {
        "scenes/all.txt"
    };
    let width = read_number(&args, 2, 400);
    let height = read_number(&args, 3, 300);

    let scene: Scene = match scene::load(scene_path) {
        Ok(scene) => scene,
        Err(message) => {
            eprintln!("{}", message);
            print_usage_and_quit();
        }
    };

    let camera = Camera::new(scene.camera_position, scene.look_at, width, height);

    let mut image = String::new();
    image.push_str("P3\n");
    image.push_str(&format!("{} {}\n", width, height));
    image.push_str("255\n");
    //how many cores
    let thread_count = thread::available_parallelism()
        .map(|count| count.get() as i32)
        .unwrap_or(4);
    let rows_left = AtomicI32::new(height);

    let finished: Vec<Vec<(i32, String)>> = thread::scope(|scope| {
        let mut workers = Vec::new();
        for first_row in 0..thread_count {
            let scene = &scene;
            let camera = &camera;
            let rows_left = &rows_left;
            workers.push(scope.spawn(move || {
                let mut my_rows = Vec::new();
                let mut y = first_row;
                while y < height {
                    let row = render_row(y, width, height, scene, camera, bounces);
                    my_rows.push((y, row));
                    let left = rows_left.fetch_sub(1, Ordering::Relaxed) - 1;
                    eprintln!("Rows left: {}", left);
                    y += thread_count;
                }
                my_rows
            }));
        }
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect()
    });

    let mut rows = vec![String::new(); height as usize];
    for worker_rows in finished {
        for (y, row) in worker_rows {
            rows[y as usize] = row;
        }
    }
    for row in rows {
        image.push_str(&row);
    }

    match output {
        Some(path) => {
            if let Err(error) = fs::write(&path, image) {
                eprintln!("Could not write '{}': {}", path, error);
                process::exit(1);
            }
            eprintln!("Done. Saved {}", path);
        }
        None => {
            print!("{}", image);
            eprintln!("Done.");
        }
    }
}
