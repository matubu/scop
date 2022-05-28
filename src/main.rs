extern crate glium;
use glium::{
	{
		Display,
		Surface,
		IndexBuffer,
		DrawParameters,
		program,
		uniform
	},
	vertex::VertexBuffer,
	index::{
		PrimitiveType::TrianglesList,
	},
	glutin::{
		{ContextBuilder, Api, GlRequest},
		event::{Event, WindowEvent},
		event_loop::{EventLoop, ControlFlow},
		window::WindowBuilder
	}
};

mod teapot;

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

	const MATRIX: [[f32; 4]; 4] = [
		[0.01, 0.0, 0.0, 0.0],
		[0.0, 0.01, 0.0, 0.0],
		[0.0, 0.0, 0.01, 0.0],
		[0.0, 0.0, 0.0, 1.0]
	];

	const LIGHT: [f32; 3] = [-1.0, 0.4, 0.9];

	let params = DrawParameters {
		depth: glium::Depth {
			test: glium::draw_parameters::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		.. Default::default()
	};

	let positions = VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
	let normals = VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
	// let vertex_buffer = VertexBuffer
	let indices = IndexBuffer::new(&display, TrianglesList,
										&teapot::INDICES).unwrap();

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

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => *control_flow = ControlFlow::Exit,
			_ => ()
		}

		let mut target = display.draw();
		target.clear_color_and_depth((0.0, 0.0, 0.02, 1.0), 1.0);

		target.draw(
				(&positions, &normals),
				&indices,
				&program,
				&uniform! { matrix: MATRIX, light: LIGHT },
				&params
		).unwrap();
		target.finish().unwrap();
	});
}