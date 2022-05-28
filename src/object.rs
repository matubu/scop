extern crate glium;
extern crate obj;

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

use obj::ObjData;
use std::io::BufReader;

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
	pub fn load(display: &Display, data: &[u8]) -> Self {
		let data = ObjData::load_buf(BufReader::new(data)).unwrap();

		let mut vertex_data = Vec::new();
	
		for object in data.objects {
			for group in object.groups {
				for polygon in group.polys {
					for vertex in polygon.0.iter() {
						let position = data.position[vertex.0];
						let texture = match vertex.1 {
							Some(i) => data.texture[i],
							None => [0.0, 0.0]
						};
						let normal = match vertex.2 {
							Some(i) => data.normal[i],
							None => [0.0, 0.0, 0.0]
						};
	
						vertex_data.push(Vertex {
							position,
							normal,
							texture
						})
					}
				}
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