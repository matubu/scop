extern crate glium;

use glium::{
	{
		Display,
		Frame,
		Surface,
		DrawParameters,
		Program,
		implement_vertex
	},
	vertex::VertexBuffer,
	index::{
		NoIndices,
		PrimitiveType::TrianglesList,
	}
};

#[path = "vector.rs"]
mod vector;
use vector::Vector;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 3],
	texture: [f32; 2],
	normal: [f32; 3]
}

implement_vertex!(Vertex, position, texture, normal);

pub struct Object {
	vertex_buffer: VertexBuffer<Vertex>
}

pub struct BoundingBox {
	min: [f32; 3],
	size: [f32; 3],
	scale: f32
}


pub fn generate_normal(a: &Vector, b: &Vector, c: &Vector) -> [f32; 3] {
	let ab = b - a;
	let ac = c - a;

	ab.cross(&ac).normalize().to_array()
}

impl Object {
	fn get_bounding_box(vertex_data: &Vec::<Vertex>) -> BoundingBox {
		let mut min = [0f32; 3];
		let mut max = [0f32; 3];
		let mut size = [0f32; 3];

		for i in 0..3 {
			min[i] = vertex_data.iter().fold(f32::MAX, |a, v| f32::min(a, v.position[i]));
			max[i] = vertex_data.iter().fold(f32::MIN, |a, v| f32::max(a, v.position[i]));
			size[i] = max[i] - min[i];
		}

		BoundingBox {
			min,
			size,
			scale: size.iter().fold(f32::MIN, |a, v| f32::max(a, *v))
		}
	}

	// Error handling
	pub fn load(display: &Display, path: &String) -> Result<Self, String> {
		let file = match File::open(path) {
			Err(_) => return Err("cannot open file".to_string()),
			Ok(file) => file,
		};
		let lines = BufReader::new(file).lines();

		let mut vertices: Vec::<Vector> = vec![Vector::new()];
		let mut textures: Vec::<Vec<f32>> = vec![vec![0.0, 0.0]];
		let mut vertex_data = Vec::<Vertex>::new();
	
		for line in lines {
			let line = line.unwrap_or("".to_string());
			let line = line.trim();
			if line.chars().nth(0).unwrap_or('#') == '#' {
				continue ;
			}

			let mut data = line.split(" ");
			let elm = data.nth(0);

			match elm {
				Some("v") | Some("vt") => {
					let vec: Vec<f32> = data.map(|s| s.parse::<f32>().unwrap()).collect();
					match elm {
						Some("v" ) => vertices.push(Vector::from_vec(&vec)),
						Some("vt") => textures.push(vec),
						_ => ()
					};
				},
				Some("f") => {
					let face: Vec<&str> = data.collect();
					if face.len() != 3 {
						return Err("model is not triangulated".to_string())
					}
					let face: Vec<Vec<usize>> =
						face
							.iter()
							.map(
								|s| {
									let mut vertex: Vec<usize> = 
										s.split("/")
										.map(|s| s.parse::<usize>().unwrap_or(0))
										.collect();
									vertex.resize(3, 0);
									vertex
								}
							)
							.collect();
					let normal = generate_normal(
						&vertices[face[0][0]],
						&vertices[face[1][0]],
						&vertices[face[2][0]]
					);
					for vertex in face {
						vertex_data.push(Vertex {
							position: vertices[vertex[0]].to_array(),
							texture : textures[vertex[1]].as_slice().try_into().unwrap(),
							normal
						});
					}
				},
				_ => ()
			}
		}

		let bounding_box: BoundingBox = Object::get_bounding_box(&vertex_data);
		const SCALE: f32 = 3.0;

		for vertex in &mut vertex_data {
			for i in 0..3 {
				vertex.position[i]
					= (vertex.position[i] - bounding_box.min[i] - bounding_box.size[i] / 2.0) * SCALE / bounding_box.scale;
			}
		}
	
		match VertexBuffer::new(display, &vertex_data) {
			Err(_) => return Err("cannot create vertex buffer".to_string()),
			Ok(buf) => Ok(Self {
				vertex_buffer: buf
			}),
		}
	}

	pub fn draw<U: glium::uniforms::Uniforms>(&self, frame: &mut Frame, program: &Program, uniform: &U) {
		let params = DrawParameters {
			depth: glium::Depth {
				test: glium::draw_parameters::DepthTest::IfLess,
				write: true,
				.. Default::default()
			},
			.. Default::default()
		};

		frame.draw(
			&self.vertex_buffer,
			&NoIndices(TrianglesList),
			program,
			uniform,
			&params
		).unwrap();
	}
}