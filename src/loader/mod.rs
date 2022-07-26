use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::instance::Instance;

pub fn load_instances(file_name: String) -> Vec<Instance> {
    // Create Vec<Instance> to be returned
    let mut return_vector: Vec<Instance> = Vec::new();

    // Create Path and Display
    let path = Path::new(&file_name);
    let display = path.display();

    // Open file and handle errors
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read into String buffer
    let mut loaded = String::new();
    file.read_to_string(&mut loaded).unwrap();

    // Split the string into Vector
    let mut v: Vec<&str> = loaded.split(";").collect();

    while v.len() > 0 {
        let v_string = v.remove(0).to_string();
        let v_split: Vec<&str> = v_string.split(":").collect();
        let points_string = v_split[0].to_string();
        let points_split: Vec<&str> = points_string.split(",").collect();

        let mut points: Vec<f64> = Vec::new();
        for j in 0..points_split.len() {
            points.push(points_split[j].parse().unwrap())
        }

        return_vector.push(Instance::new(points, v_split[1].parse().unwrap()))
    }

    return return_vector;
}

pub fn write_instances(instances: &Vec<Instance>, file_name: String) {
    // Build String to write
    let mut to_write: String = String::new();

    for i in 0..instances.len() {
        for j in 0..instances[i].points.len() {
            to_write += &instances[i].points[j].to_string();
            if j < instances[i].points.len() - 1 { to_write += ","}
        }
        to_write += ":";
        to_write += &instances[i].label.to_string();

        if i < instances.len() - 1 { to_write += ";" }
    }

    let mut file = File::create(file_name).unwrap();

    let _result_that_must_be_used = file.write_all(&to_write.as_bytes()).unwrap();
}