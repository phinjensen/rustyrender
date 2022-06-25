use crate::geometry::Vec3;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Model {
    pub vertices: Vec<Vec3<f64>>,
    pub faces: Vec<Vec<usize>>,
}

fn get_vertices(line: String) -> Result<Vec3<f64>, String> {
    let result_array: Result<Vec<_>, _> = line
        .split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse::<f64>())
        .collect();
    match result_array {
        Ok(arr) => Ok(Vec3::from_slice(arr.as_slice())),
        Err(_) => Err(String::from("Couldn't parse 3 numbers form line")),
    }
}

fn get_face_indicies(line: String) -> Result<Vec<usize>, String> {
    line.split_ascii_whitespace()
        .take(3)
        .map(|x| {
            if let Some(x) = x.split("/").next() {
                match x.parse::<usize>() {
                    Ok(x) => Ok(x - 1),
                    Err(_) => Err(String::from("Failed to parse face vertex number")),
                }
            } else {
                Err(String::from("Missing face vertex number"))
            }
        })
        .collect()
}

impl Model {
    pub fn from(filename: &str) -> Result<Self, String> {
        let f = File::open(filename);
        let f = match f {
            Err(_) => return Err(format!("Couldn't open object {filename}.")),
            Ok(f) => f,
        };
        let reader = BufReader::new(f);
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("v ") {
                    if let Ok(vertex) = get_vertices(line[2..].to_string()) {
                        vertices.push(vertex);
                    }
                } else if line.starts_with("f ") {
                    if let Ok(face) = get_face_indicies(line[2..].to_string()) {
                        faces.push(face);
                    }
                }
            }
        }
        Ok(Model { vertices, faces })
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    pub fn vertex(&self, i: usize) -> &Vec3<f64> {
        &self.vertices[i]
    }

    pub fn face(&self, i: usize) -> &Vec<usize> {
        &self.faces[i]
    }
}
