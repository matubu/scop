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
		event::{Event, WindowEvent, MouseScrollDelta},
		event_loop::{EventLoop},
		window::{WindowBuilder, CursorIcon},
		dpi::{LogicalSize, Position}
	}
};

mod object;
use object::Object;

mod texture;
use texture::Texture;

mod fps;
use fps::Fps;

mod mouse;
use mouse::Mouse;

mod matrix;
use matrix::{create_matrix};

mod render_loop;
use render_loop::{render_loop_start};

fn main() {
	let event_loop = EventLoop::new();
	let win = WindowBuilder::new()
				.with_title("scop")
				.with_inner_size(LogicalSize::new(1024.0, 1024.0));
	let ctx = ContextBuilder::new()
				.with_multisampling(4)
				.with_depth_buffer(24)
				.with_gl(GlRequest::Specific(Api::OpenGl, (4, 1)));
	let display = Display::new(win, ctx, &event_loop).unwrap();

	let obj_path = std::env::args().nth(1).unwrap_or("assets/monkey.obj".to_string());
	let mut obj: Object = Object::load(&display, &obj_path).unwrap();

	let mut tex: Texture = Texture::load(&display, "assets/metal.jpg").unwrap();

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
				uniform vec2 scaling;

				void main() {
					v_normal = transpose(inverse(mat3(matrix))) * normal;
					v_texture = texture;
					v_position = position;
					gl_Position = (matrix * vec4(position * 0.3, 1.0))
						* vec4(scaling, 1.0, 1.0);
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
					vec4 dark_color = vec4(0.56, 0.56, 0.58, 1.0);
					vec4 light_color = vec4(1.0, 1.0, 1.0, 1.0);
					f_color = texture(tex, v_texture) * mix(dark_color, light_color, brightness);
					// vec3 dark_color = vec3(0.125, 0.2, 0.5);
					// vec3 light_color = vec3(0.2, 0.4, 0.9);
					// f_color = vec4(mix(dark_color, light_color, brightness), 1.0);
				}
			",
		}
	).unwrap();

	let mut fps: Fps = Fps::new();
	let mut mouse: Mouse = Mouse::new();

	let mut rot: (f32, f32, f32) = (0.0, std::f32::consts::PI, 0.0);
	let mut scale = 1.0;

	let mut iz = 0.0;
	let mut iy = 0.0;
	let mut izoom = 0.0;

	render_loop_start(
		event_loop,
		/* RENDER */
		move |events| {
			for event in events {
				match event {
					Event::WindowEvent {
						event: WindowEvent::MouseWheel {
							delta: MouseScrollDelta::LineDelta(_, y),
							..
						},
						..
					} => izoom += y / 500.0,
					Event::WindowEvent {
						event: WindowEvent::DroppedFile(path),
						..
					} => {
						match path.to_str() {
							Some(path) => {
								if path.ends_with(".obj") {
									match Object::load(&display, &path.to_string()) {
										Ok(object) => obj = object,
										Err(e) => eprintln!("\r\x1B[2K\x1B[1;91mError\x1B[0m {e}")
									}
								}
								else {
									match Texture::load(&display, path) {
										Ok(texture) => tex = texture,
										Err(e) => eprintln!("\r\x1B[2K\x1B[1;91mError\x1B[0m {e}")
									}
								}
							},
							None => ()
						}
						
					}
					_ => ()
				}
			}
			mouse.apply_events(events);
		
			if mouse.is_left_button_pressed() {
				let delta = mouse.get_delta();
				iz = delta.0 as f32 / 100.0;
				iy = delta.1 as f32 / 100.0;
			}

			display.gl_window().window().set_cursor_icon(
				if mouse.is_left_button_pressed() { CursorIcon::Grabbing }
				else { CursorIcon::Grab }
			);

			iz *= 0.95;
			iy *= 0.95;
			izoom *= 0.90;
			rot.1 += iz;
			rot.0 = (rot.0 + iy).clamp(
				-std::f32::consts::FRAC_PI_2,
				std::f32::consts::FRAC_PI_2
			);
			scale = f32::max(scale * (1.0 + izoom), 0.05);
	
			fps.tick();
	
			let mut target = display.draw();
			target.clear_color_and_depth((0.005, 0.005, 0.018, 1.0), 1.0);
	
			let light: [f32; 3] = [-1.0, 0.4, 0.9];

			let dimentions = target.get_dimensions();
			let max = u32::max(dimentions.0, dimentions.1) as f32;
			let scale_x = dimentions.1 as f32 / max;
			let scale_y = dimentions.0 as f32 / max;
	
			obj.draw(
				&mut target,
				&program,
				&uniform! {
					matrix: create_matrix(rot, scale),
					scaling: [scale_x, scale_y],
					light: light,
					tex: &tex.texture
				}
			);
			target.finish().unwrap();
		}
	);
}
