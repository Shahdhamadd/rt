use crate::cube::Cube;
use crate::cylinder::Cylinder;
use crate::light::Light;
use crate::object::Object;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use std::fs;

pub struct Scene {
    pub objects: Vec<Object>,
    pub light: Light,
    pub camera_position: Vec3,
    pub look_at: Vec3,
}

struct Line {
    name: String,
    numbers: Vec<f64>,
    reflect: Option<f64>,
    shine: Option<f64>,
}
//parsing txt files in scenes then returns a scene
fn read_number(word: &str, line_number: usize) -> Result<f64, String> {
    match word.parse::<f64>() {
        Ok(number) => Ok(number),
        Err(_) => Err(format!("line {}: '{}' is not a number", line_number, word)),
    }
}

fn read_line(words: &[&str], line_number: usize) -> Result<Line, String> {
    let mut line = Line {
        name: words[0].to_string(),
        numbers: Vec::new(),
        reflect: None,
        shine: None,
    };

    let mut position = 1;
    while position < words.len() {
        let word = words[position];
        if word == "reflect" || word == "shine" {
            if position + 1 >= words.len() {
                return Err(format!(
                    "line {}: '{}' needs a number after it",
                    line_number, word
                ));
            }
            let value = read_number(words[position + 1], line_number)?;
            if word == "reflect" {
                line.reflect = Some(value);
            } else {
                line.shine = Some(value);
            }
            position += 2;
        } else {
            line.numbers.push(read_number(word, line_number)?);
            position += 1;
        }
    }
    Ok(line)
}

fn check_count(line: &Line, needed: usize, line_number: usize) -> Result<(), String> {
    if line.numbers.len() != needed {
        return Err(format!(
            "line {}: '{}' needs {} numbers, found {}",
            line_number,
            line.name,
            needed,
            line.numbers.len()
        ));
    }
    Ok(())
}

fn check_no_extras(line: &Line, line_number: usize) -> Result<(), String> {
    if line.reflect.is_some() || line.shine.is_some() {
        return Err(format!(
            "line {}: only objects can have 'reflect' or 'shine'",
            line_number
        ));
    }
    Ok(())
}

fn vec3_at(numbers: &[f64], start: usize) -> Vec3 {
    Vec3::new(numbers[start], numbers[start + 1], numbers[start + 2])
}

pub fn load(path: &str) -> Result<Scene, String> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => return Err(format!("Could not read '{}': {}", path, error)),
    };

    let mut objects = Vec::new();
    let mut light = None;
    let mut camera_position = None;
    let mut look_at = None;

    for (index, text_line) in text.lines().enumerate() {
        let line_number = index + 1;
        let text_line = text_line.split('#').next().unwrap_or("").trim();
        if text_line.is_empty() {
            continue;
        }

        let words: Vec<&str> = text_line.split_whitespace().collect();
        let line = read_line(&words, line_number)?;
        let numbers = &line.numbers;
        let reflect = line.reflect.unwrap_or(0.0);
        let shine = line.shine.unwrap_or(0.0);

        match line.name.as_str() {
            "camera" => {
                check_count(&line, 3, line_number)?;
                check_no_extras(&line, line_number)?;
                camera_position = Some(vec3_at(numbers, 0));
            }
            "look_at" => {
                check_count(&line, 3, line_number)?;
                check_no_extras(&line, line_number)?;
                look_at = Some(vec3_at(numbers, 0));
            }
            "light" => {
                if numbers.len() != 5 {
                    check_count(&line, 4, line_number)?;
                }
                check_no_extras(&line, line_number)?;
                let mut new_light = Light::new(vec3_at(numbers, 0), numbers[3]);
                if numbers.len() == 5 {
                    new_light.size = numbers[4];
                }
                light = Some(new_light);
            }
            "sphere" => {
                check_count(&line, 7, line_number)?;
                objects.push(Object::Sphere(
                    Sphere::new(vec3_at(numbers, 0), numbers[3], vec3_at(numbers, 4))
                        .with_reflection(reflect)
                        .with_shine(shine),
                ));
            }
            "cube" => {
                check_count(&line, 7, line_number)?;
                objects.push(Object::Cube(
                    Cube::new(vec3_at(numbers, 0), numbers[3], vec3_at(numbers, 4))
                        .with_reflection(reflect)
                        .with_shine(shine),
                ));
            }
            "cylinder" => {
                check_count(&line, 8, line_number)?;
                objects.push(Object::Cylinder(
                    Cylinder::new(
                        vec3_at(numbers, 0),
                        numbers[3],
                        numbers[4],
                        vec3_at(numbers, 5),
                    )
                    .with_reflection(reflect)
                    .with_shine(shine),
                ));
            }
            "plane" => {
                check_count(&line, 9, line_number)?;
                objects.push(Object::Plane(
                    Plane::new(
                        vec3_at(numbers, 0),
                        vec3_at(numbers, 3),
                        vec3_at(numbers, 6),
                    )
                    .with_reflection(reflect)
                    .with_shine(shine),
                ));
            }
            _ => {
                return Err(format!(
                    "line {}: unknown word '{}'",
                    line_number, line.name
                ));
            }
        }
    }

    let Some(camera_position) = camera_position else {
        return Err(format!("{}: missing a 'camera' line", path));
    };
    let Some(look_at) = look_at else {
        return Err(format!("{}: missing a 'look_at' line", path));
    };
    let Some(light) = light else {
        return Err(format!("{}: missing a 'light' line", path));
    };

    Ok(Scene {
        objects,
        light,
        camera_position,
        look_at,
    })
}
