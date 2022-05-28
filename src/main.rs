extern crate glium;
extern crate obj;

use glium::{
	{
		Display,
		Frame,
		Surface,
		DrawParameters,
		Program,
		program,
		implement_vertex,
		uniform
	},
	uniforms::{
		UniformBuffer
	}
	vertex::VertexBuffer,
	index::{
		NoIndices,
		PrimitiveType::TrianglesList,
	},
	glutin::{
		{ContextBuilder, Api, GlRequest},
		event::{Event, WindowEvent},
		event_loop::{EventLoop, ControlFlow},
		window::WindowBuilder
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

struct Object {
	vertex_buffer: VertexBuffer<Vertex>
}

impl Object {
	fn load(display: &Display, data: &[u8]) -> Self {
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
	fn draw(&self, frame: &Frame, program: Program) -> Self {
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
			&program,
			&uniform! { matrix: MATRIX, light: LIGHT },
			&params
		).unwrap();

		*self
	}
}

fn main() {
	let event_loop = EventLoop::new();
	let win = WindowBuilder::new()
				.with_title("scop");
	let ctx = ContextBuilder::new()
				.with_multisampling(16)
				.with_depth_buffer(24)
				.with_vsync(true)
				.with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)));
	let display = Display::new(win, ctx, &event_loop).unwrap();

	let obj: Object = Object::load(&display, include_bytes!("../monkey.obj"));
	let program = program!(&display,
		150 => {
			vertex: "
				#version 150

				in vec3 position;
				in vec3 normal;

				out vec3 v_normal;

				uniform mat4 matrix;

				void main() {
					v_normal = transpose(inverse(mat3(matrix))) * normal;
					gl_Position = matrix * vec4(position, 1.0);
				}
			",
			fragment: "
				#version 150

				in vec3 v_normal;
				out vec4 color;
				uniform vec3 light;

				void main() {
					float brightness = dot(normalize(v_normal), normalize(light));
					vec3 dark_color = vec3(0.6, 0.1, 0.1);
					vec3 regular_color = vec3(1.0, 0.2, 0.2);
					color = vec4(mix(dark_color, regular_color, brightness), 1.0);
				}
			",
		}
	).unwrap();

	const MATRIX: [[f32; 4]; 4] = [
		[0.3, 0.0, 0.0, 0.0],
		[0.0, 0.3, 0.0, 0.0],
		[0.0, 0.0, -0.3, 0.0],
		[0.0, 0.0, 0.0, 1.0]
	];
	const LIGHT: [f32; 3] = [-1.0, 0.4, 0.9];

	const uniforms: UniformBuffer = uniform! { matrix: MATRIX, light: LIGHT };

	event_loop.run(move |event, _, control_flow| {
		let mut target = display.draw();
		target.clear_color_and_depth((0.0, 0.0, 0.02, 1.0), 1.0);
		obj.draw(&target, program, );
		target.draw(
				&vertex_buffer,
				&NoIndices(TrianglesList),
				&program,
				&,
				&params
			).unwrap();
		target.finish().unwrap();

		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			_ => ()
		}
	});
}
