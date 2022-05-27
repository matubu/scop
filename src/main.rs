extern crate glium;
extern crate obj;

use glium::{
	{
		Display,
		Surface,
		program,
		implement_vertex
	},
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
	// texture: [f32; 2],
	// normal: [f32; 3]
}

fn load_wavefront(display: &Display, data: &[u8]) -> VertexBuffer<Vertex> {
    // implement_vertex!(Vertex, position, texture, normal);
    implement_vertex!(Vertex, position);

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
						// normal,
						// texture
					})
				}
			}
		}
    }

    VertexBuffer::new(display, &vertex_data).unwrap()
}

fn main() {
	let event_loop = EventLoop::new();
	let win = WindowBuilder::new()
				.with_title("scop");
	let ctx = ContextBuilder::new()
				.with_multisampling(0)
				.with_vsync(true)
				.with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)));
	let display = Display::new(win, ctx, &event_loop).unwrap();

    let vertex_buffer = load_wavefront(&display, include_bytes!("../monkey.obj"));
	let program = program!(&display,
		140 => {
			vertex: "
				#version 140

				in vec3 position;

				void main() {
					gl_Position = vec4(position, 10);
				}
			",
			fragment: "
				#version 140

                out vec4 color;

                void main() {
                    color = vec4(0.5, 1.0, 1.0, 1.0) * gl_FragCoord.z;
                }
			",
		}
	).unwrap();

	event_loop.run(move |event, _, control_flow| {
		let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.02, 1.0);
        target.draw(
				&vertex_buffer,
				&NoIndices(TrianglesList),
				&program,
				&glium::uniforms::EmptyUniforms,
				&Default::default()
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
