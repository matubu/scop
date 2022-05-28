extern crate glium;

use glium::{
	{
		Display,
		Surface,
		program,
		uniform
	},
	glutin::{
		{ContextBuilder, Api, GlRequest},
		event::{Event, WindowEvent},
		event_loop::{EventLoop, ControlFlow},
		window::WindowBuilder
	}
};

mod object;
use object::Object;

mod rgb;

use std::time::Instant;

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
				out vec3 v_position;

				uniform mat4 matrix;

				void main() {
					v_normal = transpose(inverse(mat3(matrix))) * normal;
					v_position = position;
					gl_Position = matrix * vec4(position, 1.0);
				}
			",
			fragment: "
				#version 150

				in vec3 v_normal;
				in vec3 v_position;

				out vec4 f_color;

				uniform vec3 light;
				uniform vec3 color;

				void main() {
					// float brightness = dot(normalize(v_normal), normalize(light));
					f_color = vec4(v_normal * v_position, 1.0);
					// vec3 dark_color = vec3(0.6, 0.1, 0.1);
					// f_color = vec4(mix(dark_color, color, brightness), 1.0);
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

	let start = Instant::now();

	event_loop.run(move |event, _, control_flow| {
		let elapsed = start.elapsed().as_secs_f32() * 0.1;
		let color: [f32; 3] = rgb::from_hsl(elapsed % 1.0, 0.7, 0.8);

		let mut target = display.draw();
		target.clear_color_and_depth((0.0, 0.0, 0.02, 1.0), 1.0);
		obj.draw(
			&mut target,
			&program,
			&uniform! {
				matrix: MATRIX,
				light: LIGHT,
				color: color
			}
		);
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
