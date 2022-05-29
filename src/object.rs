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

impl Object {
	pub fn load(display: &Display, path: &String) -> Self {
		let file = File::open(path).unwrap();
		let lines = BufReader::new(file).lines();

		let mut vertices: Vec::<Vec<f32>> = vec![vec![0.0, 0.0, 0.0]];
		let mut textures: Vec::<Vec<f32>> = vec![vec![0.0, 0.0]];
		let mut normals: Vec::<Vec<f32>> = vec![vec![0.0, 0.0, 0.0]];
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
				Some("v") | Some("vt") | Some("vn") => {
					let vec: Vec<f32> = data.map(|s| s.parse::<f32>().unwrap()).collect();
					match elm {
						Some("v" ) => vertices.push(vec),
						Some("vt") => textures.push(vec),
						Some("vn") => normals .push(vec),
						_ => ()
					};
				},
				Some("f") => {
					let face: Vec<&str> = data.collect();
					assert_eq!(face.len(), 3, "model is not triangulated");
					for vertex in face {
						let vtn: Vec<usize> = vertex.split("/").map(|s| s.parse::<usize>().unwrap_or(0)).collect();

						vertex_data.push(Vertex {
							position: vertices[vtn[0]].as_slice().try_into().unwrap(),
							texture : textures[vtn[1]].as_slice().try_into().unwrap(),
							normal  : normals [vtn[2]].as_slice().try_into().unwrap()
						});
					}
				},
				_ => ()
			}
		}
	
		Self {
			vertex_buffer: VertexBuffer::new(display, &vertex_data).unwrap()
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