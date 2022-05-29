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
		window::WindowBuilder,
		dpi::LogicalSize
	}
};

mod object;
use object::Object;

mod texture;
use texture::Texture;

use std::time::Instant;

fn main() {
	let event_loop = EventLoop::new();
	let win = WindowBuilder::new()
				.with_title("scop")
				.with_inner_size(LogicalSize::new(1024.0, 512.0))
				.with_resizable(false);
	let ctx = ContextBuilder::new()
				.with_multisampling(16)
				.with_depth_buffer(24)
				.with_vsync(true)
				.with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)));
	let display = Display::new(win, ctx, &event_loop).unwrap();

	let obj_path = std::env::args().nth(1).unwrap_or("monkey.obj".to_string());
	let obj: Object = Object::load(&display, &obj_path);

	let tex_path = "texture.jpg".to_string();
	let tex: Texture = Texture::load(&display, &tex_path);

	let program = program!(&display,
		150 => {
			vertex: "
				#version 150

				in vec3 position;
				in vec2 texture;
				in vec3 normal;

				out vec3 v_normal;
				out vec2 v_texture;
				out vec3 v_position;

				uniform mat4 matrix;

				void main() {
					v_normal = transpose(inverse(mat3(matrix))) * normal;
					v_texture = texture;
					v_position = position;
					gl_Position = matrix * vec4(position * 0.3, 1.0);
				}
			",
			fragment: "
				#version 150

				in vec3 v_normal;
				in vec2 v_texture;
				in vec3 v_position;

				out vec4 f_color;

				uniform vec3 light;
				uniform sampler2D tex;

				void main() {
					float brightness = dot(normalize(v_normal), normalize(light));
					// vec4 dark_color = vec4(0.56, 0.56, 0.58, 1.0);
					// vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);
					// f_color = texture(tex, v_texture) * mix(dark_color, light_color, brightness);
					vec3 dark_color = vec3(0.125, 0.2, 0.5);
					vec3 light_color = vec3(0.2, 0.4, 0.9);
					f_color = vec4(mix(dark_color, light_color, brightness), 1.0);
				}
			",
		}
	).unwrap();

	let start = Instant::now();

	event_loop.run(move |event, _, control_flow| {
		let elapsed = start.elapsed().as_secs_f64();

		let mut target = display.draw();
		target.clear_color_and_depth((0.005, 0.005, 0.018, 1.0), 1.0);

		let a: f32 = 0.0; // yaw
		let b: f32 = ((elapsed * 0.2) % std::f64::consts::TAU) as f32; // pitch
		let y: f32 = 0.0; // roll

		let matrix: [[f32; 4]; 4] = [
			[
				a.cos() * b.cos(),
				a.cos() * b.sin() * y.sin() - a.sin() * y.cos(),
				a.cos() * b.sin() * y.cos() + a.sin() + y.sin(),
				0.0
			],
			[
				a.sin() * b.cos(),
				a.sin() * b.sin() * y.sin() + a.cos() + y.cos(),
				a.sin() * b.sin() * y.cos() - a.cos() * y.sin(),
				0.0
			],
			[
				-b.sin(),
				b.cos() * y.sin(),
				b.cos() * y.cos(),
				0.0
			],
			[0.0, 0.0, 0.0, 1.0]
		];
		let light: [f32; 3] = [-1.0, 0.4, 0.9];

		obj.draw(
			&mut target,
			&program,
			&uniform! {
				matrix: matrix,
				light: light,
				tex: &tex.texture
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
